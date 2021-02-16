/// wasm-service template
///
use async_trait::async_trait;
use cfg_if::cfg_if;
use service_logging::{log, CoralogixConfig, CoralogixLogger, Severity};
use wasm_bindgen::{prelude::*, JsValue};
use wasm_service::{Context, Handler, HandlerReturn, Method::GET, Request, ServiceConfig};

// compile-time config settings, defined in config.toml
mod config;
use config::CONFIG;

cfg_if! {
    if #[cfg(feature="wee_alloc")] {
        // Use `wee_alloc` as the global allocator.
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

struct MyHandler {}
#[async_trait(?Send)]
impl Handler for MyHandler {
    async fn handle(&self, req: &Request, ctx: &mut Context) -> Result<(), HandlerReturn> {
        match (req.method(), req.url().path()) {
            (GET, "/") => {
                log!(ctx, Severity::Info, _:"hello logger");
                ctx.response().text("OK");
            }
            (GET, "/hello") => {
                ctx.response().text("Hello friend!");
            }
            _ => {} // 404 fallthrough is handled by wasm-service
        }
        Ok(())
    }
}

/// Main entry to service worker, called from javascript
#[wasm_bindgen]
pub async fn main_entry(req: JsValue) -> Result<JsValue, JsValue> {
    let logger = match CONFIG.logging.logger.as_ref() {
        //"console" => ConsoleLogger::init(),
        "coralogix" => CoralogixLogger::init(CoralogixConfig {
            api_key: &CONFIG.logging.coralogix.api_key,
            application_name: &CONFIG.logging.coralogix.application_name,
            endpoint: &CONFIG.logging.coralogix.endpoint,
        })
        .map_err(|e| JsValue::from_str(&e.to_string()))?,
        _ => {
            return Err(JsValue::from_str(&format!(
                "Invalid logger configured:'{}'",
                CONFIG.logging.logger
            )));
        }
    };
    wasm_service::service_request(
        req,
        ServiceConfig {
            logger,
            handlers: vec![Box::new(MyHandler {})],
            ..Default::default()
        },
    )
    .await
}
