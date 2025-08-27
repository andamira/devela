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

crate::mod_path!(_c "../../../libs/base/src/code/result/reexports.rs");

mod chain_hook; // Chain, Hook
mod mismatch; // Mismatch
mod opt_res; // ExtOption, ExtResult, OptRes, ExtOptRes, sok, serr, OptionFmt[Or[Else]]
mod own; // Own
mod value_quant; // ValueQuant

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            _c::*, chain_hook::*, mismatch::*, opt_res::_all::*, own::*, value_quant::*,
        };
        // WIPZONE
        // pub use super::enumatch::*;
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::_c::*;
    }
}
// WIPZONE
// mod enumatch; // enumatch!
// #[cfg(feature = "_tuple")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "_tuple")))]
// mod menu;
