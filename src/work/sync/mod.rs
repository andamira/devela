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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
        #[cfg(feature = "work")]
        pub use super::atomic::*;
    }
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[cfg(feature = "alloc")] #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
