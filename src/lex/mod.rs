// devela::lex
//
//! Lexical types and operations, text processing, <small>extends
//! `std::{`[`ascii`], [`char`], [`fmt`], [`str`], [`string`]`}`.</small>
//!
//! [`ascii`]: std::ascii
//! [`char`]: std::char
//! [`fmt`]: std::fmt
//! [`str`]: std::str
//! [`string`]: std::string
//

#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_lex", forbid(unsafe_code))]

pub(crate) mod helpers;

mod ascii;
mod char;
mod egc;
mod error;
mod ext;
mod fmt;
mod reexports;
pub use {ascii::all::*, char::all::*, egc::all::*, error::*, ext::*, fmt::all::*, reexports::*};

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
        ascii::all::*, char::all::*, egc::all::*, error::*, ext::*, fmt::all::*, reexports::*,
    };

    #[cfg(_some_string_u)]
    pub use super::string_u::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")]
    pub use super::nonul::*;
}
