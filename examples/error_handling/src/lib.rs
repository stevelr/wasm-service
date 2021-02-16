use async_trait::async_trait;
#[cfg(any(doc, target_arch = "wasm32"))]
use service_logging::ConsoleLogger;
use wasm_bindgen::JsValue;
use wasm_service::{Context, Handler, HandlerReturn, Request, ServiceConfig};

/// Catch-all error handler that generates error page
fn internal_error(e: impl std::error::Error) -> HandlerReturn {
    HandlerReturn {
        status: 200,
        text: format!(
            r#"<!DOCTYPE html>
            <html>
            <head><title>Server error</title></head>
            <body>
            <h1>Error</h1>
            <p>Internal error has occurred: {:?}</p>
            </body>
            </html>"#,
            e
        ),
    }
}

/// The '/err' url has a bug (intentional) that will result in generation of the internal_error page.
struct MyHandler {}
#[async_trait(?Send)]
impl Handler for MyHandler {
    /// Process incoming Request
    async fn handle(&self, req: &Request, ctx: &mut Context) -> Result<(), HandlerReturn> {
        use wasm_service::Method::GET;

        match (req.method(), req.url().path()) {
            (GET, "/") => {
                ctx.response().text("OK");
            }
            (GET, "/err") => {
                // demonstration of using 'internal_error' with '?' to generate minimal error page.
                let x: i32 = "not_an_int".parse().map_err(internal_error)?;
                ctx.response().text(format!("you never see this: {}", x));
            }
            _ => {
                ctx.response().status(404).text("Not Found");
            }
        }
        Ok(())
    }
}

pub async fn main_entry(req: JsValue) -> Result<JsValue, JsValue> {
    wasm_service::service_request(
        req,
        ServiceConfig {
            logger: service_logging::ConsoleLogger::init(),
            handlers: vec![Box::new(MyHandler {})],
            ..Default::default()
        },
    )
    .await
}
