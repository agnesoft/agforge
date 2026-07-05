#[derive(Debug)]
pub(crate) enum ErrorKind {
    Env,
    Exec,
    Fs,
    Request,
}

#[derive(Debug)]
pub(crate) struct ForgeError {
    kind: ErrorKind,
    description: String,
}

impl ForgeError {
    pub(crate) fn from_str(kind: ErrorKind, description: impl Into<String>) -> Self {
        Self {
            kind,
            description: description.into(),
        }
    }

    pub(crate) fn from_error(kind: ErrorKind, error: impl std::error::Error) -> Self {
        Self {
            kind,
            description: error.to_string(),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind_str = match self {
            ErrorKind::Env => "Env",
            ErrorKind::Exec => "Exec",
            ErrorKind::Fs => "Fs",
            ErrorKind::Request => "Request",
        };
        write!(f, "{kind_str}")
    }
}

impl std::fmt::Display for ForgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl std::error::Error for ForgeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ForgeError::from_str(ErrorKind::Env, "Test error");
        assert_eq!(format!("[{}] {}", error.kind, error), "Test error");
    }

    #[test]
    fn test_error_debug() {
        let error = ForgeError::from_str(ErrorKind::Exec, "Test error");
        assert_eq!(
            format!("{error:?}"),
            "ForgeError { kind: Exec, description: \"Test error\" }"
        );
    }
}
