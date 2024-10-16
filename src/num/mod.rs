// devela::num
//
//! Numerical types and math operations.
#![doc = crate::code::doc_!(extends: cmp, num)]
#![doc = crate::code::doc_!(modules: crate; num: algebra, logic, niche, rand)]
#![doc = crate::code::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

#[allow(unused_imports)]
use crate::code::items;

mod _private;
#[allow(unused_imports)]
pub(crate) use _private::*;

mod alias;
mod cmp;
mod error;
mod float;
mod interval;
mod no;
mod primitive;
mod sign;
mod unit;
mod r#trait;
pub use {
    alias::*, cmp::*, error::*, float::*, interval::*, no::*, primitive::*, r#trait::*, sign::*,
    unit::*,
};

pub mod algebra;
pub mod logic;
pub mod niche;
#[doc(no_inline)]
pub use {algebra::all::*, logic::all::*, niche::all::*};

#[cfg(_some_int)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_some_int)))]
items! {
    mod frac;
    mod int;
    pub use {frac::*, int::*};
}

#[cfg(feature = "num_rand")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num_rand")))]
items! {
    pub mod rand;
    #[doc(no_inline)]
    pub use rand::all::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        algebra::all::*, alias::*, cmp::all::*, error::*, float::*, interval::*, logic::all::*,
        niche::all::*, no::*, primitive::*, r#trait::*, sign::*, unit::*,
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
