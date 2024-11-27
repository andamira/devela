// devela::num
//
//! Numerical types and math operations.
#![doc = crate::doc_!(modules: crate; num: algebra, logic, niche, rand, wave)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: cmp, num)]
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

#[cfg(_int_·)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_int_·)))]
items! {
    mod frac;
    mod int;
    pub use {frac::*, int::*};
}

#[cfg(feature = "geom")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
    pub mod geom;
    #[doc(no_inline)]
    pub use geom::all::*;
}

#[cfg(feature = "rand")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
    pub mod rand;
    #[doc(no_inline)]
    pub use rand::all::*;
}

#[cfg(feature = "wave")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "wave")))]
    pub mod wave;
    #[doc(no_inline)]
    pub use wave::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{
        algebra::all::*, alias::*, cmp::all::*, error::*, float::*, interval::*, logic::all::*,
        niche::all::*, no::*, primitive::*, r#trait::*, sign::*, unit::*,
    };

    #[doc(inline)]
    #[cfg(_int_·)]
    pub use super::{frac::*, int::*};

    #[doc(inline)]
    #[cfg(feature = "geom")]
    pub use super::geom::all::*;

    #[doc(inline)]
    #[cfg(feature = "rand")]
    pub use super::rand::all::*;

    #[doc(inline)]
    #[cfg(feature = "wave")]
    pub use super::wave::*;
}
