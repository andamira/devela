// devela_base_core::text
//
#![doc = crate::_DOC_TEXT!()]
#![doc = crate::_doc!(modules: crate; text: char, errors, fmt, grapheme, parse, str)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(base_safe_text, forbid(unsafe_code))]

// mod _wip_cell;
// mod _wip_draw;

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char; // Char, CharAscii, CharIter, UnicodeScalar, char[7|8|16|utf8]
pub mod errors;
pub mod fmt;
pub mod parse;
pub mod str;

crate::structural_mods! { // mods, _pub_mods
    _mods {
        // pub use super::_wip_cell::*;
        // pub use super::_wip_draw::_all::*;
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
