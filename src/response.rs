use crate::Error;
use serde::Serialize;
use wasm_bindgen::JsValue;

/// Worker response for HTTP requests.
/// The Response is created/accessed from `ctx.response()` and has a builder-like api.
#[derive(Debug)]
pub struct Response {
    status: u16,
    headers: Option<web_sys::Headers>,
    body: Vec<u8>,
    unset: bool,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            headers: None,
            body: Vec::new(),
            unset: true,
        }
    }
}

impl Response {
    /// Sets response status
    pub fn status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self.unset = false;
        self
    }

    /// Sets response body to the binary data
    pub fn body(&mut self, bytes: &[u8]) -> &mut Self {
        self.body = Vec::from(bytes);
        self.unset = false;
        self
    }

    /// Sets response body to value serialized as json, and sets content-type to application/json
    pub fn json<T: Serialize>(&mut self, value: &T) -> Result<&mut Self, Error> {
        self.body = serde_json::to_vec(value)?;
        self.header("Content-Type", "application/json")?;
        self.unset = false;
        Ok(self)
    }

    /// Sets response body to the text string, encoded as utf-8
    pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        self.body = text.into().as_bytes().to_vec();
        self.unset = false;
        self
    }

    /// Sets a header for this response
    pub fn header<K: AsRef<str>, V: AsRef<str>>(
        &mut self,
        key: K,
        val: V,
    ) -> Result<&mut Self, Error> {
        if self.headers.is_none() {
            self.headers = Some(web_sys::Headers::new().unwrap());
        }
        if let Some(ref mut headers) = self.headers {
            headers.set(key.as_ref(), val.as_ref())?;
        }
        Ok(self)
    }

    /// Sets response content type
    pub fn content_type<T: AsRef<str>>(&mut self, ctype: T) -> Result<&mut Self, Error> {
        self.header(reqwest::header::CONTENT_TYPE, ctype)?;
        Ok(self)
    }

    /// Returns the status of this response
    pub fn get_status(&self) -> u16 {
        self.status
    }

    /// Returns body of this response
    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Returns headers for this response, or None if no headers have been set
    pub fn get_headers(&self) -> Option<&web_sys::Headers> {
        self.headers.as_ref()
    }

    /// Converts Response to JsValue
    /// This is destructive to self (removes headers) and is used after
    /// application request handling has completed.
    pub(crate) fn into_js(mut self) -> JsValue {
        let map = js_sys::Map::new();
        map.set(
            &JsValue::from_str("status"),
            &JsValue::from_f64(self.status as f64),
        );
        map.set(
            &JsValue::from_str("body"),
            &js_sys::Uint8Array::from(self.body.as_ref()),
        );
        if self.headers.is_some() {
            let headers = std::mem::take(&mut self.headers).unwrap();
            map.set(&JsValue::from_str("headers"), &JsValue::from(headers));
        } else {
            map.set(
                &JsValue::from_str("headers"),
                &JsValue::from(web_sys::Headers::new().unwrap()),
            );
        }
        JsValue::from(map)
    }

    /// True if the response has not been filled in (none of status(), text() or body() has been
    /// called).
    /// This could be used as a flag for chained handlers to determine whether a previous
    /// handler has filled in the response yet.
    /// Setting headers (including content_type or user_agent) does not mark this assigned.
    pub fn is_unset(&self) -> bool {
        self.unset
    }
}
