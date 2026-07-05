use crate::error::ErrorKind;
use crate::error::ForgeError;
use crate::result::ForgeResult;
use crate::utils;

#[derive(Default)]
pub(crate) struct EnvValue<'a> {
    pub(crate) key: &'a str,
    pub(crate) value: String,
}

pub(crate) trait Env {
    fn value<'a, T: AsRef<str> + ?Sized>(&self, key: &'a T) -> ForgeResult<EnvValue<'a>>;
}

pub(crate) struct EnvImpl;

impl Env for EnvImpl {
    fn value<'a, T: AsRef<str> + ?Sized>(&self, key: &'a T) -> ForgeResult<EnvValue<'a>> {
        std::env::var(key.as_ref())
            .map(|value| EnvValue {
                key: key.as_ref(),
                value,
            })
            .map_err(|e| ForgeError::from_error(ErrorKind::Env, e))
    }
}

impl EnvValue<'_> {
    pub(crate) fn as_bool(&self) -> ForgeResult<bool> {
        match utils::unquote(&self.value).to_lowercase().as_str() {
            "true" | "on" | "1" => Ok(true),
            "" | "false" | "off" | "0" => Ok(false),
            _ => Err(ForgeError::from_str(
                ErrorKind::Env,
                format!(
                    "[Env] Invalid boolean value for key '{}': '{}' (expected: true, on, 1 / false, off, 0, <empty>)",
                    self.key, self.value
                ),
            )),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::error::ForgeError;
    use crate::result::ForgeResult;
    use std::collections::HashMap;

    #[derive(Default)]
    pub(crate) struct TestEnv {
        values: HashMap<String, String>,
    }

    impl TestEnv {
        pub(crate) fn new() -> Self {
            Self::default()
        }

        pub(crate) fn insert<K: Into<String>, V: Into<String>>(&mut self, key: K, value: V) {
            self.values.insert(key.into(), value.into());
        }
    }

    impl Env for TestEnv {
        fn value<'a, T: AsRef<str> + ?Sized>(&self, key: &'a T) -> ForgeResult<EnvValue<'a>> {
            self.values
                .get(key.as_ref())
                .cloned()
                .map(|value| EnvValue {
                    key: key.as_ref(),
                    value,
                })
                .ok_or_else(|| {
                    ForgeError::from_error(ErrorKind::Env, std::env::VarError::NotPresent)
                })
        }
    }

    #[test]
    fn test_env_values() {
        let mut env = TestEnv::default();
        env.insert("TEST_KEY", "test_value");
        env.insert("QUOTED_VALUE", "test_value");

        for (key, expected_value) in &env.values {
            let value = env.value(&key).unwrap();
            assert_eq!(value.key, key);
            assert_eq!(value.value, *expected_value);
        }
    }

    #[test]
    fn test_env_value_accepts_string_like_keys() {
        let mut env = TestEnv::default();
        env.insert("STRING_KEY", "from_string");
        env.insert("STATIC_KEY", "from_static");

        let owned = String::from("STRING_KEY");
        let static_key: &'static str = "STATIC_KEY";

        assert!(matches!(env.value(&owned), Ok(value) if value.value == "from_string"));
        assert!(matches!(env.value(&static_key), Ok(value) if value.value == "from_static"));
    }

    #[test]
    fn test_env_missing_value() {
        let env = TestEnv::default();
        let _err: ForgeResult<EnvValue> = Err(ForgeError::from_error(
            ErrorKind::Env,
            std::env::VarError::NotPresent,
        ));
        assert!(matches!(env.value("MISSING_KEY"), _err));
    }

    #[test]
    fn test_env_as_bool_true() {
        let mut env = TestEnv::default();
        env.insert("BOOL_TRUE", "true");
        env.insert("BOOL_TRUE_QUOTED", "\'true\'");
        env.insert("BOOL_TRUE_DOUBLE_QUOTED", "\"true\"");
        env.insert("BOOL_TRUE_UPPER", "TRUE");
        env.insert("BOOL_TRUE_MIXED_CASE", "True");
        env.insert("BOOL_ON", "on");
        env.insert("BOOL_ON_QUOTED", "\'on\'");
        env.insert("BOOL_ON_DOUBLE_QUOTED", "\"on\"");
        env.insert("BOOL_ON_UPPER", "ON");
        env.insert("BOOL_ON_MIXED_CASE", "On");
        env.insert("BOOL_ONE", "1");
        env.insert("BOOL_ONE_QUOTED", "\'1\'");
        env.insert("BOOL_ONE_DOUBLE_QUOTED", "\"1\"");

        for key in env.values.keys() {
            assert!(
                env.value(key).unwrap().as_bool().unwrap(),
                "Expected {key} to be true",
            );
        }
    }

    #[test]
    fn test_env_as_bool_false() {
        let mut env = TestEnv::default();
        env.insert("BOOL_FALSE", "false");
        env.insert("BOOL_FALSE_QUOTED", "\'false\'");
        env.insert("BOOL_FALSE_DOUBLE_QUOTED", "\"false\"");
        env.insert("BOOL_FALSE_UPPER", "FALSE");
        env.insert("BOOL_FALSE_MIXED", "FalsE");
        env.insert("BOOL_OFF", "off");
        env.insert("BOOL_OFF_QUOTED", "\'off\'");
        env.insert("BOOL_OFF_DOUBLE_QUOTED", "\"off\"");
        env.insert("BOOL_OFF_UPPER", "OFF");
        env.insert("BOOL_OFF_MIXED", "Off");
        env.insert("BOOL_ZERO", "0");
        env.insert("BOOL_ZERO_QUOTED", "\'0\'");
        env.insert("BOOL_ZERO_DOUBLE_QUOTED", "\"0\"");
        env.insert("BOOL_EMPTY", "");
        env.insert("BOOL_EMPTY_QUOTED", "\'\'");
        env.insert("BOOL_EMPTY_DOUBLE_QUOTED", "\"\"");

        for key in env.values.keys() {
            assert!(
                !env.value(key).unwrap().as_bool().unwrap(),
                "Expected {key} to be false",
            );
        }
    }

    #[test]
    fn test_env_as_bool_unwrap_or_default_is_false() {
        let env = TestEnv::default();

        assert!(
            !env.value("MISSING_KEY")
                .unwrap_or_default()
                .as_bool()
                .unwrap()
        );
    }

    #[test]
    fn test_env_as_bool_invalid() {
        let mut env = TestEnv::default();
        env.insert("BOOL_INVALID_STRING", "invalid");
        env.insert("BOOL_INVALID_NUMBER", "2");
        env.insert("BOOL_INVALID_DOUBLY_QUOTED_SINGLE", "''true''");
        env.insert("BOOL_INVALID_DOUBLY_QUOTED_MIXED", "\"'false'\"");

        for key in env.values.keys() {
            let err = env.value(key).unwrap().as_bool().unwrap_err();
            assert_eq!(
                format!("{err}"),
                format!(
                    "[Env] Invalid boolean value for key '{}': '{}' (expected: true, on, 1 / false, off, 0, <empty>)",
                    key,
                    env.value(key).unwrap().value
                )
            );
        }
    }
}
