// devela::lang::js
//
//! JavaScript Interfacing.
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
//! - **Web Workers** allow parallel execution but **cannot block** execution.
//! - Delays can be simulated using `setTimeout()` or `Atomics.wait()` in shared memory.
//!
//! While Web Workers enable concurrency, they communicate via message passing
//! and do not share memory except through `SharedArrayBuffer`.
mod types; // Js, JsEvent, JsPermission*, JsWorker...

#[cfg(feature = "unsafe_ffi")]
crate::items! {
    mod reexport; // js_reexport!

    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_ffi")))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(target_arch = "wasm32")))]
    #[cfg(not(windows))]
    mod web_api;
}

crate::items! { // structural access: _mods, _internals, _all
    #[allow(unused)]
    pub use {_mods::*, _internals::*};

    mod _mods { #![allow(unused)]
        pub use super::types::*;

        #[cfg(feature = "unsafe_ffi")]
        pub use super::reexport::*;
    }
    pub(super) mod _internals {
        #[cfg(all(feature = "unsafe_ffi", not(windows)))]
        pub(crate) use super::web_api::web_api;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
