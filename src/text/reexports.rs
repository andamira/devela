// devela::text::reexports
//
//!
//

use crate::code::reexport;

/* core */

reexport! { rust: core::str,
    doc: "An iterator over the [`char`]s of a string slice.",
    @Chars as IterChars
}
// TODO: IMPROVE: recreate and impl conversion methods:
// - https://doc.rust-lang.org/src/core/str/error.rs.html#47-50
reexport! { rust: core::str,
    doc: "Errors which can occur when attempting to interpret a sequence of u8 as a string.",
    Utf8Error
}
