// devela_base_core::lang::prog::ffi::c::_reexport
//
//!
//

use crate::{_reexport, _tags};

_reexport! { rust: core::ffi, location: "lang::prog::ffi::c", tag: _tags!(primitive text),
    doc: "Equivalent to the corresponding C type.",
    c_char, c_schar, c_uchar,
}
_reexport! { rust: core::ffi, location: "lang::prog::ffi::c", tag: _tags!(primitive num),
    doc: "Equivalent to the corresponding C type.",
    c_double, c_float, c_int, c_long, c_longlong, c_short,
    c_uint, c_ulong, c_ulonglong, c_ushort,
}

_reexport! { rust: core::ffi, location: "lang::prog::ffi::c", tag: _tags!(primitive),
    doc: "Equivalent to C's void type when used as a [pointer].",
    c_void
}

_reexport! { rust: core::ffi, location: "lang::prog::ffi::c", tag: _tags!(text lifetime),
    doc: "Representation of a borrowed C string (See `CString`).",
    CStr
}
