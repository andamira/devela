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

// safety:
#![cfg_attr(feature = "safe_lex", forbid(unsafe_code))]

mod ascii;
mod char;
mod error;
mod ext;
mod fmt;
mod helpers;
mod reexports;
mod string_u;
#[allow(unused_imports)]
pub use {ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, reexports::*, string_u::*};

#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod egc;
#[cfg(feature = "lex")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "lex")))]
mod nonul;
#[cfg(feature = "lex")]
pub use {egc::all::*, nonul::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, reexports::*, string_u::*,
    };

    #[doc(inline)]
    #[cfg(feature = "lex")]
    #[allow(unused_imports)]
    pub use super::{egc::all::*, nonul::*};
}
