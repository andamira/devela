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
mod error;
mod float;
mod frac;
mod int;
mod no;
mod primitive;
mod sign;
mod r#trait;

pub use {alias::*, error::*, float::*, frac::*, int::*, no::*, primitive::*, r#trait::*, sign::*};

/* always compiled, public modules */

pub mod niche;
pub mod rand;

#[doc(no_inline)]
pub use {niche::all::*, rand::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        alias::*, error::*, float::*, niche::all::*, primitive::*, rand::all::*, sign::*,
    };
    #[doc(inline)]
    pub use super::{frac::*, int::*, no::*, r#trait::*};
}
