// devela_base_alloc::lang::ffi::c::reexports
//
//!
//

use crate::{_TAG_TEXT, _reexport};

_reexport! { rust: alloc::ffi,
    tag: _TAG_TEXT!(),
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
