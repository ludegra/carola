use std::collections::HashMap;

use super::HTTPMethod;

#[derive(Debug)]
pub struct HTTPRequest {
    method: HTTPMethod,
    version: String,
    uri: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HTTPRequest {
    pub fn parse<T: Into<String>>(raw: impl IntoIterator<Item = T>) -> Result<HTTPRequest, String> {
        let mut raw = raw.into_iter();

        // Extract the meta data in the first line
        let meta: String = match raw.next() {
            Some(meta) => meta.into(),
            None => return Err(String::from("Invalid formatting")),
        };

        let mut meta = meta.split_whitespace();

        let method = match meta.next() {
            Some(method) => method.into(),
            None => return Err(String::from("Invalid formatting")),
        };
        let uri = match meta.next() {
            Some(uri) => uri.to_string(),
            None => return Err(String::from("Invalid formatting")),
        };
        let version = match meta.next() {
            Some(version) => {
                if !version.starts_with("HTTP") {
                    return Err(String::from("Invalid request type"));
                }

                let mut split = version.split("/");
                match split.nth(1) {
                    Some(version) => version.to_string(),
                    None => return Err(String::from("Invalid formatting")),
                }
            }
            None => return Err(String::from("Invalid formatting")),
        };

        // Extract the headers
        let mut headers = HashMap::new();

        while let Some(header) = raw.next() {
            let header: String = header.into();
            if header.is_empty() {
                break;
            }

            // Split the row into key-value pairs
            let mut split = header.split(":").map(|s| s.trim());

            let key = match split.next() {
                Some(key) => key.to_string(),
                None => return Err(String::from("Invalid formatting")),
            };

            let mut value = split.fold(String::new(), |acc, v| {
                acc + v + ":"
            });
            
            if value.is_empty() {
                return Err(String::from("Invalid formatting"));
            }
            value.pop();

            headers.insert(key, value);
        }

        // If the headers contains Content-Length or Transfer-Encoding headers then get the request body
        let body = if headers.contains_key("Content-Length")
            || headers.contains_key("Transfer-Encoding")
        {
            let mut body = String::new();
            for line in raw
                .map(|l| {
                    let l: String = l.into();
                    l
                })
                .take_while(|l| !l.is_empty())
            {
                body.push_str(&format!("{}\n", line));
            }

            Some(body)
        } else {
            None
        };

        Ok(HTTPRequest {
            method,
            uri,
            version,
            headers,
            body,
        })
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_method(&self) -> &HTTPMethod {
        &self.method
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
}
