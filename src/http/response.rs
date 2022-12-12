use std::collections::HashMap;

use super::HTTPStatusCode;

pub struct HTTPResponse {
    version: String,
    status: HTTPStatusCode,
    headers: HashMap<String, String>,
    body: Option<String>
}

impl HTTPResponse {
    pub fn new(version: &str, status: HTTPStatusCode, headers: HashMap<String, String>, body: Option<String>) -> HTTPResponse {
        Self {
            version: version.to_owned(),
            status,
            headers,
            body
        }
    }

    pub fn construct(self) -> String {
        let status_line = format!("HTTP/{} {} {}", self.version, self.status.value().to_string(), self.status.message());
        let headers = self.headers.into_iter().map(|(key, value)| format!("{}:{}", key, value)).collect::<Vec<_>>();
        
        let rows: Vec<String>;

        if let Some(body) = self.body {
            rows = [vec![status_line], headers, vec![String::from("")], vec![body]].into_iter().flatten().collect();
        }
        else {
            rows = [vec![status_line], headers].into_iter().flatten().collect();
        }

        rows.join("\r\n") + "\r\n"
    }
}