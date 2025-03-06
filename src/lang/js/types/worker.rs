// devela::lang:js::types::worker
//
// TOC
// - struct JsWorker
// - struct JsWorkerError
// - struct JsWorkerJob

#[cfg(feature = "alloc")]
use crate::String;
#[allow(unused_imports)]
use crate::{Js, TaskPoll};

/// A handle to a JavaScript Web Worker.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Worker>.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsWorker {
    pub(in crate::lang::js) id: u32,
}

#[rustfmt::skip]
impl JsWorker {
    /// Returns a new invalid worker.
    pub const fn invalid() -> Self { JsWorker { id: 0 } }
    /// Returns the worker's ID.
    pub const fn id(self) -> u32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(not(windows), feature = "unsafe_ffi"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
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
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JsWorkerJob {
    pub(in crate::lang::js) worker: JsWorker,
    pub(in crate::lang::js) id: u32,
}

#[rustfmt::skip]
impl JsWorkerJob {
    /// Returns the associated worker.
    pub const fn worker(self) -> JsWorker { self.worker }
    /// Returns the job's ID.
    pub const fn id(self) -> u32 { self.id }
}

#[rustfmt::skip]
#[cfg(all(not(windows), feature = "unsafe_ffi"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
impl JsWorkerJob {
    /// Polls the result of this job.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "alloc")))]
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
