// devela::error
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

#[allow(unused_imports)]
use crate::code::items;

mod mismatch;
mod opt_res;
mod own;
mod panic;
mod reexports;
mod chain_hook;
mod unwrap;
mod value_quant;
#[allow(unused_imports)]
pub use {
    chain_hook::*, mismatch::*, opt_res::all::*, own::*, panic::all::*, reexports::*, unwrap::*,
    value_quant::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        chain_hook::*, mismatch::*, opt_res::all::*, own::*, panic::all::*, reexports::*,
        unwrap::*, value_quant::*,
    };
}
