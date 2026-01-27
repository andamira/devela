// devela::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: dom, error, fin, grain, lin, prob, quant, symb);
}

mod absence; // NoNum
mod error; // NumError, NumResult
mod float; // fsize, Float, FloatConst, FloatExt
mod frac; // Frac
mod int; // [i|u]size_[down|up], Int
// pub mod symb;
mod traits; // Num, NumRef

#[cfg(feature = "unit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unit")))]
mod unit; // Unit, Unit[Bi|Si]

pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values.
pub mod quant; // Cycle*, Interval, interval!, Ratio
pub mod rand;

crate::structural_mods! { // _mods, _pub_mods, _reexports, _crate_internals, _hidden
    _mods {
        pub use super::{
            absence::*,
            error::*,
            float::_all::*,
            frac::_all::*,
            int::_all::*,
            traits::_all::*,
            // wip_power::*;
        };
        #[cfg(feature = "unit")]
        pub use super::unit::_all::*;
    }
    _pub_mods {
        pub use super::{
            fin::_all::*,
            grain::_all::*,
            quant::_all::*,
            rand::_all::*,
            // symb::_all::*;
        };
    }
    _reexports {
        pub use devela_base_core::num::{
            // individual errors:
            IncompatibleBounds,
            NoInverse,
            MismatchedSizes,
            NonNegativeRequired,
            PositiveRequired,
            NonZeroRequired,
            Overflow,
            // composite errors:
            IntError, IntResult,
            NicheValueError,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_NUM_MODULES,
            rand::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            grain::_hidden::*,
        };
    }
}
