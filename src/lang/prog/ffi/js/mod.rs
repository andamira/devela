// devela::lang::prog::ffi::js
//
//! <a href="https://en.wikipedia.org/wiki/JavaScript">JavaScript</a> interfacing.
//!
//! # Overview
//!
//! - JavaScript is a high-level, dynamically typed language
//!   with an **event-driven, single-threaded** execution model.
//! - It primarily runs in **web browsers**, interacting with the DOM,
//!   but is also used in other environments.
//! - Blocking the main thread prevents UI updates and user interactions,
//!   requiring **asynchronous programming** via callbacks, promises, or `async/await`.
//!
//! # Syntax Basics
//!
//! ```js
//! // Variables
//! let x;
//! x = 1;          // Integer
//! x = 0.1;        // Floating-point number
//! x = "hello";    // String (double quotes)
//! x = 'hello';    // String (single quotes)
//! x = true;       // Boolean
//! x = null;       // Explicit "no value"
//! x = undefined;  // Uninitialized or missing value
//!
//! // Objects (key-value pairs)
//! let obj = {
//!     field1: "string",
//!     field2: 5
//! };
//!
//! // Arrays (heterogeneous lists)
//! let arr = [1, "two", 3.0, false];
//! ```
//!
//! # Concurrency
//!
//! - JavaScript runs in a **single-threaded** environment.
//! - [`WebWorker`][crate::WebWorker]s allow parallel execution but **cannot block** execution.
//! - Delays can be simulated using `setTimeout()` or `Atomics.wait()` in shared memory.
//!
//! While Web Workers enable concurrency, they communicate via message passing
//! and do not share memory except through `SharedArrayBuffer`.
//

mod _helper; // _js_doc!, _js_extern!, js_method_str_alloc!

mod console; // JsConsole
// mod error; // JsError WIP
mod instant; // JsInstant, JsTimeout
mod namespace; // Js
// mod object; // JsObject WIP
mod primitives; // js_number, js_int32, js_unit32, js_bool…
mod text; // JsTextMetrics, JsTextMetricsFull
mod value; // JsValue

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            console::*,
            // error::*,
            instant::*,
            namespace::*,
            // object::*,
            primitives::*,
            text::*,
            value::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
