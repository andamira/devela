// devela::media::font
//
//! Font functionality.
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod bitmap;
mod error;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{bitmap::*, error::*};
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bdf::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// #[cfg(any(feature = "std", feature = "dep_hashbrown"))]
// pub mod bdf;
