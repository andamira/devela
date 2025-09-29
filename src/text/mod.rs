// devela::text
//
#![doc = crate::_DOC_TEXT!()]
#![doc = crate::_doc!(modules: crate; text: fmt, parse, str)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

pub mod errors {
    //! Text-related errors.
    #[doc(inline)]
    pub use devela_base_core::text::errors::*;
}
mod grapheme; // Grapheme

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
pub mod char;
pub mod fmt;
pub mod parse;
pub mod str;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            grapheme::_all::*,
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
