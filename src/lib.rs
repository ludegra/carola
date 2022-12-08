pub mod handler;
pub mod http;

#[cfg(test)]
mod tests {
    use crate::http;

    #[test]
    fn parse_http_get() {
        let raw = vec![
            "GET / HTTP/1.1",
            "Host: localhost:8000",
            "Accept: application/json",
        ];

        http::HTTPRequest::parse(raw).expect("Failed to parse HTTP request");
    }

    #[test]
    fn parse_http_post() {
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
}
