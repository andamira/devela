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

mod opt_res; // ExtOption, ExtResult, OptionFmt[Or[Else]]
mod reexports;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{opt_res::_all::*, reexports::*};
    }
    _always {
        pub use super::reexports::*;
    }
}
