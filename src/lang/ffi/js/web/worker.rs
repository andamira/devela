// devela::lang::ffi::js::web::worker
//
//! Defines [`WebWorker`], [`WebWorkerError`], [`WebWorkerJob`].
//
// TOC
// - struct WebWorker
// - struct WebWorkerError
// - struct WebWorkerJob

#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", feature = "alloc", not(windows)))]
use crate::String;
#[allow(unused_imports)]
use crate::{TaskPoll, Web, js_uint32};

/// A handle to a JavaScript Web Worker.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Worker>.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebWorker {
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl WebWorker {
    /// Returns a new invalid worker.
    pub const fn invalid() -> Self { WebWorker { id: 0 } }
    /// Returns the worker's ID.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebWorker {
    /// Spawns a new JavaScript Web Worker from a script.
    pub fn spawn(script: &str) -> Result<Self, WebWorkerError> { Web::worker_spawn(script) }
    /// Stops this Web Worker.
    pub fn stop(self) { Web::worker_stop(self); }
    /// Checks if this worker is still active by querying JavaScript.
    pub fn is_active(self) -> bool { Web::worker_is_active(self) }
    /// Sends a message to this Web Worker.
    pub fn send_message(self, message: &str) { Web::worker_send_message(self, message); }
    /// Executes JavaScript inside this worker, returning a `WebWorkerJob`.
    pub fn eval(self, js_code: &str) -> Result<WebWorkerJob, WebWorkerError> {
        if !self.is_active() { return Err(WebWorkerError::WorkerNotFound); }
        let job = Web::worker_eval(self, js_code);
        if job.id == 0 { return Err(WebWorkerError::WorkerNotFound); }
        Ok(job)
    }
}

/// Errors that can occur when working with JavaScript Web Workers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebWorkerError {
    /// The worker script provided was invalid.
    InvalidScript,
    /// The worker was not found.
    WorkerNotFound,
    /// The job was not found.
    JobNotFound,
}

/// Represents a job running inside a [`WebWorker`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WebWorkerJob {
    pub(in crate::lang::ffi::js) worker: WebWorker,
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl WebWorkerJob {
    /// Returns the associated worker.
    pub const fn worker(self) -> WebWorker { self.worker }
    /// Returns the job's ID.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(not(feature = "safe_lang"))]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl WebWorkerJob {
    /// Polls the result of this job and writes it into `buffer`.
    ///
    /// Returns the number of bytes written to the buffer.
    pub fn poll(self, buffer: &mut [u8]) -> TaskPoll<Result<usize, WebWorkerError>> {
        Web::worker_poll(self, buffer)
    }
    /// Polls the result of this job.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn poll_alloc(self) -> TaskPoll<Result<String, WebWorkerError>> {
        Web::worker_poll_alloc(self)
    }
    /// Cancels this job.
    pub fn cancel(self) { Web::worker_eval_cancel(self); }
}
