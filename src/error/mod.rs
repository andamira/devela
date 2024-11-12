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

mod ext_result;
mod mismatch;
mod option;
mod own;
mod panic;
mod reexports;
mod traits;
mod unwrap;
mod value_quant;
#[allow(unused_imports)]
pub use {
    ext_result::*, mismatch::*, option::all::*, own::*, panic::all::*, reexports::*, traits::*,
    unwrap::*, value_quant::*,
};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        ext_result::*, mismatch::*, option::all::*, own::*, panic::all::*, reexports::*, traits::*,
        unwrap::*, value_quant::*,
    };
}
