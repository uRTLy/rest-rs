#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum HttpStatus {
    OK = 200,
    NotFound = 404,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
        }
    }

    fn from_str(method: &str) -> Option<HttpMethod> {
        match method {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            &_ => None,
        }
    }
}
