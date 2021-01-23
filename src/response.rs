use crate::Error;
use bytes::Bytes;
use serde::Serialize;
use std::fmt;
use wasm_bindgen::JsValue;

/// Worker response for HTTP requests.
/// The Response is created/accessed from `ctx.response()` and has a builder-like api.
#[derive(Debug)]
pub struct Response {
    status: u16,
    headers: Option<web_sys::Headers>,
    body: Body,
    unset: bool,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            headers: None,
            body: Body::from(Bytes::new()),
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
    pub fn body<T: Into<Body>>(&mut self, body: T) -> &mut Self {
        self.body = body.into();
        self.unset = false;
        self
    }

    /// Sets response body to value serialized as json, and sets content-type to application/json
    pub fn json<T: Serialize>(&mut self, value: &T) -> Result<&mut Self, Error> {
        use mime::APPLICATION_JSON;
        self.body = serde_json::to_vec(value)?.into();
        self.content_type(APPLICATION_JSON).unwrap();
        self.unset = false;
        Ok(self)
    }

    /// Sets response body to the text string, encoded as utf-8
    pub fn text<T: Into<String>>(&mut self, text: T) -> &mut Self {
        let str_val = text.into();
        self.body = str_val.into();
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

    /// Returns body of this response.
    pub fn get_body(&self) -> &[u8] {
        &self.body.inner.as_ref()
    }

    /// Returns headers for this response, or None if no headers have been set
    pub fn get_headers(&self) -> Option<&web_sys::Headers> {
        self.headers.as_ref()
    }

    /// Returns true if the body is empty
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
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
            &js_sys::Uint8Array::from(self.body.inner.as_ref()),
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
    /// called). (even if status() is called with 200 status or body is set to empty)
    /// This could be used as a flag for chained handlers to determine whether a previous
    /// handler has filled in the response yet.
    /// Setting headers (including content_type or user_agent) does not mark the request "set"
    /// This is so that headers can be set at the top of a handler, before errors may occur
    pub fn is_unset(&self) -> bool {
        self.unset
    }
}

/// The body of a `Response`.
// this is adapted from reqwest::wasm::Body, which is used in requests
pub struct Body {
    inner: Bytes,
}

impl Body {
    /// True if the body is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from(bytes: Bytes) -> Body {
        Body { inner: bytes }
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(vec: Vec<u8>) -> Body {
        Body { inner: vec.into() }
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(s: &'static [u8]) -> Body {
        Body {
            inner: Bytes::from_static(s),
        }
    }
}

impl From<String> for Body {
    #[inline]
    fn from(s: String) -> Body {
        Body { inner: s.into() }
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(s: &'static str) -> Body {
        s.as_bytes().into()
    }
}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Body").finish()
    }
}
