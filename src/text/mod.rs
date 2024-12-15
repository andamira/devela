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

mod helpers; // impl_sized_alias!

mod ascii;
#[allow(hidden_glob_reexports, reason = "re-exported char")]
mod char;
mod grapheme; // Grapheme
mod parse;

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod error;

pub mod fmt;
pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use {_pub_mods::*, _mods::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{ascii::_all::*, char::_all::*, grapheme::_all::*, parse::_all::*};
        #[cfg(feature = "text")]
        pub use super::error::*;
    }
    mod _pub_mods {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{fmt::_all::*, str::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_pub_mods::*, _mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            fmt::_always::*, char::_always::*, parse::_always::*, str::_always::*,
        };
    }
}
