// devela::text::char::reexports

use crate::reexport;

reexport! { rust: core::str,
    doc: "An iterator over the [`char`]s of a string slice.",
    @Chars as IterChars
}
