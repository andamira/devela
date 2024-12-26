// devela::ffi::reexports
//
//! Reexported items from `core`.
//

use crate::reexport;

reexport! { rust: core::ffi,
    tag: crate::TAG_PRIMITIVE!(),
    doc: "Equivalent to the corresponding C type.",
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
    c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void
}

reexport! { rust: core::ffi,
    doc: "Representation of a borrowed C string (See [`CString`]).",
    CStr
}

reexport! { rust: alloc::ffi,
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}

reexport! { rust: std::ffi,
    doc: "Borrowed reference to an OS string (See [`OsString`]).",
    OsStr
}
reexport! { rust: std::ffi,
    doc: "A type that can represent owned, mutable platform-native strings,
    but is cheaply inter-convertible with Rust strings.",
    OsString
}
