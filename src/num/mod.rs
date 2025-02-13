// devela::num
//
//! Numerical types and math operations.
#![doc = crate::doc_!(modules: crate; num: alg, geom, logic, niche, ord, quant, rand)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: cmp, num)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private; // upcasted_op!, impl_ops!

mod error; // NumError, NumResult
mod float; // fsize, ExtFloat, ExtFloatConst, Float
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
mod no; // NoNum
mod primitive; // Cast, Primitive[Cast|Join|Split]
mod sign; // Sign
mod traits; // Num, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod geom;
pub mod logic;
pub mod niche; // NonZero*, NonValue*, NonRange*
pub mod ord; // Compare
pub mod quant; // Cycle*, Interval
pub mod rand;

#[cfg(feature = "alg")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alg")))]
pub mod alg;

crate::items! { // structural access: _mods, _pub_mods, _internals, _all, _always
    #[allow(unused)]
    pub use {_internals::*, _mods::*};
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::{
            error::*, float::_all::*, frac::_all::*, int::_all::*,
            no::*, primitive::_all::*, sign::*, traits::*,
        };
        #[cfg(feature = "unit")]
        pub use super::unit::_all::*;
    }
    mod _pub_mods {
        pub use super::{
            geom::_all::*, logic::_all::*, niche::_all::*,
            ord::_all::*, quant::_all::*,rand::_all::*,
        };

        #[cfg(feature = "alg")]
        pub use super::alg::_all::*;
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::{_private::*, rand::_internals::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            ord::_always::*, float::_always::*, int::_always::*, niche::_always::*,
        };
    }
}
