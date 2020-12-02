use std::fmt;
use wasm_bindgen::JsValue;

/// HTTP Method
#[derive(Clone, Copy, Debug)]
pub enum Method {
    /// HTTP GET method
    GET,
    /// HTTP POST method
    POST,
    /// HTTP PUT method
    PUT,
    /// HTTP DELETE method
    DELETE,
    /// HTTP HEAD method
    HEAD,
}

impl Method {
    /// Converts string to Method
    pub fn from(s: &str) -> Result<Method, JsValue> {
        Ok(match s {
            "GET" | "get" => Method::GET,
            "POST" | "post" => Method::POST,
            "PUT" | "put" => Method::PUT,
            "DELETE" | "delete" => Method::DELETE,
            "HEAD" | "head" => Method::HEAD,
            _ => return Err(JsValue::from_str("Unsupported http method")),
        })
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Method::GET => "GET",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::DELETE => "DELETE",
                Method::HEAD => "HEAD",
            }
        )
    }
}
