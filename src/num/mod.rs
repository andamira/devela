// devela::num
//
//! Numerical types and operations, algebra, <small>extends
//! `std::{`[`cmp`], [`num`]`}`.</small>
//!
//! [`cmp`]: std::cmp
//! [`num`]: std::num
//

// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private;
#[allow(unused_imports)]
pub(crate) use _private::*;

mod alias;
mod cmp;
mod error;
mod float;
mod no;
mod primitive;
mod sign;
mod r#trait;
mod unit;
pub use {alias::*, cmp::*, error::*, float::*, no::*, primitive::*, r#trait::*, sign::*, unit::*};

pub mod algebra;
pub mod niche;
#[doc(no_inline)]
pub use {algebra::all::*, niche::all::*};

#[cfg(_some_int)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_int)))]
mod frac;
#[cfg(_some_int)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_int)))]
mod int;
#[cfg(_some_int)]
pub use {frac::*, int::*};

#[cfg(feature = "num_rand")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_rand")))]
pub mod rand;
#[doc(no_inline)]
#[cfg(feature = "num_rand")]
pub use rand::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        algebra::all::*, alias::*, cmp::all::*, error::*, float::*, niche::all::*, no::*,
        primitive::*, r#trait::*, sign::*, unit::*,
    };

    #[doc(inline)]
    #[cfg(_some_int)]
    #[allow(unused_imports)]
    pub use super::{frac::*, int::*};

    #[doc(inline)]
    #[cfg(feature = "num_rand")]
    #[allow(unused_imports)]
    pub use super::rand::all::*;
}
