// devela::text
//
//! Text types and operations, text processing.
#![doc = crate::doc_!(modules: crate; text: fmt, str)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ascii, char, fmt, str, string)]
//

// safety:
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

mod helpers; // impl_sized_alias!

mod ascii;
#[allow(hidden_glob_reexports, reason = "re-exported char")]
mod char;
mod error;
mod grapheme;
mod reexports;
#[allow(unused_imports)]
pub use {ascii::all::*, char::all::*, error::*, grapheme::all::*, reexports::*};

pub mod fmt;
pub mod str;
#[doc(no_inline)]
pub use {fmt::all::*, str::all::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, fmt::all::*, grapheme::all::*, reexports::*,
        str::all::*,
    };
}
