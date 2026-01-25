// devela::media::font
//
//! Font functionality.
#![doc = crate::_doc!(lf)]
#![doc = crate::_doc!(modules: crate::media; font)]
#![doc = crate::_doc!(flat:"media")]
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod art; // FontArt
mod bitmap; // FontBitmap

// WIPZONE
// #[cfg(any(feature = "std", feature = "dep_hashbrown"))]
// pub mod bdf;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            art::*,
            bitmap::*,
        };
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::bdf::*;
    }
}
