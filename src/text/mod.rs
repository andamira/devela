// devela::text
//
//! Text types and operations, text processing.
#![doc = crate::doc_!(extends: ascii, char, fmt, str, string)]
#![doc = crate::doc_!(modules: crate; text: fmt)]
#![doc = crate::doc_!(newline)]
//!
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
mod reexports;
#[allow(unused_imports)]
pub use {ascii::all::*, char::all::*, error::*, ext::*, grapheme::all::*, reexports::*};

pub mod fmt;
#[doc(no_inline)]
pub use fmt::all::*;

#[cfg(_string_u_·)]
items! {
    mod string_u;
    pub use string_u::*;
}

#[cfg(feature = "_string_nonul")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
items! {
    mod nonul;
    pub use nonul::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, grapheme::all::*, reexports::*,
    };

    #[cfg(_string_u_·)]
    pub use super::string_u::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")]
    pub use super::nonul::*;
}
