// devela::phys
//
//! Physical units and measurements.
#![doc = crate::doc_!(modules: crate; phys: time)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_phys", forbid(unsafe_code))]

// #[cfg(feature = "time")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
pub mod time;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        // #[cfg(feature = "time")]
        pub use super::time::all::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::time::always::*;
    }
}
