// devela::work::sync
//
//! Synchronization primitives.
#![doc = crate::doc_!(extends: sync)]
#![doc = crate::doc_!(modules: crate::work; sync)]
#![doc = crate::doc_!(newline)]
//!
//

mod reexports;

#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
mod atomic;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "work")]
        pub use super::atomic::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
