// devela_base_core::text
//
#![doc = crate::_DOC_TEXT!()]
//
// safety
#![cfg_attr(base_safe_text, forbid(unsafe_code))]

// mod _wip_cell; // WIP TextCell
mod grapheme; // GraphemeU8

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char; // Char, CharAscii, IterChars, UnicodeScalar, char[7|8|16|utf8]
pub mod errors;
pub mod fmt;
pub mod parse;
pub mod str;

crate::structural_mods! { // mods, _pub_mods
    _mods {
        pub use super::{
            grapheme::_all::*,
            // _wip_cell::*, // WIP
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{
            char::_all::*,
            fmt::_all::*,
            parse::_all::*,
            str::_all::*,
        };
    }
}
