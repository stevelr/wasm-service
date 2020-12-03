use crate::{js_values, Error, Method};
use serde::de::DeserializeOwned;
use url::Url;
use wasm_bindgen::JsValue;

/// Incoming HTTP request (to Worker).
pub struct Request {
    method: Method,
    url: Url,
    headers: web_sys::Headers,
    body: Option<Vec<u8>>,
}

impl Request {
    /// Creates Request object representing incoming HTTP request
    /// Used internally and for testing
    pub(crate) fn new(
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
        match self.headers.has(name) {
            Ok(b) => b,
            Err(_) => false,
        }
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
}
