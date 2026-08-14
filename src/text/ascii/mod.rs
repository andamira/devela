// devela/src/text/ascii/mod.rs
//
#![doc = crate::_DOC_TEXT_ASCII!()] // public
#![doc = crate::_doc!(modules: crate::text; ascii)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: ascii)]

mod char; // CharAscii
mod digits; // Digits
mod namespace; // Ascii
mod set; // AsciiSet

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::CharAscii,
            digits::_all::Digits,
            namespace::Ascii,
            set::AsciiSet,
        };
    }
}
