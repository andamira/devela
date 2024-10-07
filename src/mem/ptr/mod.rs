// devela::mem::cell
//
//! Shareable mutable containers.
#![doc = crate::code::doc_!(extends: ptr)]
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
