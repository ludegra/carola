use std::collections::HashMap;

use carola::http::{self, HTTPMethod};

#[test]
fn request_without_content() {
    let raw = vec![
        "GET / HTTP/1.1",
        "Host: localhost:8000",
        "Accept: application/json",
    ];

    http::HTTPRequest::parse(raw).expect("Failed to parse HTTP request");
}

#[test]
fn request_with_content() {
    let raw = vec![
        "POST /echo/post/json HTTP/1.1",
        "Host: reqbin.com",
        "Accept: application/json",
        "Content-Type: application/json",
        "Content-Length: 81",
        "",
        "{",
        "\"Id\": 78912,",
        "\"Customer\": \"Jason Sweet\",",
        "\"Quantity\": 1,",
        "\"Price\": 18.00",
        "}",
    ];

    http::HTTPRequest::parse(raw).expect("Failed to parse");
}

#[test]
fn http_metadata() {
    let raw = vec![
        "GET / HTTP/1.1",
        "Host: localhost:8000",
        "Accept: application/json",
    ];

    let parsed = http::HTTPRequest::parse(raw).expect("Failed to parse");
    assert_eq!(*parsed.get_method(), HTTPMethod::Get);
    assert_eq!(parsed.get_uri(), "/");
    assert_eq!(parsed.get_version(), "1.1");
}

#[test]
fn headers() {
    let raw = vec![
        "GET / HTTP/1.1",
        "Host: localhost:8000",
        "Accept: application/json",
    ];

    let parsed = http::HTTPRequest::parse(raw).expect("Failed to parse");
    let mut expected = HashMap::new();
    expected.insert("Host".to_string(), "localhost:8000".to_string());
    expected.insert("Accept".to_string(), "application/json".to_string());

    assert_eq!(*parsed.get_headers(), expected);
}
