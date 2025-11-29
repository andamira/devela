// devela_base_std::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(base_safe_color, forbid(unsafe_code))]

mod gamma; // Gamma

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            gamma::*,
        };
    }
}
