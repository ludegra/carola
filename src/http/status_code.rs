#[derive(Debug, Clone, Copy)]
pub enum HTTPStatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HTTPStatusCode {
    pub fn value(&self) -> isize {
        *self as isize
    }
    pub fn message(&self) -> String {
        String::from(*self)
    }
}

impl From<HTTPStatusCode> for String {
    fn from(source: HTTPStatusCode) -> Self {
        match source {
            HTTPStatusCode::Continue => String::from("Continue"),
            HTTPStatusCode::SwitchingProtocols => String::from("Switching Protocols"),
            HTTPStatusCode::Processing => String::from("Processing"),
            HTTPStatusCode::EarlyHints => String::from("Early Hints"),
            HTTPStatusCode::OK => String::from("OK"),
            HTTPStatusCode::Created => String::from("Created"),
            HTTPStatusCode::Accepted => String::from("Accepted"),
            HTTPStatusCode::NonAuthoritativeInformation => {
                String::from("Non-Authoritative Information")
            }
            HTTPStatusCode::NoContent => String::from("No Content"),
            HTTPStatusCode::ResetContent => String::from("Reset Content"),
            HTTPStatusCode::PartialContent => String::from("Partial Content"),
            HTTPStatusCode::MultiStatus => String::from("Multi-Status"),
            HTTPStatusCode::AlreadyReported => String::from("Already Reported"),
            HTTPStatusCode::IMUsed => String::from("IM Used"),
            HTTPStatusCode::MultipleChoices => String::from("Multiple Choices"),
            HTTPStatusCode::MovedPermanently => String::from("Moved Permanently"),
            HTTPStatusCode::Found => String::from("Found"),
            HTTPStatusCode::SeeOther => String::from("See Other"),
            HTTPStatusCode::NotModified => String::from("Not Modified"),
            HTTPStatusCode::UseProxy => String::from("Use Proxy"),
            HTTPStatusCode::TemporaryRedirect => String::from("Temporary Redirect"),
            HTTPStatusCode::PermanentRedirect => String::from("Permanent Redirect"),
            HTTPStatusCode::BadRequest => String::from("Bad Request"),
            HTTPStatusCode::Unauthorized => String::from("Unauthorized"),
            HTTPStatusCode::PaymentRequired => String::from("Payment Required"),
            HTTPStatusCode::Forbidden => String::from("Forbidden"),
            HTTPStatusCode::NotFound => String::from("Not Found"),
            HTTPStatusCode::MethodNotAllowed => String::from("Method Not Allowed"),
            HTTPStatusCode::NotAcceptable => String::from("Not Acceptable"),
            HTTPStatusCode::ProxyAuthenticationRequired => {
                String::from("Proxy Authentication Required")
            }
            HTTPStatusCode::RequestTimeout => String::from("Request Timeout"),
            HTTPStatusCode::Conflict => String::from("Conflict"),
            HTTPStatusCode::Gone => String::from("Gone"),
            HTTPStatusCode::LengthRequired => String::from("Length Required"),
            HTTPStatusCode::PreconditionFailed => String::from("Precondition Failed"),
            HTTPStatusCode::PayloadTooLarge => String::from("Payload Too Large"),
            HTTPStatusCode::URITooLong => String::from("URI Too Long"),
            HTTPStatusCode::UnsupportedMediaType => String::from("Unsupported Media Type"),
            HTTPStatusCode::RangeNotSatisfiable => String::from("Range Not Satisfiable"),
            HTTPStatusCode::ExpectationFailed => String::from("Expectation Failed"),
            HTTPStatusCode::ImATeapot => String::from("I'm a teapot"),
            HTTPStatusCode::MisdirectedRequest => String::from("Misdirected Request"),
            HTTPStatusCode::UnprocessableEntity => String::from("Unprocessable Entity"),
            HTTPStatusCode::Locked => String::from("Locked"),
            HTTPStatusCode::FailedDependency => String::from("Failed Dependency"),
            HTTPStatusCode::TooEarly => String::from("Too Early"),
            HTTPStatusCode::UpgradeRequired => String::from("Upgrade Required"),
            HTTPStatusCode::PreconditionRequired => String::from("Precondition Required"),
            HTTPStatusCode::TooManyRequests => String::from("Too Many Requests"),
            HTTPStatusCode::RequestHeaderFieldsTooLarge => {
                String::from("Request Header Fields Too Large")
            }
            HTTPStatusCode::UnavailableForLegalReasons => {
                String::from("Unavailable For Legal Reasons")
            }
            HTTPStatusCode::InternalServerError => String::from("Internal Server Error"),
            HTTPStatusCode::NotImplemented => String::from("Not Implemented"),
            HTTPStatusCode::BadGateway => String::from("Bad Gateway"),
            HTTPStatusCode::ServiceUnavailable => String::from("Service Unavailable"),
            HTTPStatusCode::GatewayTimeout => String::from("Gateway Timeout"),
            HTTPStatusCode::HTTPVersionNotSupported => String::from("HTTP Version Not Supported"),
            HTTPStatusCode::VariantAlsoNegotiates => String::from("Variant Also Negotiates"),
            HTTPStatusCode::InsufficientStorage => String::from("Insufficient Storage"),
            HTTPStatusCode::LoopDetected => String::from("Loop Detected"),
            HTTPStatusCode::NotExtended => String::from("Not Extended"),
            HTTPStatusCode::NetworkAuthenticationRequired => {
                String::from("Network Authentication Required")
            }
        }
    }
}
