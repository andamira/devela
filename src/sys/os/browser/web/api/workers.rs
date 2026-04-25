// devela::sys::os::browser::web::api::workers
// (in sync with ./web_api.js)
//
//! Implements the web workers API.
//

use crate::AsyncPoll;
use crate::{_js_extern, js_bool, js_uint32};
#[cfg(feature = "alloc")]
use crate::{Js, String, Vec, js_int32, vec_ as vec};
use crate::{Web, WebWorker, WebWorkerError as Error, WebWorkerJob};

/// Web API workers
#[rustfmt::skip]
impl Web {
    /// Spawns a Web Worker and returns its ID.
    pub fn worker_spawn(script: &str) -> Result<WebWorker, Error> {
        let id = unsafe { worker_spawn(script.as_ptr(), script.len()) };
        if id == 0 { Err(Error::InvalidScript) } else { Ok(WebWorker { id }) }
    }
    /// Checks if this worker is still active by querying JavaScript.
    pub fn worker_is_active(worker: WebWorker) -> js_bool { worker_is_active(worker.id()) }
    /// Stops a specific Web Worker by ID.
    pub fn worker_stop(worker: WebWorker) { worker_stop(worker.id()); }
    /// Stops all Web Workers.
    pub fn worker_stop_all() { worker_stop_all(); }
    /// Returns the number of active workers.
    pub fn worker_list_len() -> usize { worker_list_len() as usize }
    /// Returns the list of active worker IDs.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn worker_list() -> Vec<WebWorker> {
        let len = worker_list_len() as usize;
        let mut workers = vec![WebWorker::default(); len];
        let count = Web::worker_list_buf(&mut workers);
        workers.truncate(count);
        workers
    }
    /// Writes active worker handles into a buffer and returns the number written.
    pub fn worker_list_buf(buffer: &mut [WebWorker]) -> usize {
        let len = worker_list_len() as usize;
        let count = len.min(buffer.len());
        unsafe { worker_list(buffer.as_mut_ptr() as *mut js_uint32, count as js_uint32); }
        count
    }
    /// Sends a message to a specific Web Worker.
    pub fn worker_send_message(worker: WebWorker, msg: &str) {
        unsafe { worker_send_message(worker.id(), msg.as_ptr(), msg.len()); }
    }
    /// Requests execution of JavaScript inside a worker.
    pub fn worker_eval(worker: WebWorker, js_code: &str) -> WebWorkerJob {
        let id = unsafe { worker_eval(worker.id(), js_code.as_ptr(), js_code.len()) };
        WebWorkerJob { worker, id }
    }
    /// Polls for the result of a JavaScript execution in a worker, and writes it into `buffer`.
    ///
    /// If ready, returns the number of bytes written to the buffer.
    pub fn worker_poll(job: WebWorkerJob, buffer: &mut [u8])
        -> AsyncPoll<Result<usize, Error>> {
        if !job.worker().is_active() { return AsyncPoll::Ready(Err(Error::WorkerNotFound)); }
        let written = unsafe { worker_poll(job.id(), buffer.as_mut_ptr(), buffer.len()) };
        match written {
            0 => AsyncPoll::Pending,
            js_uint32::MAX => AsyncPoll::Ready(Err(Error::JobNotFound)),
            _ => AsyncPoll::Ready(Ok(written as usize)),
        }
    }
    /// Polls for the result of a JavaScript execution in a worker.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub fn worker_poll_alloc(job: WebWorkerJob) -> AsyncPoll<Result<String, Error>> {
        if !job.worker().is_active() { return AsyncPoll::Ready(Err(Error::WorkerNotFound)); }
        let mut first_check = true;
        let result = Js::read_string_capped(128, false, |ptr, cap| {
            let res = unsafe { worker_poll(job.id(), ptr, cap as usize) as js_int32 };
            if first_check {
                first_check = false; // Intercept status codes before bothering with decoding
                if res == 0 { return 0; } else if res == -1 { return -1; } // pending or not found
            }
            res
        });
        match result.as_str() {
            "" => AsyncPoll::Pending, // Covers 0 and -1 (mapped above)
            _ => AsyncPoll::Ready(Ok(result)),
        }
    }
    /// Cancels an ongoing JavaScript evaluation.
    pub fn worker_eval_cancel(job: WebWorkerJob) { worker_cancel_eval(job.id()); }
}
_js_extern! {
    [module: "api_workers"]
    unsafe fn worker_spawn(script_ptr: *const u8, script_len: usize) -> js_uint32;
    safe fn worker_is_active(worker_id: js_uint32) -> js_bool;
    safe fn worker_stop(worker_id: js_uint32);
    safe fn worker_stop_all();
    safe fn worker_list_len() -> js_uint32;
    unsafe fn worker_list(worker_list_ptr: *mut js_uint32, len: js_uint32) -> js_uint32;
    unsafe fn worker_send_message(worker_id: js_uint32, msg_ptr: *const u8, msg_len: usize);
    unsafe fn worker_eval(worker_id: js_uint32, js_code_ptr: *const u8, js_code_len: usize)
        -> js_uint32;
    unsafe fn worker_poll(job_id: js_uint32, buffer_ptr: *mut u8, buffer_len: usize)
        -> js_uint32;
    safe fn worker_cancel_eval(job_id: js_uint32);
}
