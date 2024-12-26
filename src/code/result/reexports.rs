// devela::code::result::reexports
//
//! Reexported result-related items.
//

#![allow(unused_imports)]

use crate::reexport;

/* `core` re-exports */

reexport! { rust: core,
    doc: "Causes compilation to fail with the given error message when encountered.",
    compile_error
}
reexport! { rust: core::result,
    tag: crate::TAG_RESULT!(),
    doc: "A type that represents either success ([`Ok`]) or failure ([`Err`]).",
    Result
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
