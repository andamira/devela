// devela_base_core::text::ascii
//
#![doc = crate::_DOC_TEXT_ASCII!()]
#![doc = crate::_doc!(extends: ascii)]
#![doc = crate::_doc!(modules: crate::text; ascii)]
#![doc = crate::_doc!(newline)]
//!
//

mod char; // AsciiChar
mod wrapper; // Ascii

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::AsciiChar,
            wrapper::Ascii,
        };
    }
}
