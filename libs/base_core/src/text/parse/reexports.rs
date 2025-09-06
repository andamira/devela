// devela_base_core::text::parse::reexports
//
//!
//

use crate::{_reexport, TAG_ERROR, TAG_NUM, TAG_TEXT};

_reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "An error which can be returned when parsing an integer.",
    ParseIntError
}
_reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "An error which can be returned when parsing an float.",
    ParseFloatError
}
_reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "Kinds of errors that can cause parsing an integer to fail.",
    @IntErrorKind as ParseIntErrorKind
}

// NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
// _reexport! { rust: core::str,
//     tag: crate::TAG_ERROR!(),
//     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
//     Utf8Error
// }
