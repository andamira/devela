// devela::code::result
//
//! Results, options, and outcome-based types.
// #![doc = crate::doc_!(modules: crate::code; result)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: option, result)]
//!
//! Computation doesn’t just yield values, it reaches resolutions.
//! This module refines how results are formed, owned, transformed,
//! and resolved, ensuring that every outcome finds its place.
//

mod chain_hook; // Chain, Hook
mod mismatch; // MisMatch
mod opt_res; // ExtOption, ExtResult, OptRes, ExtOptRes, sok, serr, OptionFmt[Or[Else]]
mod own; // Own
mod reexports;
mod value_quant; // ValueQuant

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            chain_hook::*, mismatch::*, opt_res::_all::*,
            own::*, reexports::*, value_quant::*,
        };
        // WIPZONE
        // #[cfg(feature = "_tuple")]
        // pub use super::menu::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
// WIPZONE
// #[cfg(feature = "_tuple")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_tuple")))]
// mod menu;
