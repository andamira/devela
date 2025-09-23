// devela_base_core::text::parse::reexports
//
//!
//

use crate::{_TAG_ERROR, _TAG_NUM, _TAG_TEXT, _reexport};

_reexport! { rust: core::num,
    tag: _TAG_TEXT!() _TAG_NUM!() _TAG_ERROR!(),
    doc: "An error which can be returned when parsing an integer.",
    ParseIntError
}
_reexport! { rust: core::num,
    tag: _TAG_TEXT!() _TAG_NUM!() _TAG_ERROR!(),
    doc: "An error which can be returned when parsing an float.",
    ParseFloatError
}
_reexport! { rust: core::num,
    tag: _TAG_TEXT!() _TAG_NUM!() _TAG_ERROR!(),
    doc: "Kinds of errors that can cause parsing an integer to fail.",
    @IntErrorKind as ParseIntErrorKind
}

// NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
// _reexport! { rust: core::str,
//     tag: crate::_TAG_ERROR!(),
//     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
//     Utf8Error
// }
