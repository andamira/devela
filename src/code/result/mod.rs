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

crate::items! { // structural access: doc_inline, doc_hidden, all, always
    #[allow(unused)]
    pub use {doc_inline::*, doc_hidden::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{
            chain_hook::*, mismatch::*, opt_res::all::*, own::*,
            panic::all::*, reexports::*, value_quant::*,
        };
    }
    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::error::all::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{error::always::*, panic::always::*, reexports::*};
    }
}
