// devela::text::reexports
//
//!
//

use crate::reexport;

// #[doc(inline)] // FIX: doesn't show inline :(
// pub use core::primitive::char;

reexport! { rust: core::str,
    doc: "An iterator over the [`char`]s of a string slice.",
    @Chars as IterChars
}
