// devela::mem::cell
//
//! Shareable mutable containers.
#![doc = crate::code::doc_!(extends: cell)]
#![doc = crate::code::doc_!(modules: crate::mem; cell)]
#![doc = crate::code::doc_!(newline)]
//!
//

mod option;
mod reexports;
#[allow(unused_imports)]
pub use {option::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{option::*, reexports::*};
}
