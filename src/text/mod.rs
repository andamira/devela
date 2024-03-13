// devela::text
//
//! Text manipulation, extends
//! `std::{`[`ascii`], [`char`], [`fmt`], [`str`], [`string`]`}`.
//!
//! [`ascii`]: std::ascii
//! [`char`]: std::char
//! [`fmt`]: std::fmt
//! [`str`]: std::str
//! [`string`]: std::string
//

// safety:
#![cfg_attr(feature = "safe_text", forbid(unsafe_code))]

/* always compiled */

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

/* feature-gated, public modules */

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod egc;

#[doc(no_inline)]
#[cfg(feature = "text")]
pub use egc::all::*;

/* feature-gated, non-public modules */

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod nonul;

#[cfg(feature = "text")]
pub use nonul::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ascii::all::*, char::all::*, error::*, ext::*, fmt::all::*, reexports::*, string_u::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{egc::all::*, nonul::*};
}
