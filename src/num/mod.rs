// devela::num
//
#![doc = crate::_DOC_NUM!()] // public, root
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(feature = "safe_num", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: dom, error, fin, grain, lin, prob, quant); // symb
}

mod symb; // Symbolic numeric forms and manipulation WIP

pub mod dom; // Numeric domains and value representations
pub mod error; // Numeric-related error types.
pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values
pub mod prob; // Probability theory and statistical inference
pub mod quant; // Quantification, measurement, and numerical relationships

#[cfg(feature = "lin")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "lin")))]
pub mod lin;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            symb::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            dom::_all::*,
            error::_all::*,
            fin::_all::*,
            grain::_all::*,
            prob::_all::*,
            quant::_all::*,
        };
        #[cfg(feature = "lin")]
        pub use super::lin::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_NUM_MODULES;
    }
    _hidden {
        pub use super::{
            grain::_hidden::*,
        };
    }
}
