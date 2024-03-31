// devela::sys::ffi::reexports
//
//! Reexported items from `core`.
//

use crate::code::reexport;

reexport! { rust: core::ffi,
    doc: "Representation of a borrowed C string.",
    CStr
}
reexport! { rust: core::ffi,
    doc: "Equivalent to the corresponding C type.",
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
    c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void
}

reexport! { rust: alloc::ffi,
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
