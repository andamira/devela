// devela::num
//
//! Numerical types and computations, arithmetic operations, extends
//! `std::`[`num`].
//!
//! [`num`]: std::num
//

// warnings:
#![cfg_attr(not(feature = "num"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

/* always compiled, crate-private modules */

mod _private;
pub(crate) use _private::*;

/* always compiled, non-public modules */

mod alias;
mod always_fns;
mod error;
mod float;
mod primitive;
mod sign;

pub use {alias::*, always_fns::*, error::*, float::*, primitive::*, sign::*};

/* always compiled, public modules */

pub mod niche;

#[doc(no_inline)]
pub use niche::all::*;

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
