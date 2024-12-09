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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        // #[cfg(feature = "time")]
        pub use super::time::all::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::time::always::*;
    }
}
