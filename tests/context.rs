wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use wasm_bindgen_test::*;
use wasm_service::Context;

#[wasm_bindgen_test]
fn response_defaults() {
    let mut ctx = crate::Context::default();
    assert_eq!(ctx.response().get_status(), 200);
    assert_eq!(ctx.response().get_body().len(), 0);
    assert_eq!(ctx.response().is_empty(), true);
}

#[wasm_bindgen_test]
fn response_unset_status() {
    let mut ctx = crate::Context::default();
    assert_eq!(ctx.response().is_unset(), true);

    ctx.response().status(200); // any status change, even 200 should make unset false
    assert_eq!(ctx.response().is_unset(), false);
}

#[wasm_bindgen_test]
fn response_unset_body() {
    let mut ctx = crate::Context::default();
    assert_eq!(ctx.response().is_unset(), true);

    ctx.response().body(""); // any body change, even empty, should make unset false
    assert_eq!(ctx.response().is_unset(), false);
}

#[wasm_bindgen_test]
fn response_body_empty() {
    let mut ctx = Context::default();

    assert_eq!(ctx.response().is_empty(), true);
    ctx.response().body("x");
    assert_eq!(ctx.response().is_empty(), false);
}

#[wasm_bindgen_test]
fn response_body_into() {
    let mut ctx = Context::default();

    // from &'static str
    ctx.response().body("hello");
    assert_eq!(ctx.response().get_body(), b"hello");

    // from Vec<u8>
    let v: Vec<u8> = vec![1, 1, 2, 3, 5, 8];
    ctx.response().body(v.clone());
    assert_eq!(ctx.response().get_body(), &v);

    // from Bytes
    let buf = bytes::Bytes::from_static(b"xyz");
    ctx.response().body(buf);
    assert_eq!(ctx.response().get_body(), b"xyz");

    // from &'static [u8]
    let static_buf: &'static [u8] = b"alice";
    ctx.response().body(static_buf);
    assert_eq!(ctx.response().get_body(), static_buf);
}

#[wasm_bindgen_test]
fn response_text() {
    let mut ctx = Context::default();
    ctx.response().status(201).text("hello");

    assert_eq!(ctx.response().get_status(), 201);
    assert_eq!(&ctx.response().get_body(), b"hello");
    assert_eq!(ctx.response().is_empty(), false);
}

#[wasm_bindgen_test]
fn response_bin() {
    let mut ctx = Context::default();
    ctx.response().status(202).body("bytes");

    assert_eq!(ctx.response().get_status(), 202);
    assert_eq!(&ctx.response().get_body(), b"bytes");
    assert_eq!(ctx.response().is_empty(), false);
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
