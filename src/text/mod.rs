// devela::text
//
//! Text types and operations, text processing.
#![doc = crate::doc_!(modules: crate; text: fmt, str)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ascii, char, fmt, str, string)]
//
// safety
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

mod ascii;
#[allow(hidden_glob_reexports, reason = "re-exported char")]
mod char;
mod error;
mod grapheme; // Grapheme
mod parse;

pub mod fmt;
pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods {
        pub use super::{ascii::_all::*, char::_all::*, grapheme::_all::*, parse::_all::*};
        pub use super::error::*;
    }
    mod _pub_mods {
        pub use super::{fmt::_all::*, str::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            fmt::_always::*, char::_always::*, parse::_always::*, str::_always::*,
        };
    }
}
