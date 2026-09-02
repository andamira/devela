// devela/src/lang/prog/ffi/c/_reexport_alloc.rs

use crate::{_TAG_TEXT, _reexport};

_reexport! { rust: alloc::ffi,
    location: "lang::prog::ffi::c" => struct CString, tag: _TAG_TEXT!(),
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
