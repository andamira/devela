// devela::text::str:reexports
//
//! String related re-exports.
//!
//! Reexport the *const-str* crate macros related to string slices,
//! prefixed with `str_` and with a new first line of documentation.
//

use crate::{impl_cdef, reexport};

impl_cdef!["" => &str];
#[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
impl crate::ConstDefault for &mut str {
    // SAFETY: The empty string is valid UTF-8.
    const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
}
#[cfg(feature = "alloc")]
impl_cdef![Self::new() => String];

/* from other modules */

pub use crate::CStr;
#[cfg(feature = "alloc")]
pub use crate::CString;
#[cfg(feature = "std")]
crate::items! { pub use crate::{OsStr, OsString}; }

/* core */

reexport! { rust: core::str,
    doc: "Parse a value from a string.",
    FromStr
}

/* alloc */

reexport! { rust: alloc::string,
    doc: "A UTF-8–encoded, growable string.",
    String
}
reexport! { rust: alloc::string,
    doc: "A trait for converting a value to a [`String`].",
    ToString
}
