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

#[allow(hidden_glob_reexports, reason = "re-exported `char`")]
mod char;
pub mod errors {
    //! Text-related errors.
    #[doc(inline)]
    pub use devela_base::text::errors::*;
}
mod grapheme; // Grapheme

pub mod fmt;
pub mod parse;
pub mod str;

// WIPZONE
// mod bytes; // Utf8Byte
// mod cell; // TextCell

crate::structural_mods! { // _mods, _pub_mods, _always
    _mods {
        pub use super::{
            char::_all::*,
            grapheme::_all::*,
        };

        #[doc(inline)]
        pub use devela_base::text::{Ascii, AsciiChar}; // ascii

        // WIPZONE
        // pub use super::bytes::*;
        // pub use super::cell::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{
            fmt::_all::*,
            parse::_all::*,
            str::_all::*,
        };
    }
    _always {
        #[doc(inline)]
        pub use super::{
            errors::*, fmt::_always::*, str::_always::*,
        };
    }
}
