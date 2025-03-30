// devela::text::parse::reexports
//
//!
//

use crate::{TAG_ERROR, TAG_NUM, TAG_TEXT, reexport};

reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "An error which can be returned when parsing an integer.",
    ParseIntError
}
reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "An error which can be returned when parsing an float.",
    ParseFloatError
}
reexport! { rust: core::num,
    tag: concat![TAG_TEXT!(), TAG_NUM!(), TAG_ERROR!()],
    doc: "Kinds of errors that can cause parsing an integer to fail.",
    @IntErrorKind as ParseIntErrorKind
}

// NOTE: Utf8Error not re-exported. See `InvalidUtf8` instead.
// reexport! { rust: core::str,
//     tag: crate::TAG_ERROR!(),
//     doc: "An error which can occur when interpreting a sequence of `u8` as a string.",
//     Utf8Error
// }
