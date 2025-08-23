// devela_base_std::text::str:reexports
//
//! String related re-exports.
//

#![allow(unused_imports)]

use crate::{_reexport, TAG_TEXT};

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
