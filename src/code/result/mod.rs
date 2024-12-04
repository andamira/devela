// devela::code::result
//
//! Error management, result handling.
#![doc = crate::doc_!(modules: crate; error)]
#![doc = crate::doc_!(newline)]
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
#[allow(unused_imports)]
pub use {
    all_error::*, chain_hook::*, ext::*, mismatch::*, opt_res::all::*, own::*, panic::all::*,
    reexports::*, value_quant::*,
};

pub(crate) mod all {
    #![allow(unused_imports)]
    #[doc(inline)]
    pub use super::{
        all_error::*, chain_hook::*, ext::*, mismatch::*, opt_res::all::*, own::*, panic::all::*,
        reexports::*, value_quant::*,
    };
}
