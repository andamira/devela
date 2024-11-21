// devela::mem::ptr
//
//! Manually manage memory through raw pointers.
//!
// #![doc = crate::doc_!(extends: ptr)]
//

mod namespace;
mod reexports;

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{namespace::Ptr, reexports::*};

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub use super::fat::FatPtr;
}
