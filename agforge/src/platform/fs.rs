use crate::error::ErrorKind;
use crate::error::ForgeError;
use crate::result::ForgeResult;

pub(crate) trait Fs {
    fn exists<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<bool>;
    fn read_to_string<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<String>;
}

pub(crate) struct FsImpl;

impl Fs for FsImpl {
    fn exists<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<bool> {
        std::fs::exists(path).map_err(|e| ForgeError::from_error(ErrorKind::Fs, e))
    }

    fn read_to_string<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<String> {
        std::fs::read_to_string(path).map_err(|e| ForgeError::from_error(ErrorKind::Fs, e))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    pub(crate) struct TestFs {
        files: HashMap<String, String>,
    }

    impl TestFs {
        pub(crate) fn new() -> Self {
            Self::default()
        }

        pub(crate) fn insert<P: Into<String>, C: Into<String>>(&mut self, path: P, content: C) {
            self.files.insert(path.into(), content.into());
        }
    }

    impl Fs for TestFs {
        fn exists<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<bool> {
            Ok(self
                .files
                .contains_key(path.as_ref().to_str().unwrap_or_default()))
        }

        fn read_to_string<P: AsRef<std::path::Path>>(&self, path: P) -> ForgeResult<String> {
            self.files
                .get(path.as_ref().to_str().unwrap_or_default())
                .cloned()
                .ok_or_else(|| {
                    ForgeError::from_str(
                        ErrorKind::Fs,
                        format!(
                            "File not found: {}",
                            path.as_ref().to_str().unwrap_or_default()
                        ),
                    )
                })
        }
    }
}
