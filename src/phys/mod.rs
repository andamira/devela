// devela::phys
//
//! Physical units and measurements.
#![doc = crate::doc_!(modules: crate; phys: time, wave)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_phys", forbid(unsafe_code))]

pub mod time;

#[cfg(feature = "wave")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "wave")))]
pub mod wave;

crate::items! { // structural access: _pub_mods, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        pub use super::time::_all::*;

        #[cfg(feature = "wave")]
        pub use super::wave::_all::*;

        // WIPZONE
        // pub use super::bio::_all::*;
        // pub use super::chem::_all::*;
        // pub use super::mech::_all::*;
        // pub use super::unit::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::time::_always::*;
    }
}
// WIPZONE
// pub mod bio;
// pub mod chem;
// pub mod mech;
// pub mod unit;
