// devela::text::parse::reexports
//
//!
//

use crate::reexport;

reexport! { rust: core::num,
    tag: crate::TAG_ERROR!(),
    doc: "An error which can be returned when parsing an integer.",
    ParseIntError
}
reexport! { rust: core::num,
    tag: crate::TAG_ERROR!(),
    doc: "An error which can be returned when parsing an float.",
    ParseFloatError
}
reexport! { rust: core::num,
    tag: crate::TAG_ERROR_COMPOSITE!(),
    doc: "Kinds of errors that can cause parsing an integer to fail.",
    @IntErrorKind as ParseIntErrorKind
}

// NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
// reexport! { rust: core::str,
//     tag: crate::TAG_ERROR!(),
//     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
//     Utf8Error
// }
