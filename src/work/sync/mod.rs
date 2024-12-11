// devela::work::sync
//
//! Synchronization primitives.
#![doc = crate::doc_!(extends: sync)]
#![doc = crate::doc_!(modules: crate::work; sync)]
#![doc = crate::doc_!(newline)]
//!
//

mod atomic;

#[cfg(feature = "alloc")]
mod reexports;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::atomic::*;
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
    }
    pub(super) mod all { #[allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::atomic::*;
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
    }
}
