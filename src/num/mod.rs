// devela::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_doc!(modules: crate; num: geom, logic, niche, ord, quant, rand)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num)]
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
mod reexports;
mod traits; // Num, NumConst, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod geom;
pub mod logic;
pub mod niche; // NonZero*, NonValue*|NonExtreme*, NonRange*
pub mod ord; // Compare
pub mod quant; // Cycle*, Interval, interval!, Ratio
pub mod rand;

// WIPZONE
// pub mod symb;
// mod power; // Tp

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _always
    _mods {
        pub use super::{
            error::*, float::_all::*, frac::_all::*, int::_all::*,
            no::*, primitive::_all::*, reexports::*, traits::*,
        };
        #[cfg(feature = "unit")]
        pub use super::unit::_all::*;
        // WIPZONE
        // pub use super::power::*;
    }
    _pub_mods {
        pub use super::{
            geom::_all::*, logic::_all::*, niche::_all::*,
            ord::_all::*, quant::_all::*,rand::_all::*,
        };
        // WIPZONE
        // pub use super::symb::_all::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _private::*,
            geom::_crate_internals::*,
            rand::_crate_internals::*,
        };
    }
    _always {
        pub use super::{
            float::_always::*, niche::_always::*,
        };
    }
}
