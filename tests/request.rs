wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
mod test {
    use wasm_bindgen_test::*;
    use wasm_service::{Method, Request, Url};

    #[wasm_bindgen_test]
    fn req_method() {
        let req = Request::new(
            Method::POST,
            Url::parse("https://www.example.com").unwrap(),
            web_sys::Headers::new().unwrap(),
            None,
        );

        assert_eq!(req.method(), Method::POST);

        let req = Request::new(
            Method::DELETE,
            Url::parse("https://www.example.com").unwrap(),
            web_sys::Headers::new().unwrap(),
            None,
        );
        assert_eq!(req.method(), Method::DELETE);
    }

    #[wasm_bindgen_test]
    fn req_url() {
        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            web_sys::Headers::new().unwrap(),
            None,
        );

        assert_eq!(&req.url().host().unwrap().to_string(), "www.example.com");
    }

    #[wasm_bindgen_test]
    fn req_headers() {
        let headers = web_sys::Headers::new().expect("new");
        headers.set("Content-Type", "application/json").expect("ok");
        headers.set("X-Custom-Shape", "round").expect("ok");

        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            headers,
            None,
        );

        // has_header, case-insensitive, success
        assert_eq!(req.has_header("content-type"), true);

        // has_header, non-existent
        assert_eq!(req.has_header("not-here"), false);

        // get_header, success
        assert_eq!(&req.get_header("content-type").unwrap(), "application/json");

        // get_header, non-existent
        assert_eq!(req.get_header("not-here"), None);
    }

    #[wasm_bindgen_test]
    fn req_body() {
        let ascii_text = "hello-world";

        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            web_sys::Headers::new().unwrap(),
            Some(ascii_text.as_bytes().to_vec()),
        );

        assert_eq!(req.body().unwrap(), ascii_text.as_bytes());

        let body_bin = vec![0, 1, 2, 3];

        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            web_sys::Headers::new().unwrap(),
            Some(body_bin),
        );
        let body = req.body().unwrap();
        assert_eq!(body.len(), 4);
        assert_eq!(body[1], 1);
    }

    #[wasm_bindgen_test]
    fn req_query() {
        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com?fruit=apple&shape=round").unwrap(),
            web_sys::Headers::new().unwrap(),
            None,
        );

        assert_eq!(req.get_query_value("fruit").unwrap(), "apple");
        assert_eq!(req.get_query_value("shape").unwrap(), "round");
        assert_eq!(req.get_query_value("size"), None);
    }

    #[wasm_bindgen_test]
    fn req_cookie() {
        let headers = web_sys::Headers::new().expect("new");
        headers.set("Cookie", "foo=bar;color=green").expect("ok");

        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            headers,
            None,
        );

        assert_eq!(&req.get_cookie_value("foo").unwrap(), "bar");
        assert_eq!(req.get_cookie_value("bar"), None);
        assert_eq!(&req.get_cookie_value("color").unwrap(), "green");

        // test parsing of cookie with spaces around ';'
        let headers = web_sys::Headers::new().expect("new");
        headers
            .set("Cookie", "foo=bar ; color=green ; bar=baz")
            .expect("ok");
        let req = Request::new(
            Method::GET,
            Url::parse("https://www.example.com").unwrap(),
            headers,
            None,
        );
        assert_eq!(&req.get_cookie_value("foo").unwrap(), "bar"); // after
        assert_eq!(&req.get_cookie_value("color").unwrap(), "green"); // before and after
        assert_eq!(&req.get_cookie_value("bar").unwrap(), "baz"); //  before
    }
}
