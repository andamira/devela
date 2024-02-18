// devela::num
//
//! Numeric types and operations, extends
//! `std::`[`num`].
//!
//! [`num`]: std::num
//

// warnings:
#![cfg_attr(not(feature = "num"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

/* modules */

// always compiled, crate-private
mod _private;

// always compiled, non-public
mod alias;
mod always_fns;
mod error;
mod float;
mod primitive;
mod sign;

// always compiled, public
pub mod niche;

// feature gated, private
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod frac;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod int;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod no;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod r#trait;

/* re-exports */

// always compiled, crate-private
pub(crate) use _private::*;

// always compiled, non-public
pub use {alias::*, always_fns::*, error::*, float::*, primitive::*, sign::*};

// always compiled, public
#[doc(no_inline)]
pub use niche::all::*;

// feature-gated, private
#[cfg(feature = "num")]
pub use {frac::*, int::*, no::*, r#trait::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        alias::*, always_fns::*, error::*, float::*, niche::all::*, primitive::*, sign::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{frac::*, int::*, no::*, r#trait::*};
}
