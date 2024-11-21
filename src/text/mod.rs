// devela::text
//
//! Text types and operations, text processing.
#![doc = crate::doc_!(modules: crate; text: fmt)]
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
mod char;
mod error;
mod ext;
mod grapheme;
mod str;
#[allow(unused_imports)]
pub use {ascii::all::*, char::all::*, error::*, ext::*, grapheme::all::*, str::all::*};

pub mod fmt;
#[doc(no_inline)]
pub use fmt::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, grapheme::all::*, str::all::*,
    };
}
