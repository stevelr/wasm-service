#![deny(missing_docs)]
//! Base support for wasm service using Confluence Workers
//!
use async_trait::async_trait;
use js_sys::{Function, Reflect};
use service_logging::{log, LogEntry, LogQueue, Logger, Severity};
use std::sync::Mutex;
use wasm_bindgen::JsValue;

mod error;
pub use error::Error;
mod method;
pub use method::Method;
mod request;
pub use request::Request;
mod response;
pub use response::{Body, Response};
mod media_type;
pub use media_type::media_type;

/// re-export url::Url
pub use url::Url;

mod context;
pub use context::Context;
mod assets;
pub use assets::StaticAssetHandler;
mod httpdate;
pub(crate) mod js_values;
pub use httpdate::HttpDate;

/// Logging support for deferred tasks
#[derive(Debug)]
pub struct RunContext {
    /// queue of deferred messages
    pub log_queue: Mutex<LogQueue>,
}

impl RunContext {
    /// log message (used by log! macro)
    pub fn log(&self, entry: LogEntry) {
        let mut guard = match self.log_queue.lock() {
            Ok(guard) => guard,
            Err(_poisoned) => {
                // lock shouldn't be poisoned because we don't have panics in production wasm,
                // so this case shouldn't occur
                return;
            }
        };
        guard.log(entry);
    }
}

/// Runnable trait for deferred tasks
/// Deferred tasks are often useful for logging and analytics.
/// ```rust
/// use std::{rc::Rc,sync::Mutex};;
/// use async_trait::async_trait;
/// use service_logging::{log,Logger,LogQueue,Severity};
/// use wasm_service::{Runnable,RunContext};
///
/// struct Data { s: String }
/// #[async_trait]
/// impl Runnable for Data {
///     async fn run(&self, ctx: &RunContext) {
///         log!(ctx, Severity::Info, msg: format!("Deferred with data: {}", self.s ));
///     }
/// }
/// ```
#[async_trait]
pub trait Runnable {
    /// Execute a deferred task. The task may append
    /// logs to `lq` using the [`log`] macro. Logs generated
    /// are sent to the log service after all deferred tasks have run.
    ///
    /// Note that if there is a failure sending logs to the logging service,
    /// those log messages (and the error from the send failure) will be unreported.
    async fn run(&self, ctx: &RunContext);
}

/// Generic page error return - doesn't require ctx
#[derive(Clone, Debug)]
pub struct HandlerReturn {
    /// status code (default: 200)
    pub status: u16,
    /// body text
    pub text: String,
}

/// Generate handler return "error"
pub fn handler_return(status: u16, text: &str) -> HandlerReturn {
    HandlerReturn {
        status,
        text: text.to_string(),
    }
}

impl Default for HandlerReturn {
    fn default() -> Self {
        Self {
            status: 200,
            text: String::default(),
        }
    }
}

/// Trait that defines app/service's request handler and router
/// See [rustwasm-service-template](https://github.com/stevelr/rustwasm-service-template/blob/master/src/lib.rs)
///   for a more complete example
///
///```rust
/// use service_logging::{Severity::Verbose,log,Logger};
/// use wasm_service::{Context,Handler,HandlerReturn,Request};
/// use async_trait::async_trait;
/// struct MyHandler {}
/// #[async_trait(?Send)]
/// impl Handler for MyHandler {
///     /// Process incoming Request
///     async fn handle(&self, req: &Request, ctx: &mut Context) -> Result<(), HandlerReturn> {
///         // log all incoming requests
///         log!(ctx, Verbose, method: req.method(), url: req.url());
///         match (req.method(), req.url().path()) {
///             (GET, "/hello") => {
///                 ctx.response().content_type("text/plain; charset=UTF-8").unwrap()
///                               .text("Hello world!");
///             }
///             _ => {
///                 ctx.response().status(404).text("Not Found");
///             }
///         }
///         Ok(())
///     }
/// }
///```
#[async_trait(?Send)]
pub trait Handler {
    /// Implementation of application request handler
    async fn handle(&self, req: &Request, ctx: &mut Context) -> Result<(), HandlerReturn>;
}

/// Configuration parameters for service
/// Parameter E is your crate's error type
pub struct ServiceConfig {
    /// Logger
    pub logger: Box<dyn Logger>,

    /// Request handler
    pub handlers: Vec<Box<dyn Handler>>,

    /// how to handle internal errors. This function should modify ctx.response() with results,
    /// which, for example, could include rendering a page or sending a redirect.
    /// The default implementation returns status 200 with a short text message.
    pub internal_error_handler: fn(req: &Request, ctx: &mut Context),
}

impl Default for ServiceConfig {
    /// Default construction of ServiceConfig does no logging and handles no requests.
    fn default() -> ServiceConfig {
        ServiceConfig {
            logger: service_logging::silent_logger(),
            handlers: Vec::new(),
            internal_error_handler: default_internal_error_handler,
        }
    }
}

struct DeferredData {
    tasks: Vec<Box<dyn Runnable + std::panic::UnwindSafe>>,
    logs: Vec<LogEntry>,
    logger: Box<dyn Logger>,
}

/// Entrypoint for wasm-service. Converts parameters from javascript into [Request],
/// invokes app-specific [Handler](trait.Handler.html), and converts [`Response`] to javascript.
/// Also sends logs to [Logger](https://docs.rs/service-logging/0.3/service_logging/trait.Logger.html) and runs deferred tasks.
pub async fn service_request(req: JsValue, config: ServiceConfig) -> Result<JsValue, JsValue> {
    let map = js_sys::Map::from(req);
    let req = Request::from_js(&map)?;
    // From incoming request, extract 'event' object, and get ref to its 'waitUntil' function
    let js_event = js_sys::Object::from(check_defined(map.get(&"event".into()), "missing event")?);
    let wait_func = Function::from(
        Reflect::get(&js_event, &JsValue::from_str("waitUntil"))
            .map_err(|_| "event without waitUntil")?,
    );
    let mut ctx = Context::default();
    let mut handler_result = Ok(());
    for handler in config.handlers.iter() {
        if ctx.is_internal_error().is_some() {
            (config.internal_error_handler)(&req, &mut ctx);
            break;
        }
        handler_result = handler.handle(&req, &mut ctx).await;
        // if handler set response, or returned HandlerReturn (which is a response), stop iter
        if handler_result.is_err() || !ctx.response().is_unset() {
            break;
        }
    }
    if let Err(result) = handler_result {
        // Convert HandlerReturn to status/body
        ctx.response().status(result.status).text(result.text);
    } else {
        // if no handler set response (status or body), create fallback 404 response
        if ctx.response().is_unset() {
            ctx.response().status(404).text("Not Found");
        }
    }
    let response = ctx.take_response();
    log!(ctx, Severity::Verbose, _:"service",
        method: req.method(), url: req.url(), status: response.get_status());
    // this should always return OK (event has waitUntil property) unless api is broken.
    let promise = deferred_promise(Box::new(DeferredData {
        tasks: ctx.take_tasks(),
        logs: ctx.take_logs(),
        logger: config.logger,
    }));
    let _ = wait_func.call1(&js_event, &promise); // todo: handle result
    Ok(response.into_js())
}

/// Default implementation of internal error handler
/// Sets status to 200 and returns a short error message
fn default_internal_error_handler(req: &Request, ctx: &mut Context) {
    let error = ctx.is_internal_error();
    log!(ctx, Severity::Error, _:"InternalError", url: req.url(),
        error: error.map(|e| e.to_string()).unwrap_or_else(|| String::from("none")));
    ctx.response()
        .status(200)
        .content_type(mime::TEXT_PLAIN_UTF_8)
        .unwrap()
        .text("Sorry, an internal error has occurred. It has been logged.");
}
/// Future task that will run deferred. Includes deferred logs plus user-defined tasks.
/// This function contains a rust async wrapped in a Javascript Promise that will be passed
/// to the event.waitUntil function, so it gets processed after response is returned.
fn deferred_promise(args: Box<DeferredData>) -> js_sys::Promise {
    wasm_bindgen_futures::future_to_promise(async move {
        // send first set of logs
        if let Err(e) = args.logger.send("http", args.logs).await {
            log_log_error(e);
        }
        // run each deferred task
        let log_queue = Mutex::new(LogQueue::default());
        let run_ctx = RunContext { log_queue };
        for t in args.tasks.iter() {
            t.run(&run_ctx).await;
        }
        // if any logs were generated during processing of deferred tasks, send those
        let mut lock_queue = run_ctx.log_queue.lock().unwrap();
        if let Err(e) = args.logger.send("http", lock_queue.take()).await {
            log_log_error(e);
        }
        // all done, return nothing
        Ok(JsValue::undefined())
    })
}

/// Returns javascript value, or Err if undefined
fn check_defined(v: JsValue, msg: &str) -> Result<JsValue, JsValue> {
    if v.is_undefined() {
        return Err(JsValue::from_str(msg));
    }
    Ok(v)
}

/// logging fallback: if we can't send to external logger,
/// log to "console" so it can be seen in worker logs
fn log_log_error(e: Box<dyn std::error::Error>) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "Error sending logs: {:?}",
        e
    )))
}
