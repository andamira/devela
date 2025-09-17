// devela_base_alloc::text::str:reexports
//
//! String related re-exports.
//!
//! Reexport the *const-str* crate macros related to string slices,
//! prefixed with `str_` and with a new first line of documentation.
//

#![allow(unused_imports)]

use crate::{_TAG_TEXT, _reexport};

_reexport! { rust: alloc::string,
    tag: _TAG_TEXT!(),
    doc: "A UTF-8–encoded, growable string.",
    String
}
_reexport! { rust: alloc::string,
    tag: _TAG_TEXT!(),
    doc: "A trait for converting a value to a [`String`].",
    ToString
}
