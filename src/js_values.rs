/// utilities for converting to/from JsValue
/// Specifically, for use in getting params from incoming Request
use wasm_bindgen::JsValue;

/// Retrieve a string value from map
pub(crate) fn get_map_str(map: &js_sys::Map, key: &str) -> Option<String> {
    let val = map.get(&JsValue::from_str(key));
    if !val.is_undefined() {
        val.as_string()
    } else {
        None
    }
}

/// Retrieve Vec<u8> from map
pub(crate) fn get_map_bytes(map: &js_sys::Map, key: &str) -> Option<Vec<u8>> {
    let val = map.get(&JsValue::from_str(key));
    if !val.is_undefined() {
        let arr = js_sys::Uint8Array::from(val);
        Some(arr.to_vec())
    } else {
        None
    }
}

/// Retrieve headers from map
pub(crate) fn get_map_headers(map: &js_sys::Map, key: &str) -> Option<web_sys::Headers> {
    let val = map.get(&JsValue::from_str(key));
    if !val.is_undefined() {
        Some(web_sys::Headers::from(val))
    } else {
        None
    }
}
