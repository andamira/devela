// devela::num
//
//! Numeric types and operations, extends
//! `std::{`[`num`], [`ops`]`}`.
//!
//! [`num`]: core::num
//! [`ops`]: core::ops
//

#![cfg_attr(not(feature = "num"), allow(unused_imports))]

/* modules */

// always compiled, crate-private
mod _private;

// always compiled, non-public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod alias;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod error;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod float;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod primitive;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod sign;

// always compiled, public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
pub mod niche;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
pub mod ops;

// feature gated, private
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod frac;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod int;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod no;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "num")))]
mod r#trait;

/* re-exports */

// always compiled, crate-private
pub(crate) use _private::*;

// always compiled, non-public
pub use {alias::*, error::*, float::*, primitive::*, sign::*};

// always compiled, public
#[doc(no_inline)]
pub use {niche::all::*, ops::all::*};

// feature-gated, private
#[cfg(feature = "num")]
pub use {frac::*, int::*, no::*, r#trait::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        alias::*, error::*, float::*, niche::all::*, ops::all::*, primitive::*, sign::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{frac::*, int::*, no::*, r#trait::*};
}
