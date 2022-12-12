use std::collections::HashMap;

use carola::http::{HTTPResponse, HTTPStatusCode};

#[test]
fn construct_response() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html".to_string());
    headers.insert("Content-Length".to_string(), "88".to_string());

    let body = "<html><body><h1>Hello, World!</h1></body></html>".to_string();

    let response = HTTPResponse::new("1.1", HTTPStatusCode::Ok, headers, Some(body));
    let constructed = response.construct();

    assert!(
        constructed
            == String::from("\
                HTTP/1.1 200 OK\r\n\
                Content-Type:text/html\r\n\
                Content-Length:88\r\n\
                \r\n\
                <html><body><h1>Hello, World!</h1></body></html>\r\n\
            ") || constructed
            == String::from("\
                HTTP/1.1 200 OK\r\n\
                Content-Length:88\r\n\
                Content-Type:text/html\r\n\
                \r\n\
                <html><body><h1>Hello, World!</h1></body></html>\r\n\
            ")
    )
}
