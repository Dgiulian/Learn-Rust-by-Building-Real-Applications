use crate::{http::Response, server::Handler};

pub struct WebsiteHandler {}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &crate::http::Request) -> crate::http::Response {
        dbg!(request);
        Response::new(
            crate::http::StatusCode::Ok,
            Some("<h1>Hello world!</h1>".to_string()),
        )
    }
}
