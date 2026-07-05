pub(crate) mod env;
pub(crate) mod exec;
pub(crate) mod fs;
pub(crate) mod request;

pub(crate) trait Platform {
    fn exec(&self) -> &impl exec::Exec;
    fn env(&self) -> &impl env::Env;
    fn fs(&self) -> &impl fs::Fs;
    fn request(&self) -> &impl request::Request;
}

pub(crate) fn platform() -> impl Platform {
    PlatformImpl::new()
}

#[derive(Default)]
pub(crate) struct PlatformImpl;

impl PlatformImpl {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Platform for PlatformImpl {
    fn exec(&self) -> &impl exec::Exec {
        &exec::ExecImpl
    }

    fn env(&self) -> &impl env::Env {
        &env::EnvImpl
    }

    fn fs(&self) -> &impl fs::Fs {
        &fs::FsImpl
    }

    fn request(&self) -> &impl request::Request {
        &request::RequestImpl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::env::tests::TestEnv;
    use crate::platform::exec::tests::TestExec;
    use crate::platform::fs::tests::TestFs;
    use crate::platform::request::tests::TestRequest;

    #[derive(Default)]
    pub(crate) struct TestPlatform {
        pub(crate) env: TestEnv,
        pub(crate) exec: TestExec,
        pub(crate) fs: TestFs,
        pub(crate) request: TestRequest,
    }

    pub(crate) fn test_platform() -> impl Platform {
        TestPlatform::new()
    }

    impl TestPlatform {
        pub(crate) fn new() -> Self {
            Self::default()
        }
    }

    impl Platform for TestPlatform {
        fn exec(&self) -> &impl exec::Exec {
            &self.exec
        }

        fn env(&self) -> &impl env::Env {
            &self.env
        }

        fn fs(&self) -> &impl fs::Fs {
            &self.fs
        }

        fn request(&self) -> &impl request::Request {
            &self.request
        }
    }
}
