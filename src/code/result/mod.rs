// devela::code::result
//
//! Result handling.
// #![doc = crate::doc_!(modules: crate::code; result)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: backtrace, error, option, panic, result)]
//!
//! Streamlines error management, result chaining, and introduces utility types and macros.
//!
//! It re-exports the error and result types defined in other modules.
//
// safety
// #![cfg_attr(feature = "safe_result", forbid(unsafe_code))]

mod all_error; // AllError
mod chain_hook; // Chain, Hook
mod ext; // ExtError
mod mismatch; // MisMatch
mod opt_res; // ExtOption, ExtResult, OptRes, ExtOptRes, sok, serr, OptionFmt[Or[Else]]
mod own; // Own
mod panic;
mod reexports;
mod value_quant; // ValueQuant

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{
            all_error::*, chain_hook::*, ext::*, mismatch::*, opt_res::all::*, own::*,
            panic::all::*, reexports::*, value_quant::*,
        };
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
