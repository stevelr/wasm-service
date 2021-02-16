## 0.5.0

- feature: support for oauth - see wasm-service-oauth crate
- feature: static asset handler, for serving static files
- feature: add media_type() for looking up media type (aka MIME) based on file extension.  
- feature: added internal_error_handler in ServiceConfig.
  If you don't want to write one, a default implementation is included in ServiceConfig::default()
- added two example projects: simple/ and error_handling/
- removed context.default_content_type
- added Request.is_empty() to test whether POST/PUT body has zero bytes
- made ctx.take_logs() public
- removed Mutex for deferred task logging - not needed because
  workers are single-threaded
- response.body() takes Into(Body) so can take a wider range of parameters
- additional dependencies: mime,bincode,chrono,kv_assets
- upgrade dependencies: reqwest 0.11
  
## v0.4.0 2021-01-12

- response.body takes Vec<u8> instead of &[u8] to avoid an extra copy
- added impl Default for ServiceConfig

## v0.3

- __Breaking changes__ to support add-ons
  - for `run()`, if you did logging with `log!(lq,...`, this changes to
    `log(ctx,...`
  - `handle()` changes from
  ```async fn handle(&self, ctx: &mut Context) -> Result<(), E>;```
  to
  ```async fn handle(&self, req: &Request, ctx: &mut Context) -> Result<(), HandlerReturn>```
  - Request is available as a separate parameter to `handle`, so instead
    of `ctx.request()` you can just use `req`.
    This avoids conflicting borrows from immutable `req`,
    and mutable `ctx` (needed for logging and updating Response).
  - The error return type is `HandlerReturn` instead of a generic
    error trait `E`. HandlerReturn can be used for 302 redirects, or
    can contain the body of a response page. As the Err() variant
    of Result, it makes it easy to return a response with the '?'
    operator. This also forces the developer to think about how internal
    errors should translate into user-visible http responses.
  - `service_request` takes an array of Handlers instead of one,
    so Handlers may be chained, and used like "middleware"
    or plugins. An Oauth2 plugin is under development.

- New features:
  - Support for add-ons, implemented as a chain of Handlers.
  - methods: [`Request.get_cookie_value`], [`Request.get_query_value`],
  [`Response.is_unset`]
  - enum: [`Method::OPTIONS`]

- Fixes
  - moved wasm-bindgen-test to dev-dependencies

- Other
  - More unit tests. To run all tests, run both:
    - cargo test
    - wasm-pack test --firefox --headless

