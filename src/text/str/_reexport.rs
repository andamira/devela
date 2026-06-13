// devela/src/text/str/_reexport.rs

#![allow(unused_imports)]

use crate::{_reexport, _tags};

/* core */

_reexport! { rust: core::str, location: "text/str", tag: _tags!(text lifetime),
    doc: "Parse a value from a string.",
    FromStr
}

/* alloc */

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

/* std */

_reexport! { rust: std::ffi, location: "text/str", tag: _tags!(text lifetime platform),
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
_reexport! { rust: std::ffi, location: "text/str", tag: _tags!(text platform),
    doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
    OsString
}
