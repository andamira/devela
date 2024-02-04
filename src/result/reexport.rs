// devela::result::reexport_std
//
//! Reexported items from `std`.
//

use crate::code::reexport;

reexport! {non-optional "either" | either,
    doc: "A general purpose sum type with two cases: `Left` and `Right`.",
    Either
}
// for_both
// try_left
// try_right

#[cfg(feature = "std")]
pub use std::*;
#[cfg(feature = "std")]
mod std {
    use super::reexport;

    reexport! { rust: no_std|std::error, local_module: "result",
        doc: "A trait representing the basic expectations for error values.",
        Error
    }
}
