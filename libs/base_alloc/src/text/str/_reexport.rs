// devela_base_alloc::text::str::_reexport

#![allow(unused_imports)]

use crate::{_TAG_TEXT, _TAG_VALUE, _reexport};

_reexport! { rust: alloc::string, location: "text/str", tag: _TAG_TEXT!(),
    doc: "A UTF-8–encoded, growable string.",
    String
}
_reexport! { rust: alloc::string, location: "text/str", tag: _TAG_TEXT!() _TAG_VALUE!(),
    doc: "A trait for converting a value to a [`String`].",
    ToString
}
