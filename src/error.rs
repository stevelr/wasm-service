//! Errors generated by this crate
use std::fmt;
use wasm_bindgen::JsValue;

/// Errors generated by this crate
/// It's not necessary for users of wasm_service to import this,
/// because Error implements trait std::error::Error.
#[derive(Debug)]
pub enum Error {
    /// Error serializing/deserializing request, response, or log messages
    Json(serde_json::Error),

    /// Error converting parameters to/from javascript
    Js(String),

    /// Error in external http sub-request (via reqwest lib)
    Http(reqwest::Error),

    /// Error deserializing asset index
    DeserializeAssets(Box<bincode::ErrorKind>),

    /// Invalid header value (contains non-ascii characters)
    InvalidHeaderValue(String),

    /// No static asset is available at this path
    NoStaticAsset(String),

    /// KV asset not found
    #[allow(clippy::upper_case_acronyms)]
    KVKeyNotFound(String, u16),

    /// Error received from Cloudflare API while performing KV request
    #[allow(clippy::upper_case_acronyms)]
    KVApi(reqwest::Error),

    /// Catch-all
    Other(String),
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::Other(msg)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Error::Js(
            e.as_string()
                .unwrap_or_else(|| "Javascript error".to_string()),
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
