// devela::work::sync
//
//! Synchronization primitives.
#![doc = crate::doc_!(extends: sync)]
#![doc = crate::doc_!(modules: crate::work; sync)]
#![doc = crate::doc_!(newline)]
//!
//

#[cfg(feature = "alloc")]
mod reexports;

#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
mod atomic;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
        #[cfg(feature = "work")]
        pub use super::atomic::*;
    }
    pub(super) mod all { #[allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
    }
}
