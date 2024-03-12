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
mod float;
mod frac;
mod int;
mod no;
mod primitive;
mod sign;
mod r#trait;

pub use {
    alias::*, cmp::all::*, error::*, float::*, frac::*, int::*, no::*, primitive::*, r#trait::*,
    sign::*,
};

pub mod niche;
pub mod rand;

#[doc(no_inline)]
pub use {niche::all::*, rand::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        alias::*, cmp::all::*, error::*, float::*, frac::*, int::*, niche::all::*, no::*,
        primitive::*, r#trait::*, rand::all::*, sign::*,
    };
}
