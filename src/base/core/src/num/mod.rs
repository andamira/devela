// devela_base_core::num
//
#![doc = crate::_DOC_NUM!()] // public, root
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: dom, error, fin, grain, lin, prob, quant, symb);
}

mod absence; // NoNum

pub mod dom; // Numeric domains and value representations
pub mod error; // error types
pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values.

#[cfg(feature = "lin")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "lin")))]
pub mod lin; // Linear algebraic structures and methods.

pub mod prob; // Probability theory and statistical inference
pub mod quant; // Quantification, measurement, and numerical relationships
pub mod symb; // Symbolic numeric forms and manipulation

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _workspace_internals, _hidden
    _mods {
        pub use super::absence::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            dom::_all::*,
            error::*,
            fin::_all::*,
            grain::_all::*,
            prob::_all::*,
            quant::_all::*,
            // symb::_all::*,
        };
        #[cfg(feature = "lin")]
        pub use super::lin::_all::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_NUM_MODULES,
            fin::_crate_internals::*,
            prob::_crate_internals::*,
        };
    }
    _workspace_internals {
        pub use super::{
            dom::_workspace_internals::*,
            grain::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::{
            dom::_hidden::*,
            grain::_hidden::*,
            prob::_hidden::*,
        };
    }
}
