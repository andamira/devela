// devela::text
//
//! Text types and operations, text processing.
#![doc = crate::code::doc_extends!(ascii, char, fmt, str, string)]
//!
//

#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

pub(crate) mod helpers;

mod ascii;
mod char;
mod error;
mod ext;
mod grapheme;
mod fmt;
mod reexports;
pub use {
    ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, grapheme::all::*, reexports::*,
};

#[cfg(_some_string_u)]
mod string_u;
#[cfg(_some_string_u)]
pub use string_u::*;

#[cfg(feature = "_string_nonul")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
mod nonul;
#[cfg(feature = "_string_nonul")]
pub use nonul::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, grapheme::all::*, reexports::*,
    };

    #[cfg(_some_string_u)]
    pub use super::string_u::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")]
    pub use super::nonul::*;
}
