// devela::examples::lang::js_web_worker::main
//
//! A Web API canvas example.
//

#![allow(unused)]
// global config
#![no_std]
#![allow(clippy::deref_addrof, reason = "safe references to static mut")]
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html#safe-references

use devela::{JsConsole as console, Web, WebWorker, format_buf as fmt, set_panic_handler};

set_panic_handler! { web }

// #[global_allocator]
// static ALLOCATOR: WasmAlloc = WasmAlloc::INIT;

/// Buffer for strings.
static mut BUF: [u8; 1024] = [0; 1024];
/// Buffer for workers.
static mut WBUF: [WebWorker; 10] = [WebWorker::invalid(); 10];
/// Buffer for job results
static mut JBUF: [u8; 256] = [0; 256];

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    let buf = unsafe { &mut *&raw mut BUF };

    console::log("Initializing Web Workers...");

    // Start Worker from File
    let w_file = WebWorker::spawn("worker.js").expect("new worker");
    console::log(fmt!(?buf, "Started worker from file with ID: {}", w_file.id()));

    // Start Worker from String
    let script = r#"
        console.info(">>> Worker 2 started from string is executing!");
        self.onmessage = (event) => {
            if (event.data.type === "eval") {
                try {
                    const result = eval(event.data.jsCode);
                    self.postMessage({
                        type: "eval_result",
                        jobId: event.data.jobId, result
                    });
                } catch (error) {
                    self.postMessage({
                        type: "eval_result",
                        jobId: event.data.jobId,
                        result: `Error: ${error.message}`
                    });
                }
            } else if (event.data.type === "message") {
                self.postMessage({
                    type: "message_response",
                    message: `Worker 2 echoes: ${event.data.message}`
                });
            }
        };
    "#;
    let w_string = Web::worker_spawn(script).expect("new worker");
    console::log(fmt!(?buf, "Started worker from string with ID: {}", w_string.id()));
    show_workers();

    // Evaluate JavaScript in Worker
    let eval_js = "Math.random() * 100";
    console::log("Evaluating JavaScript in worker...");

    let j1 = w_file.eval(eval_js).expect("1st job");
    console::log(fmt!(?buf, "job1 = ({j1:?})"));

    console::info("Polling workers:");
    for w in 0..3 {
        let res1 = j1.poll(unsafe { &mut *&raw mut JBUF });
        // let res1 = j1.poll_alloc(); // alternative with alloc
        console::log(fmt!(?buf, "worker-{w} = ({res1:?})"));
    }

    // Send Message to Worker
    let message = "Hello from Rust!";
    console::log(fmt!(?buf, "Sending message to worker {w_string:?}: {message}"));

    Web::worker_send_message(w_string, message);
    show_workers();

    // Stop Workers
    // console::log(fmt!(?buf, "Stopping worker {}…", w_file.id()));
    // w_file.stop();
    // show_workers();

    console::log("All done!");
}

/// Show information about active workers.
fn show_workers() {
    let buf = unsafe { &mut *&raw mut BUF };
    let wbuf = unsafe { &mut *&raw mut WBUF };

    let workers = Web::worker_list_len() as usize;
    assert_eq![workers, Web::worker_list_buf(wbuf)];
    let worker_list = &wbuf[..workers];

    console::info(fmt!(?buf, "Current workers: ({workers}) = {worker_list:?}"));
}
