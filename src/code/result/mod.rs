// devela::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()] // public
#![doc = crate::_doc!(modules: crate::code; result)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: option, result)]
//!
//! Computation doesn't just yield values, it organizes outcomes.
//! This module refines how results, selections, and structured alternatives
//! are formed, owned, transformed, and resolved, ensuring that every outcome
//! finds its place.
//!
//! It includes fundamental outcome types ([`Option`], [`Result`], [`OptRes`]),
//! structured ownership ([`Own`]) and mismatch-aware comparisons ([`Mismatch`]).
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/code/result/_reexport.rs

#[cfg(all(test, feature = "std"))]
mod unwrap_tests_std;

// mod enumatch; // enumatch! WIP
mod hook_morph; // Hook, Morph, hook!, morph!
mod mismatch; // Mismatch
mod opt_res; // OptRes, sok, serr
mod own; // Own

// #[cfg(feature = "_tuple")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
// mod menu; // WIP

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            // enumatch::*,
            hook_morph::*,
            mismatch::*,
            opt_res::_all::*,
            own::*,
        };
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
