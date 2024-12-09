// devela::num
//
//! Numerical types and math operations.
#![doc = crate::doc_!(modules: crate; num: algebra, logic, niche, rand, wave)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: cmp, num)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private; // upcasted_op!, impl_ops!

mod cmp; // Compare
mod error; // NumError, NumResult
mod float; // fsize, ExtFloat, ExtFloatConst, Float
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
mod interval; // Interval
mod no; // NoNum
mod primitive; // Cast, Primitive[Cast|Join|Split]
mod sign; // Sign
mod traits; // Num, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod logic;
pub mod niche;

#[cfg(feature = "alg")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alg")))]
pub mod alg;
#[cfg(feature = "geom")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "geom")))]
pub mod geom;
#[cfg(feature = "rand")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rand")))]
pub mod rand;
#[cfg(feature = "wave")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "wave")))]
pub mod wave;

crate::items! { // structural access: doc_inline, doc_hidden, items_private, all, always
    #[allow(unused)]
    pub use {doc_hidden::*, doc_inline::*, items_private::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline { #![allow(unused)]
        pub use super::{
            cmp::all::*, error::*, float::all::*, frac::all::*, int::all::*,
            interval::*, no::*, primitive::all::*, sign::*, traits::*,
        };
        #[cfg(feature = "geom")]
        pub use super::geom::all::*;
        #[cfg(feature = "rand")]
        pub use super::rand::all::*;
        #[cfg(feature = "unit")]
        pub use super::unit::all::*;
        #[cfg(feature = "wave")]
        pub use super::wave::all::*;
    }
    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{logic::all::*, niche::all::*};

        #[doc(hidden)] #[doc(no_inline)]
        #[cfg(feature = "alg")]
        pub use super::alg::all::*;
    }
    pub(super) mod items_private { #![allow(unused)]
        pub(crate) use super::_private::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{
            cmp::always::*, float::always::*, int::always::*, niche::always::*,
        };
    }
}
