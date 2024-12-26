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

// TODO: IMPROVE: recreate and impl conversion methods:
// - https://doc.rust-lang.org/src/core/str/error.rs.html#47-50
reexport! { rust: core::str,
    tag: crate::TAG_ERROR!(),
    doc: "Errors which can occur when attempting to interpret a sequence of u8 as a string.",
    Utf8Error
}
