// devela::text::str:reexports
//
//! String related re-exports.
//!
//! Reexport the *const-str* crate macros related to string slices,
//! prefixed with `str_` and with a new first line of documentation.
//

use crate::{TAG_TEXT, impl_cdef, _reexport};

/* core */

_reexport! { rust: core::str,
    tag: TAG_TEXT!(),
    doc: "Parse a value from a string.",
    FromStr
}

/* alloc */

// WAIT: [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)
// #[cfg(feature = "alloc")] #[doc(inline)] #[rustfmt::skip]
// pub use devela_base_alloc::{String, ToString};
_reexport! { rust: alloc::string,
    tag: TAG_TEXT!(),
    doc: "A UTF-8–encoded, growable string.",
    String
}
_reexport! { rust: alloc::string,
    tag: TAG_TEXT!(),
    doc: "A trait for converting a value to a [`String`].",
    ToString
}

/* std */

_reexport! { rust: std::ffi,
    tag: TAG_TEXT!(),
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
_reexport! { rust: std::ffi,
    tag: TAG_TEXT!(),
    doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
    OsString
}

/* from other modules */

pub use crate::CStr;
#[cfg(feature = "alloc")]
pub use crate::CString;

/* impl ConstDefault */

impl_cdef!["" => &str];
#[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
impl crate::ConstDefault for &mut str {
    // SAFETY: The empty string is valid UTF-8.
    const DEFAULT: Self = unsafe { ::core::str::from_utf8_unchecked_mut(&mut []) };
}
#[cfg(feature = "alloc")]
impl_cdef![Self::new() => String];
