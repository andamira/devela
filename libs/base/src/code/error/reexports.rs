// devela::code::error::reexports
//
//! Reexported error-related items.
//

#![allow(unused_imports)]

// TEMP
pub use core::compile_error;
pub use core::error::Error;

// TODO
// use crate::reexport;
// reexport! { rust: core::error,
//     doc: "A trait representing the basic expectations for error values.",
//     Error
// }
//
// reexport! { rust: core,
//     doc: "Causes compilation to fail with the given error message when encountered.",
//     compile_error
// }
