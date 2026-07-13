// devela/src/num/mod.rs
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
    crate::_doc!(modules: crate; num: dom, fin, grain, lin, prob, quant, signal); // expr
}

#[cfg(feature = "alg")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alg")))]
pub mod alg; // Algebraic numeric structures and operations.
pub mod dom; // Numeric domains and value representations
// pub mod expr; // Symbolic numeric forms and manipulation
pub mod fin; // Finite and discrete numeric structures
pub mod grain; // Structural granularity and representation of numeric values
// mod intro; // WIP
// pub mod learn; // WIP Adaptive numeric systems for prediction and training.
// pub mod optim; // WIP Objective functions, constraints, and numerical optimization.
pub mod prob; // Probability theory and statistical inference
pub mod quant; // Quantification, measurement, and numerical relationships
pub mod signal; // Numerical signals and value evolution over domains

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        // pub use super::{
        //     intro::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            dom::_all::*,
            // expr::_all::*,
            fin::_all::*,
            grain::_all::*,
            // learn::_all::*,
            // optim::_all::*,
            prob::_all::*,
            quant::_all::*,
            signal::_all::*,
        };
        #[cfg(feature = "alg")]
        pub use super::alg::_all::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_NUM_MODULES,
            dom::_crate_internals::*,
            fin::_crate_internals::*,
            grain::_crate_internals::*,
            prob::_crate_internals::*,
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
