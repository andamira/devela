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

mod absence; // NoNum
mod error; // NumError, NumResult
mod float; // fsize, Float, FloatConst, FloatExt
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
mod primitive; // Cast, Primitive[Cast|Join|Split]
mod reexports;
mod traits; // Num, NumConst, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod geom;
pub mod logic;
pub mod niche; // NonZero*, NonValue*|NonExtreme*, ne!, nz!
pub mod ord; // Cmp
pub mod quant; // Cycle*, Interval, interval!, Ratio
pub mod rand;

// WIPZONE
// pub mod wip_symb;
// mod wip_power; // Tp

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            absence::*, error::*, float::_all::*, frac::_all::*, int::_all::*,
            primitive::_all::*, reexports::*, traits::*,
        };
        #[cfg(feature = "unit")]
        pub use super::unit::_all::*;
        // WIPZONE
        // pub use super::wip_power::*;
    }
    _pub_mods {
        pub use super::{
            geom::_all::*, logic::_all::*, niche::_all::*,
            ord::_all::*, quant::_all::*,rand::_all::*,
        };
        // WIPZONE
        // pub use super::wip_symb::_all::*;
    }
    _crate_internals {
        pub(crate) use super::{
            rand::_crate_internals::*,
        };
    }
}
