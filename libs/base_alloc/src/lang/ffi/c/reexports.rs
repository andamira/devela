// devela_base_alloc::lang::ffi::c::reexports
//
//!
//

use crate::{_reexport, TAG_TEXT};

_reexport! { rust: alloc::ffi,
    tag: TAG_TEXT!(),
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
