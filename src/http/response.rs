use std::collections::HashMap;

use super::HTTPStatusCode;

pub struct HTTPResponse {
    version: String,
    status: HTTPStatusCode,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HTTPResponse {
    pub fn new(
        version: &str,
        status: HTTPStatusCode,
        headers: HashMap<String, String>,
        body: Option<String>,
    ) -> HTTPResponse {
        Self {
            version: version.to_owned(),
            status,
            headers,
            body,
        }
    }

    /// # HTTPResponse::construct
    /// 
    /// Constructs the http response into a string
    /// 
    /// This automatically creates the `Content-Length` header and overides any already existing headers with the same key
    pub fn construct(mut self) -> String {
        let status_line = format!(
            "HTTP/{} {} {}",
            self.version,
            self.status.value().to_string(),
            self.status.message()
        );

        if let Some(body) = &self.body {
            self.headers.insert(
                String::from("Content-Length"),
                body.len().to_string(),
            );
        }
        let headers = self
            .headers
            .into_iter()
            .map(|(key, value)| format!("{}:{}", key, value))
            .collect::<Vec<_>>();

        let rows: Vec<String>;

        if let Some(body) = self.body {
            rows = [
                vec![status_line],
                headers,
                vec![String::from("")],
                vec![body],
            ]
            .into_iter()
            .flatten()
            .collect();
        } else {
            rows = [vec![status_line], headers].into_iter().flatten().collect();
        }

        rows.join("\r\n") + "\r\n\r\n"
    }
}
