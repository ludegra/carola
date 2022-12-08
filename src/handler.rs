use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener, collections::HashMap,
};

use crate::http;

pub struct RequestHandler {}

impl RequestHandler {
    pub fn new() -> RequestHandler {
        RequestHandler {}
    }

    pub fn set_listener<C: FnMut()>(&mut self, method: &str, path: &str, callback: C) {
        todo!()
    }

    pub fn listen(&self, port: usize) {
        // TODO: Error handling
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            let request = http::HTTPRequest::parse(buf_reader.lines().map(|l| l.unwrap()));

            let response = match request {
                Ok(req) => {
                    println!("{:?}", req);

                    let status_line = "HTTP/1.1 200 OK";
                    status_line.to_string()
                },
                Err(err) => {
                    let status_line = "HTTP/1.1 400 Bad Request".to_string();
                    let mut headers = HashMap::new();
                    headers.insert("Content-Type", "text/plain");
                    let len = err.len().to_string();
                    headers.insert("Content-Length", &len);
                    let headers = headers.into_iter().fold(String::new(), |acc, (k, v)| acc + &format!("{}: {}\r\n", k, v));
                    let body = err;

                    vec![
                        status_line,
                        headers,
                        body
                    ].into_iter().fold(String::new(), |acc, val| acc + &val + "\r\n")
                }
            };
            println!("{}", response);

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
