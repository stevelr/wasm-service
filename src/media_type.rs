/// Determines the Media Type (aka MIME) for file based on extension.
/// If type is not known (based on implemented list), returns None.
/// All type values are valid utf-8 strings, so it is safe to use unwrap()
/// if used in response.header(CONTENT_TYPE, ...)
pub fn media_type(file_path: &str) -> Option<&'static str> {
    // References
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    // https://www.iana.org/assignments/media-types/media-types.xhtml#application
    match std::path::Path::new(file_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        Some(ext) => match ext {
            "html" => Some(mime::TEXT_HTML_UTF_8.as_ref()),
            "css" => Some(mime::TEXT_CSS_UTF_8.as_ref()),
            "js" => Some(mime::TEXT_JAVASCRIPT.as_ref()),
            "csv" => Some(mime::TEXT_CSV_UTF_8.as_ref()),
            "wasm" => Some("application/wasm"),
            "md" => Some("text/markdown"),
            "toml" => Some("application/toml"),
            "yaml" => Some("text/x-yaml"),
            "jpg" | "jpeg" => Some(mime::IMAGE_JPEG.as_ref()),
            "png" => Some(mime::IMAGE_PNG.as_ref()),
            "ico" => Some("image/vnd.microsoft.icon"),
            "bin" => Some(mime::APPLICATION_OCTET_STREAM.as_ref()),
            _ => None,
        },
        None => None,
    }
}
