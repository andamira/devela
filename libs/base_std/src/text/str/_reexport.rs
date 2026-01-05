// devela_base_std::text::str:_reexport
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_TAG_LIFETIME, _TAG_PLATFORM, _TAG_TEXT, _reexport};

_reexport! { rust: std::ffi, location: "text/str",
    tag: _TAG_TEXT!() _TAG_LIFETIME!() _TAG_PLATFORM!(),
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
_reexport! { rust: std::ffi, location: "text/str", tag: _TAG_TEXT!() _TAG_PLATFORM!(),
    doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
    OsString
}
