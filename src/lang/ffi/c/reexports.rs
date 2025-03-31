// devela::lang::ffi::c::reexports
//
//! Reexported items from `core`.
//

use crate::{TAG_NUM, TAG_PRIMITIVE, TAG_TEXT, reexport};

reexport! { rust: core::ffi,
    tag: concat![TAG_PRIMITIVE!(), TAG_TEXT!()],
    doc: "Equivalent to the corresponding C type.",
    c_char, c_schar, c_uchar,
}
reexport! { rust: core::ffi,
    tag: concat![TAG_PRIMITIVE!(), TAG_NUM!()],
    doc: "Equivalent to the corresponding C type.",
    c_double, c_float, c_int, c_long, c_longlong, c_short,
    c_uint, c_ulong, c_ulonglong, c_ushort,
}

reexport! { rust: core::ffi,
    tag: TAG_PRIMITIVE!(),
    doc: "Equivalent to C’s void type when used as a [pointer].",
    c_void
}

reexport! { rust: core::ffi,
    tag: TAG_TEXT!(),
    doc: "Representation of a borrowed C string (See [`CString`]).",
    CStr
}

reexport! { rust: alloc::ffi,
    tag: TAG_TEXT!(),
    doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
    CString
}
