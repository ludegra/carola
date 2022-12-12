#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HTTPMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Other(String),
}

impl From<&str> for HTTPMethod {
    fn from(source: &str) -> Self {
        match source {
            "GET" => HTTPMethod::Get,
            "HEAD" => HTTPMethod::Head,
            "POST" => HTTPMethod::Post,
            "PUT" => HTTPMethod::Put,
            "DELETE" => HTTPMethod::Delete,
            "CONNECT" => HTTPMethod::Connect,
            "OPTIONS" => HTTPMethod::Options,
            _ => HTTPMethod::Other(source.to_string()),
        }
    }
}