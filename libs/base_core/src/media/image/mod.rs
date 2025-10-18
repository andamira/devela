// devela_base_core::media::image
//
//! Image manipulation.
//
// safety
// #![cfg_attr(base_safe_image, forbid(unsafe_code))] // no feature

// mod pnm;
pub mod sixel; // SixelChar, SixelColor, SixelEncoder, SixelPalette

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            // pnm::*,
            sixel::_all::*,
        };
    }
}
