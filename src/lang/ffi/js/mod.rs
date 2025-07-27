// devela::lang::ffi::js
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
//! - **Web Workers** allow parallel execution but **cannot block** execution.
//! - Delays can be simulated using `setTimeout()` or `Atomics.wait()` in shared memory.
//!
//! While Web Workers enable concurrency, they communicate via message passing
//! and do not share memory except through `SharedArrayBuffer`.

mod reexport;

mod types; // js_number, js_int32, js_unit32, js_bool…
mod web; // Js, ...

crate::items! { // structural access: _mods, _pub_mods, _internals, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::types::*;
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::web::_all::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::reexport::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
