// devela::text::str:_reexport_std

#![allow(unused_imports)]

use crate::{_reexport, _tags};

_reexport! { rust: std::ffi, location: "text/str", tag: _tags!(text lifetime platform),
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
_reexport! { rust: std::ffi, location: "text/str", tag: _tags!(text platform),
    doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
    OsString
}
