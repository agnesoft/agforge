#[derive(Clone, Debug)]
pub(crate) struct Response {
    pub(crate) status: u16,
    pub(crate) body: String,
}

pub(crate) trait Request {}

pub(crate) struct RequestImpl;

impl Request for RequestImpl {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[derive(Default)]
    pub(crate) struct TestRequest {
        requests: Vec<(String, Response)>,
    }

    impl TestRequest {
        pub(crate) fn new() -> Self {
            Self::default()
        }

        pub(crate) fn set<P: Into<String>, R: Into<Response>>(
            &mut self,
            request_pattern: P,
            response: R,
        ) {
            self.requests
                .push((request_pattern.into(), response.into()));
        }
    }

    impl From<u16> for Response {
        fn from(status: u16) -> Self {
            Response {
                status,
                body: String::new(),
            }
        }
    }

    impl<B: Into<String>> From<(u16, B)> for Response {
        fn from((status, body): (u16, B)) -> Self {
            Response {
                status,
                body: body.into(),
            }
        }
    }

    impl Request for TestRequest {}
}
