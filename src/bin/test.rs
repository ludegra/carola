use std::collections::HashMap;

use carola::{handler::RequestHandler, http::{HTTPRequest, HTTPResponse, HTTPStatusCode}};

fn main() {
    let mut handler = RequestHandler::new();
    handler.set_listener("GET", "/example", |_: HTTPRequest| {
        HTTPResponse::new("1.1", HTTPStatusCode::OK, HashMap::new(), Some(String::from("Hello World!")))
    });
    handler.listen(8080).unwrap();
}