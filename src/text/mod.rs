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

crate::items! { // structural access: doc_inline, doc_hidden, all, always
    #[allow(unused)]
    pub use {doc_hidden::*, doc_inline::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{ascii::all::*, char::all::*, grapheme::all::*, parse::all::*};
        #[cfg(feature = "text")]
        pub use super::error::*;
    }
    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{fmt::all::*, str::all::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{
            fmt::always::*, char::always::*, parse::always::*, str::always::*,
        };
    }
}
