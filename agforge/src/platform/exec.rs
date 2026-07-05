use crate::error::ErrorKind;
use crate::error::ForgeError;
use crate::result::ForgeResult;
use std::process::Command;

#[derive(Clone, Debug)]
pub(crate) struct CommandOutput {
    pub(crate) status: i32,
    pub(crate) stdout: String,
    pub(crate) stderr: String,
}

pub(crate) trait Exec {
    fn run(&self, command: Command) -> ForgeResult<CommandOutput>;
}

#[derive(Default)]
pub(crate) struct ExecImpl;

impl ExecImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Exec for ExecImpl {
    fn run(&self, mut command: Command) -> ForgeResult<CommandOutput> {
        let output = command
            .output()
            .map_err(|e| ForgeError::from_error(ErrorKind::Exec, e))?;

        let status = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(CommandOutput {
            status,
            stdout,
            stderr,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[derive(Default)]
    pub(crate) struct TestExec {
        commands: Vec<(String, CommandOutput)>,
    }

    impl TestExec {
        pub(crate) fn new() -> Self {
            Self::default()
        }

        pub(crate) fn set<P: Into<String>, O: Into<CommandOutput>>(
            &mut self,
            command_pattern: P,
            output: O,
        ) {
            self.commands.push((command_pattern.into(), output.into()));
        }
    }

    impl Exec for TestExec {
        fn run(&self, command: Command) -> ForgeResult<CommandOutput> {
            let command_str = format!("{:?}", command);
            self.commands
                .iter()
                .find(|(pattern, _)| command_str.starts_with(pattern))
                .map(|(_, output)| output.clone())
                .ok_or_else(|| {
                    ForgeError::from_str(
                        ErrorKind::Exec,
                        format!("No output set for command: {}", command_str),
                    )
                })
        }
    }

    impl From<i32> for CommandOutput {
        fn from(status: i32) -> Self {
            CommandOutput {
                status,
                stdout: String::new(),
                stderr: String::new(),
            }
        }
    }

    impl<O: Into<String>> From<(i32, O)> for CommandOutput {
        fn from((status, stdout): (i32, O)) -> Self {
            CommandOutput {
                status,
                stdout: stdout.into(),
                stderr: String::new(),
            }
        }
    }

    impl<O: Into<String>, E: Into<String>> From<(i32, O, E)> for CommandOutput {
        fn from((status, stdout, stderr): (i32, O, E)) -> Self {
            CommandOutput {
                status,
                stdout: stdout.into(),
                stderr: stderr.into(),
            }
        }
    }
}
