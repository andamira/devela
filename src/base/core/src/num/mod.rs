// devela_base_core::num
//
#![doc = crate::_DOC_NUM!()]
#![doc = crate::_DOC_NUM_MODULES!()]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: cmp, num, simd)]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_NUM_MODULES =
    crate::_doc!(modules: crate; num: dom, error, fin, grain, lin, prob, quant, symb);
}

pub mod dom; // Numeric domains and value representations
mod _internals; // impl_ops!, upcasted_op!
pub mod error; // error types
pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values.
// pub mod lin; //
pub mod quant; // Cycle*, Interval, interval!, Ratio, ValueQuant
// pub mod symb; //

crate::structural_mods! { // _pub_mods, _crate_internals, _workspace_internals, _hidden
    _pub_mods {
        #[doc(inline)]
        pub use super::error::*;
        pub use super::{
            dom::_all::*,
            fin::_all::*,
            grain::_all::*,
            // lin::_all::*,
            quant::_all::*,
            // symb::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_NUM_MODULES,
            fin::_crate_internals::*,
        };
    }
    _workspace_internals {
        pub use super::{
            _internals::*,
            dom::_workspace_internals::*,
            grain::_workspace_internals::*,
        };
    }
    _hidden {
        pub use super::{
            dom::_hidden::*,
            grain::_hidden::*,
        };
    }
}
