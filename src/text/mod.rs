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

mod ascii;
#[allow(hidden_glob_reexports, reason = "re-exported char")]
mod char;
pub mod errors {
    //! Text-related errors.
    pub use devela_base::text::errors::*;
}
mod grapheme; // Grapheme

pub mod fmt;
pub mod parse;
pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods {
        pub use super::{ascii::_all::*, char::_all::*, grapheme::_all::*};
        // WIPZONE
        // pub use super::bytes::*;
        // pub use super::cell::*;
    }
    mod _pub_mods {
        #[doc(inline)]
        pub use super::{errors::*, fmt::_all::*, parse::_all::*, str::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        #[doc(inline)]
        pub use super::{
            errors::*, fmt::_always::*, char::_always::*, parse::_always::*, str::_always::*,
        };
    }
}
// WIPZONE
// mod bytes; // Utf8Byte
// mod cell; // TextCell
