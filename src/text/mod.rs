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
mod error;
mod grapheme;
mod parse;

pub mod fmt;
pub mod str;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {doc_hidden::*, doc_inline::*};

    mod doc_inline {
        pub use super::{ascii::all::*, char::all::*, error::*, grapheme::all::*, parse::*};
    }
    mod doc_hidden { #[doc(hidden)] #[doc(no_inline)]
        pub use super::{fmt::all::*, str::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
}
