// devela::code::result
//
//! Result handling.
#![doc = crate::doc_!(modules: crate::code; result: error)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: backtrace, error, option, panic, result)]
//!
//! Streamlines error management, result chaining, and introduces utility types and macros.
//

mod chain_hook; // Chain, Hook
mod mismatch; // MisMatch
mod opt_res; // ExtOption, ExtResult, OptRes, ExtOptRes, sok, serr, OptionFmt[Or[Else]]
mod own; // Own
mod panic;
mod reexports;
mod value_quant; // ValueQuant

pub mod error; // AllError, modular errors

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use {_mods::*, _pub_mods::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{
            chain_hook::*, mismatch::*, opt_res::_all::*, own::*,
            panic::_all::*, reexports::*, value_quant::*,
        };
    }
    mod _pub_mods {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::error::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{error::_always::*, panic::_always::*, reexports::*};
    }
}
