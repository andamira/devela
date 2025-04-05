// devela::num
//
//! Numerical types and math operations.
#![doc = crate::doc_!(modules: crate; num: geom, logic, niche, ord, quant, rand)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: cmp, num)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]

mod _private; // upcasted_op!, impl_ops!

mod error; // NumError, NumResult
mod float; // fsize, ExtFloat, Float, FloatConst
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
mod no; // NoNum
mod primitive; // Cast, Primitive[Cast|Join|Split]
mod sign; // Sign
mod traits; // Num, NumConst, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod geom;
pub mod logic;
pub mod niche; // NonZero*, NonValue*|NonExtreme*, NonRange*
pub mod ord; // Compare
pub mod quant; // Cycle*, Interval, Ratio
pub mod rand;

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
        // WIPZONE
        // pub use super::power::*;
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            geom::_all::*, logic::_all::*, niche::_all::*,
            ord::_all::*, quant::_all::*,rand::_all::*,
        };
        // WIPZONE
        // pub use super::symb::_all::*;
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
// WIPZONE
// pub mod symb;
// mod power; // Tp
