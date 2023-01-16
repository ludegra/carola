use std::collections::HashMap;

use carola::http::{HTTPResponse, HTTPStatusCode};

#[test]
fn construct_response() {
    let headers = HashMap::new();
    let body = "<html><body><h1>Hello, World!</h1></body></html>".to_string();

    let response = HTTPResponse::new("1.1", HTTPStatusCode::OK, headers, Some(body.clone()));
    let constructed = response.construct();

    assert_eq!(
        constructed,
        format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Length:{}\r\n\
            \r\n\
            <html><body><h1>Hello, World!</h1></body></html>\r\n\
            \r\n",
            body.len()
        )
    )
}
