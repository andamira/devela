// devela_base_std::text::str:reexports
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_TAG_LIFETIME, _TAG_TEXT, _reexport};

_reexport! { rust: std::ffi,
    tag: _TAG_TEXT!() _TAG_LIFETIME!(),
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
_reexport! { rust: std::ffi,
    tag: _TAG_TEXT!(),
    doc: "A type for owned, mutable native strings, interconvertible with Rust strings.",
    OsString
}
