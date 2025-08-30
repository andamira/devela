// devela::code::result
//
#![doc = crate::_DOC_CODE_RESULT!()]
//!
// #![doc = crate::_doc!(modules: crate::code; result)]
// #![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: option, result)]
//!
//! Computation doesn’t just yield values, it organizes outcomes.
//! This module refines how results, selections, and structured alternatives
//! are formed, owned, transformed, and resolved, ensuring that every outcome
//! finds its place.
//!
//! It includes fundamental outcome types ([`Option`], [`Result`], [`OptRes`]),
//! structured ownership ([`Own`]), quantified values ([`ValueQuant`]),
//! and mismatch-aware comparisons ([`Mismatch`]).
//

mod chain_hook; // Chain, Hook
mod opt_res; // ExtOption, ExtResult, OptRes, ExtOptRes, sok, serr, OptionFmt[Or[Else]]
mod own; // Own
mod reexports;
mod value_quant; // ValueQuant

// WIPZONE
// mod enumatch; // enumatch!
// #[cfg(feature = "_tuple")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
// mod menu;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{
            chain_hook::*, opt_res::_all::*, own::*, reexports::*, value_quant::*,
        };
        // WIPZONE
        // pub use super::enumatch::*;
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
    _always {
        pub use super::reexports::*;
    }
}
