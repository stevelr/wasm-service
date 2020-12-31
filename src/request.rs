use crate::js_values;
use crate::{Error, Method};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use url::Url;
use wasm_bindgen::JsValue;

/// Incoming HTTP request (to Worker).
#[derive(Debug, Clone)]
pub struct Request {
    method: Method,
    url: Url,
    headers: web_sys::Headers,
    body: Option<Vec<u8>>,
}
unsafe impl Sync for Request {}

impl Request {
    /// Creates Request object representing incoming HTTP request
    pub fn new(
        method: Method,
        url: Url,
        headers: web_sys::Headers,
        body: Option<Vec<u8>>,
    ) -> Request {
        Request {
            method,
            url,
            headers,
            body,
        }
    }

    /// Creates Request from javascript object
    pub(crate) fn from_js(map: &js_sys::Map) -> Result<Self, JsValue> {
        Ok(Request::new(
            Method::from(
                &js_values::get_map_str(&map, "method")
                    .ok_or_else(|| JsValue::from_str("invalid_req.method"))?,
            )?,
            Url::parse(
                &js_values::get_map_str(&map, "url")
                    .ok_or_else(|| JsValue::from_str("invalid_req.url"))?,
            )
            .map_err(|e| JsValue::from_str(&format!("invalid req.url:{}", e.to_string())))?,
            js_values::get_map_headers(&map, "headers")
                .ok_or_else(|| JsValue::from_str("invalid_req"))?,
            js_values::get_map_bytes(&map, "body"),
        ))
    }

    /// Returns the HTTP method
    pub fn method(&self) -> Method {
        self.method
    }

    /// Returns the parsed url
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the set of request headers
    pub fn headers(&self) -> &web_sys::Headers {
        &self.headers
    }

    /// Returns the value of the header, or None if the header is not set.
    /// Header name search is case-insensitive.
    pub fn get_header(&self, name: &str) -> Option<String> {
        match self.headers.get(name) {
            Ok(v) => v,
            Err(_) => None,
        }
    }

    /// Returns true if the header is set. Name is case-insensitive.
    pub fn has_header(&self, name: &str) -> bool {
        self.headers.has(name).unwrap_or(false)
    }

    /// Returns request body as byte vector, or None if body is empty
    pub fn body(&self) -> Option<&Vec<u8>> {
        self.body.as_ref()
    }

    /// Interpret body as json object.
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, Error> {
        if let Some(vec) = self.body.as_ref() {
            Ok(serde_json::from_slice(vec)?)
        } else {
            Err(Error::Other("body is empty".to_string()))
        }
    }

    /// Returns the cookie string, if set
    pub fn get_cookie_value(&self, cookie_name: &str) -> Option<String> {
        self.get_header("cookie")
            .map(|cookie| {
                (&cookie)
                    .split(';')
                    // allow spaces around ';'
                    .map(|s| s.trim())
                    // if name=value, return value
                    .find_map(|part| cookie_value(part, cookie_name))
                    .map(|v| v.to_string())
            })
            .unwrap_or_default()
    }

    /// returns the query variable from the url, or None if not found
    pub fn get_query_value<'req>(&'req self, key: &'_ str) -> Option<Cow<'req, str>> {
        self.url()
            .query_pairs()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
}

// If 'part' is of the form 'name=value', return value
fn cookie_value<'cookie>(part: &'cookie str, name: &str) -> Option<&'cookie str> {
    if part.len() > name.len() {
        let (left, right) = part.split_at(name.len());
        if left == name && right.starts_with('=') {
            return Some(&right[1..]);
        }
    }
    None
}

#[test]
// test cookie_value function. Additional tests of Request are in tests/request.rs
fn test_cookie_value() {
    // short value
    assert_eq!(cookie_value("x=y", "x"), Some("y"));

    // longer value
    assert_eq!(cookie_value("foo=bar", "foo"), Some("bar"));

    // missing value
    assert_eq!(cookie_value("x=y", "z"), None);

    // empty value
    assert_eq!(cookie_value("foo=", "foo"), Some(""));
}
