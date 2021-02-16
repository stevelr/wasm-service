/// Determines the Media Type (aka MIME) for file based on extension.
/// If type is not known (based on implemented list), returns None.
/// ```
/// # use wasm_service::media_type;
/// assert_eq!(media_type("index.html"), Some("text/html; charset=utf-8"));
/// ```
///
/// All type values are valid utf-8 strings, so it is safe to use unwrap()
/// when setting headers, e.g.
/// ```
/// # cfg_if::cfg_if!{   if #[cfg(target_arch = "wasm32")] {
/// # use wasm_service::{Response,media_type};
/// # use reqwest::header::CONTENT_TYPE;
/// # let response = Response::default();
/// if let Some(mtype) = media_type("index.html") {
///     response.header(CONTENT_TYPE, mtype).unwrap();
/// }
/// assert_eq!(55, 22);
/// # }}
/// ```
pub fn media_type(file_path: &str) -> Option<&'static str> {
    std::path::Path::new(file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(ext_to_mime)
        .unwrap_or_default()
}

/// map extension to mime type
// References
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
// https://www.iana.org/assignments/media-types/media-types.xhtml#application
fn ext_to_mime(ext: &str) -> Option<&'static str> {
    match ext {
        "html" => Some(mime::TEXT_HTML_UTF_8.as_ref()),
        "css" => Some(mime::TEXT_CSS.as_ref()),
        "js" | "ts" => Some(mime::TEXT_JAVASCRIPT.as_ref()),
        "jpg" | "jpeg" => Some(mime::IMAGE_JPEG.as_ref()),
        "png" => Some(mime::IMAGE_PNG.as_ref()),
        "gif" => Some(mime::IMAGE_GIF.as_ref()),
        "toml" => Some("application/toml"),
        "yaml" | "yml" => Some("text/x-yaml"),
        "json" => Some(mime::APPLICATION_JSON.as_ref()),
        "txt" | "py" | "rs" | "hbs" => Some(mime::TEXT_PLAIN.as_ref()),
        "md" => Some("text/markdown"),
        "wasm" => Some("application/wasm"),
        "ico" => Some("image/vnd.microsoft.icon"),
        "csv" => Some(mime::TEXT_CSV_UTF_8.as_ref()),
        "pdf" => Some(mime::APPLICATION_PDF.as_ref()),
        "bin" | "enc" | "dat" | "gz" | "tar" | "z" => Some(mime::APPLICATION_OCTET_STREAM.as_ref()),
        _ => None,
    }
}
