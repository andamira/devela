// devela::sys::os
//
//! OS-specific.
#![doc = crate::doc_!(modules: crate::sys; os: linux)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: os)]
//

#[cfg(feature = "linux")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub mod linux;

crate::items! { // structural access: doc_hidden, all
    #[allow(unused)]
    pub use doc_hidden::*;

    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        #[cfg(feature = "linux")]
        pub use super::linux::all::*;
    }
    pub(super) mod all { #[allow(unused)]
        #[doc(inline)]
        pub use super::doc_hidden::*;
    }
}
