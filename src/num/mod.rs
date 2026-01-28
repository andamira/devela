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
// pub mod symb;

pub mod dom; // Numeric domains and value representations
pub mod error; // Numeric-related error types.
pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values
pub mod quant; // Cycle*, Interval, interval!, Ratio
pub mod rand;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            absence::*,
            // wip_power::*;
        };
    }
    _pub_mods {
        pub use super::{
            dom::_all::*,
            error::_all::*,
            fin::_all::*,
            grain::_all::*,
            quant::_all::*,
            rand::_all::*,
            // symb::_all::*;
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
