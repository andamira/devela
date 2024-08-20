// devela::mem::cell
//
//! Shareable mutable containers.
#![doc = crate::code::doc_extends!(cell)]
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
