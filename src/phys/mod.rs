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

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        // #[cfg(feature = "time")]
        pub use super::time::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::time::_always::*;
    }
}
