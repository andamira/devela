// devela::sys::mem::ptr
//
//! Manually manage memory through raw pointers.
//!
// #![doc = crate::doc_!(extends: ptr)]
//

mod namespace;
mod reexports;

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod fat;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{namespace::Ptr, reexports::*};
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        pub use super::fat::FatPtr;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        pub use super::reexports::*;
    }
}
