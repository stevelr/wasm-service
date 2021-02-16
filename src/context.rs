use crate::Response;
use crate::Runnable;
use service_logging::{LogEntry, LogQueue};
use std::panic::UnwindSafe;

/// Context manages the information flow for an incoming HTTP [`Request`],
/// the application handler, and the generated HTTP [`Response`]. It holds a buffer
/// for log messages, and a hook for deferred tasks to be processed after the [`Response`] is returned.
#[derive(Default)]
pub struct Context {
    response: Response,
    log_queue: LogQueue,
    deferred: Vec<Box<dyn Runnable + UnwindSafe>>,
    internal_error: Option<Box<dyn std::error::Error>>,
}

unsafe impl Send for Context {}

impl Context {
    /// Creates response builder
    pub fn response(&mut self) -> &mut Response {
        &mut self.response
    }

    /// Adds a task to the deferred task queue. The task queue uses
    /// [event.waitUntil](https://developers.cloudflare.com/workers/runtime-apis/fetch-event)
    /// to extend the lifetime of the request event, and runs tasks after the response
    /// has been returned to the client.
    /// Deferred tasks are often useful for logging and analytics.
    pub fn defer(&mut self, task: Box<dyn Runnable + UnwindSafe>) {
        self.deferred.push(task);
    }

    /// Returns pending log messages, emptying internal queue.
    /// This is used for sending queued messages to an external log service
    pub fn take_logs(&mut self) -> Vec<LogEntry> {
        self.log_queue.take()
    }

    /// Returns deferred tasks, emptying internal list
    pub(crate) fn take_tasks(&mut self) -> Vec<Box<dyn Runnable + UnwindSafe>> {
        std::mem::take(&mut self.deferred)
    }

    /// Returns response, replacing self.response with default
    pub(crate) fn take_response(&mut self) -> Response {
        std::mem::take(&mut self.response)
    }

    /// Adds log to deferred queue
    pub fn log(&mut self, e: LogEntry) {
        self.log_queue.log(e);
    }

    /// Sets the internal error flag, which causes wasm_service to invoke the internal_error_handler
    pub fn raise_internal_error(&mut self, e: Box<dyn std::error::Error>) {
        self.internal_error = Some(e);
    }

    /// Returns whether the internal error flag has been set
    pub fn is_internal_error(&self) -> Option<&dyn std::error::Error> {
        self.internal_error.as_deref()
    }
}
