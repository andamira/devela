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
//! structured ownership ([`Own`]) and mismatch-aware comparisons ([`Mismatch`]).
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/code/result/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;

        #[doc(inline)]
        pub use devela_base_core::code::result::{
            Chain, Hook, Mismatch,
            OptRes, OptResExt, serr, sok,
            OptionExt, OptionFmt, OptionFmtOr, OptionFmtOrElse,
            Own, ResultExt, unwrap,
        };
    }
}
