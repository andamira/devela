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

/* modules */

// always compiled, public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod ascii;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod char;

// always compiled, non-public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod reexports;

// feature-gated, public
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod egc;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
pub mod fmt;

// feature-gated, non-public
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
mod helpers;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod non_nul;

/* re-exports */

// always compiled, public
#[doc(no_inline)]
#[allow(unused_imports)]
pub use {ascii::all::*, char::all::*};

// always compiled, non-public
#[allow(unused_imports)]
pub use reexports::*;

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "text")]
pub use {egc::all::*, fmt::all::*};

// feature-gated, private
#[cfg(feature = "text")]
pub use {array_string::*, error::*, ext::*, non_nul::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ascii::all::*, char::all::*, reexports::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{array_string::*, egc::all::*, error::*, ext::*, fmt::all::*, non_nul::*};
}
