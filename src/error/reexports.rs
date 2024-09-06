// devela::error::reexports
//
//! Reexported items.
//

#![allow(unused_imports)]

use crate::code::reexport;

/* `core::result` re-exports */

reexport! { rust: core::error,
    doc: "A trait representing the basic expectations for error values.",
    Error
}

#[cfg(feature = "std")]
pub use std::*;
#[cfg(feature = "std")]
mod std {
    use super::reexport;

    reexport! { rust: std::backtrace,
        doc: "A captured OS thread stack backtrace.",
        Backtrace
    }
    reexport! { rust: std::backtrace,
        doc: "The current status of a backtrace.",
        BacktraceStatus
    }
}
