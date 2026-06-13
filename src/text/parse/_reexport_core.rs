// devela/src/text/parse/_reexport_core.rs

use crate::{_reexport, _tags};

_reexport! { rust: core::num, location: "text/parse", tag: _tags!(text parser num error),
    doc: "An error which can be returned when parsing an integer.",
    ParseIntError
}
_reexport! { rust: core::num, location: "text/parse", tag: _tags!(text parser num error),
    doc: "An error which can be returned when parsing an float.",
    ParseFloatError
}
_reexport! { rust: core::num, location: "text/parse", tag: _tags!(text parser num error),
    doc: "Kinds of errors that can cause parsing an integer to fail.",
    @IntErrorKind as ParseIntErrorKind
}

// NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
// _reexport! { rust: core::str, location: "text/parse", tag: _tags!(text parser error),
//     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
//     Utf8Error
// }
