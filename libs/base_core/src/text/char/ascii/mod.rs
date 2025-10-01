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
mod lut; // CHAR_ASCII

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::CharAscii,
            digits::AsciiDigits,
            lut::CHAR_ASCII,
        };
    }
}
