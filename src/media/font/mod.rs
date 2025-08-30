// devela::media::font
//
//! Font functionality.
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod bitmap;

// WIPZONE
// #[cfg(any(feature = "std", feature = "dep_hashbrown"))]
// pub mod bdf;

crate::structural_mods! { // _mods, _all
    _mods {
        pub use super::bitmap::*;
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bdf::*;
    }
}
