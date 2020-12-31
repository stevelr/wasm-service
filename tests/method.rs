wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;
use wasm_service::Method;

// other method tests are in src/method.rs
// This has to be here because it depends on web-sys
#[wasm_bindgen_test]
fn method_parse() {
    assert!(Method::from("HEAD").is_ok());
    assert!(Method::from("none").is_err());
}
