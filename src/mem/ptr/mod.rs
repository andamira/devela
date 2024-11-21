// devela::mem::ptr
//
//! Manually manage memory through raw pointers.
//!
// #![doc = crate::doc_!(extends: ptr)]
//

mod namespace;
mod reexports;
#[allow(unused_imports)]
pub use {namespace::Ptr, reexports::*};

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
crate::code::items! {
    mod fat;
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub use fat::FatPtr;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{namespace::Ptr, reexports::*};

    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub use super::fat::FatPtr;
}
