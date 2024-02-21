// devela::result::reexport_std
//
//! Reexported items from `std`.
//

use crate::code::reexport;

/* crate */

#[doc(inline)]
#[cfg(feature = "io")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "io")))]
pub use crate::io::IoError;
#[doc(inline)]
#[cfg(feature = "text")]
pub use crate::text::{ArrayStringError, CharConversionError};
#[doc(inline)]
pub use crate::{
    data::{DataError, DataResult},
    gfx::{GfxError, GfxResult},
    num::{NumError, NumResult},
};

/* std */

reexport! {non-optional "either" | either,
    doc: "A general purpose sum type with two cases: `Left` and `Right`.",
    Either
}
reexport! {non-optional "either" | either,
    doc: "Evaluate the provided expression for both [`Either::Left`] and [`Either::Right`]",
    @for_both as either_for_both
}
reexport! {non-optional "either" | either,
    doc: "Unwraps the left side of an [`Either`], which fails early with the opposite side.",
    @try_left as either_try_left
}
reexport! {non-optional "either" | either,
    doc: "Unwraps the right side of an [`Either`], which fails early with the opposite side.",
    @try_right as either_try_right
}

#[cfg(feature = "std")]
pub use std::*;
#[cfg(feature = "std")]
mod std {
    use super::reexport;

    // In sync with super::define_no_std_error::Error
    reexport! { rust: not(std)|std::error,
        doc: "A trait representing the basic expectations for error values.",
        Error
    }
}
