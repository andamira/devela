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

/* always compiled, public modules */

pub mod char;
pub mod fmt;

#[doc(no_inline)]
#[allow(unused_imports)]
pub use {char::all::*, fmt::all::*};

/* always compiled, non-public moduels */

#[allow(unused_imports)]
pub use {ascii::all::*, reexports::*};

mod ascii;
mod reexports;

/* feature-gated, public modules */
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod egc;

#[doc(no_inline)]
#[cfg(feature = "text")]
pub use egc::all::*;

/* feature-gated, non-public modules */

// crate-private
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod helpers; // impl_sized_alias (RETHINK)

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod array_string;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod error;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod ext;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod non_nul;

#[cfg(feature = "text")]
pub use {array_string::*, error::*, ext::*, non_nul::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ascii::all::*, char::all::*, fmt::all::*, reexports::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{array_string::*, egc::all::*, error::*, ext::*, non_nul::*};
}
