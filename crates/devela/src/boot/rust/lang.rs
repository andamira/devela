use crate::{_reexport, _tags};

pub(crate) use prog_ffi_c::*;

pub(crate) mod prog_ffi_c {
    use super::*;

    /* structs */

    _reexport! { rust: core::ffi,
        location: "lang/prog/ffi/c" => struct CStr, tag: _tags!(text lifetime),
        doc: "Representation of a borrowed C string (See `CString`).",
        CStr
    }
    _reexport! { rust: alloc::ffi,
        location: "lang/prog/ffi/c" => struct CString, tag: _tags!(text),
        doc: "An owned, C-compatible, nul-terminated string with no nul bytes in the middle.",
        CString
    }

    /* enums */

    _reexport! { rust: core::ffi,
        location: "lang/prog/ffi/c" => enum c_void, tag: _tags!(primitive),
        doc: "Equivalent to C's void type when used as a [pointer].",
        c_void
    }

    /* type aliases */

    _reexport! { rust: core::ffi,
        location: "lang/prog/ffi/c", tag: _tags!(primitive text),
        doc: "Equivalent to the corresponding C type.",
        c_char, c_schar, c_uchar,
    }
    _reexport! { rust: core::ffi,
        location: "lang/prog/ffi/c", tag: _tags!(primitive num),
        doc: "Equivalent to the corresponding C type.",
        c_double, c_float, c_int, c_long, c_longlong, c_short,
        c_uint, c_ulong, c_ulonglong, c_ushort,
    }
}
