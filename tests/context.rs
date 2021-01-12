wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;
use wasm_service::Context;

#[wasm_bindgen_test]
fn response_defaults() {
    let mut ctx = crate::Context::default();
    assert_eq!(ctx.response().get_status(), 200);
    assert_eq!(ctx.response().get_body().len(), 0);
}

#[wasm_bindgen_test]
fn response_text() {
    let mut ctx = Context::default();
    ctx.response().status(201).text("hello");

    assert_eq!(ctx.response().get_status(), 201);
    assert_eq!(&ctx.response().get_body(), &b"hello");
}

#[wasm_bindgen_test]
fn response_bin() {
    let mut ctx = Context::default();
    ctx.response()
        .status(202)
        .body(b"bytes".to_owned().to_vec());

    assert_eq!(ctx.response().get_status(), 202);
    assert_eq!(&ctx.response().get_body(), &b"bytes");
}

#[wasm_bindgen_test]
fn response_headers() {
    let mut ctx = Context::default();
    ctx.response()
        .header("Content-Type", "application/json")
        .expect("set-header");

    let sv = ctx
        .response()
        .get_headers()
        .unwrap()
        .get("Content-Type")
        .expect("get header");
    assert!(sv.is_some(), "is-defined content-type");
    assert_eq!(sv.unwrap(), "application/json", "content-type value");
}
