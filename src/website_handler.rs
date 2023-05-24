use crate::{
    http::{Request, Response, StatusCode},
    server::Handler,
};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, _request: &Request) -> Response {
        Response::new(
            StatusCode::Ok,
            Some("<h1>Data sent by WebsiteHandler</h1>".to_string()),
        )
    }
}
