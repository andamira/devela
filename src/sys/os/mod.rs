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

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(feature = "linux")]
        pub use super::linux::all::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] #[allow(unused_imports)]
        pub use super::doc_inline::*;
    }
}
