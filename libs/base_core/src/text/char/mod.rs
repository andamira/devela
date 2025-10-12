// devela_base_core::text::char
//
#![doc = crate::_DOC_TEXT_CHAR!()]
//

mod ascii; // CharAscii
mod digits; // Digits
mod iter; // CharIter
mod luts; // LUT_ASCII_CHARS, LUT_DIGITS_BASE36, LUT_POWERS10
mod namespace; // Char
mod scalar; // char7, char8, char16, char_utf8

mod reexports;

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ascii::*,
            digits::*,
            iter::*,
            luts::*,
            namespace::*,
            reexports::*,
            scalar::*,
        };
    }
}
