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

pub mod time;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
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
