use std::fmt;
use wasm_bindgen::JsValue;

/// HTTP Method
#[derive(Clone, Copy, PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
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
    /// HTTP OPTIONS method
    OPTIONS,
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
            "OPTIONS" | "options" => Method::OPTIONS,
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
                Method::OPTIONS => "OPTIONS",
            }
        )
    }
}

#[test]
fn method_str() {
    // to_string() and from()
    assert_eq!(Method::from("GET").unwrap().to_string(), "GET");
    assert_eq!(Method::from("POST").unwrap().to_string(), "POST");
    assert_eq!(Method::from("PUT").unwrap().to_string(), "PUT");
    assert_eq!(Method::from("DELETE").unwrap().to_string(), "DELETE");
    assert_eq!(Method::from("HEAD").unwrap().to_string(), "HEAD");
    assert_eq!(Method::from("OPTIONS").unwrap().to_string(), "OPTIONS");

    // PartialEq
    assert!(Method::from("GET").unwrap() == Method::GET);

    // Debug
    assert_eq!(format!("{:?}", Method::PUT), "PUT");

    // parse error
    // moved this to tests/method.rs because it depends on web_sys::JsValue
    //assert!(Method::from("none").is_err());
}
