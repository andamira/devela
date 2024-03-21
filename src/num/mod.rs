// devela::num
//
//! Numerical types and computations, arithmetic operations, extends
//! `std::{`[`cmp`], [`num`]`}`.
//!
//! [`cmp`]: std::cmp
//! [`num`]: std::num
//

// warnings:
#![cfg_attr(not(feature = "num"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

/* always compiled */

mod _private;
pub(crate) use _private::*;

mod alias;
mod cmp;
mod error;
mod gcd;
mod no;
mod primitive;
mod sign;
mod r#trait;
pub use {alias::*, cmp::all::*, error::*, gcd::*, no::*, primitive::*, r#trait::*, sign::*};

pub mod niche;
pub mod rand;
#[doc(no_inline)]
pub use {niche::all::*, rand::all::*};

/* feature-gated */

#[cfg(feature = "num_float")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_float")))]
mod float;
#[cfg(feature = "num_int")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_int")))]
mod frac;
#[cfg(feature = "num_int")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_int")))]
mod int;

#[cfg(feature = "num_float")]
pub use float::*;
#[cfg(feature = "num_int")]
pub use {frac::*, int::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        alias::*, cmp::all::*, error::*, gcd::*, niche::all::*, no::*, primitive::*, r#trait::*,
        rand::all::*, sign::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "num_float")]
    pub use super::float::*;
    #[doc(inline)]
    #[cfg(feature = "num_int")]
    pub use super::{frac::*, int::*};
}
