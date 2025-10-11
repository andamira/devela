// devela_base_core::text::char::ascii
//
#![doc = crate::_DOC_TEXT_ASCII!()]
#![doc = crate::_doc!(extends: ascii)]
#![doc = crate::_doc!(modules: crate::text; ascii)]
#![doc = crate::_doc!(newline)]
//!
//

mod char; // CharAscii
mod digits; // AsciiDigits
mod luts; // LUT_ASCII_CHARS, LUT_DIGITS_BASE36, LUT_POWERS10

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::*,
            digits::*,
            luts::*,
        };
    }
}
