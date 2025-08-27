// devela_base::lang::ffi::c::reexports
//
//!
//

use crate::{_reexport, TAG_NUM, TAG_PRIMITIVE, TAG_TEXT};

_reexport! { rust: core::ffi,
    tag: concat![TAG_PRIMITIVE!(), TAG_TEXT!()],
    doc: "Equivalent to the corresponding C type.",
    c_char, c_schar, c_uchar,
}
_reexport! { rust: core::ffi,
    tag: concat![TAG_PRIMITIVE!(), TAG_NUM!()],
    doc: "Equivalent to the corresponding C type.",
    c_double, c_float, c_int, c_long, c_longlong, c_short,
    c_uint, c_ulong, c_ulonglong, c_ushort,
}

_reexport! { rust: core::ffi,
    tag: TAG_PRIMITIVE!(),
    doc: "Equivalent to C’s void type when used as a [pointer].",
    c_void
}

_reexport! { rust: core::ffi,
    tag: TAG_TEXT!(),
    doc: "Representation of a borrowed C string (See `CString`).",
    CStr
}
