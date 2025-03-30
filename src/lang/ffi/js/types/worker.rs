// devela::lang::ffi::js::types::worker
//
// TOC
// - struct JsWorker
// - struct JsWorkerError
// - struct JsWorkerJob

#[cfg(all(feature = "unsafe_ffi", feature = "alloc", not(windows)))]
use crate::String;
#[allow(unused_imports)]
use crate::{Js, TaskPoll, js_uint32};

/// A handle to a JavaScript Web Worker.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Worker>.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsWorker {
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl JsWorker {
    /// Returns a new invalid worker.
    pub const fn invalid() -> Self { JsWorker { id: 0 } }
    /// Returns the worker's ID.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsWorker {
    /// Spawns a new JavaScript Web Worker from a script.
    pub fn spawn(script: &str) -> Result<Self, JsWorkerError> { Js::worker_spawn(script) }
    /// Stops this Web Worker.
    pub fn stop(self) { Js::worker_stop(self); }
    /// Checks if this worker is still active by querying JavaScript.
    pub fn is_active(self) -> bool { Js::worker_is_active(self) }
    /// Sends a message to this Web Worker.
    pub fn send_message(self, message: &str) { Js::worker_send_message(self, message); }
    /// Executes JavaScript inside this worker, returning a `JsWorkerJob`.
    pub fn eval(self, js_code: &str) -> Result<JsWorkerJob, JsWorkerError> {
        if !self.is_active() { return Err(JsWorkerError::WorkerNotFound); }
        let job = Js::worker_eval(self, js_code);
        if job.id == 0 { return Err(JsWorkerError::WorkerNotFound); }
        Ok(job)
    }
}

/// Errors that can occur when working with JavaScript Web Workers.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JsWorkerError {
    /// The worker script provided was invalid.
    InvalidScript,
    /// The worker was not found.
    WorkerNotFound,
    /// The job was not found.
    JobNotFound,
}

/// Represents a job running inside a [`JsWorker`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JsWorkerJob {
    pub(in crate::lang::ffi::js) worker: JsWorker,
    pub(in crate::lang::ffi::js) id: js_uint32,
}

#[rustfmt::skip]
impl JsWorkerJob {
    /// Returns the associated worker.
    pub const fn worker(self) -> JsWorker { self.worker }
    /// Returns the job's ID.
    pub const fn id(self) -> js_uint32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(feature = "unsafe_ffi", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(nightly_doc, doc(cfg(target_arch = "wasm32")))]
impl JsWorkerJob {
    /// Polls the result of this job.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(target_arch = "alloc")))]
    pub fn poll(self) -> TaskPoll<Result<String, JsWorkerError>> { Js::worker_poll(self) }
    /// Polls the result of this job and writes it into `buffer`.
    ///
    /// Returns the number of bytes written to the buffer.
    pub fn poll_buf(self, buffer: &mut [u8]) -> TaskPoll<Result<usize, JsWorkerError>> {
        Js::worker_poll_buf(self, buffer)
    }
    /// Cancels this job.
    pub fn cancel(self) { Js::worker_eval_cancel(self); }
}
