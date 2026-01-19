// devela_base_alloc::text::str::_reexport

#![allow(unused_imports)]

use crate::{_TAG_TEXT, _TAG_VALUE, _reexport, _tags};

_reexport! { rust: alloc::string, location: "text/str", tag: _tags!(text error),
    doc: "A possible error value when converting a String from a UTF-8 byte vector.",
    FromUtf8Error
}
_reexport! { rust: alloc::string, location: "text/str", tag: _tags!(text),
    doc: "A UTF-8–encoded, growable string.",
    String
}
_reexport! { rust: alloc::string, location: "text/str", tag: _tags!(text value),
    doc: "A trait for converting a value to a [`String`].",
    ToString
}
