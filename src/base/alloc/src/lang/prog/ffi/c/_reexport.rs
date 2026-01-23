// devela_base_alloc::lang::prog::ffi::c::_reexport
//
//!
//

use crate::{_TAG_TEXT, _reexport};

_reexport! { rust: alloc::ffi, location: "lang::prog::ffi::c", tag: _TAG_TEXT!(),
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
