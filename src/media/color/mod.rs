// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod helpers;

mod color; // Color
mod gamma; // Gamma
mod luminance; // Luma, Luminance
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

// WIPZONE
// mod spectral;
// #[cfg(feature = "linear")]
// mod xyz; // Xyz

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{color::*, gamma::*, luminance::*, rgb::_all::*};
        // WIPZONE
        // pub use super::spectral::*;
        // #[cfg(feature = "linear")]
        // pub use super::xyz::*;
    }
    _crate_internals {
        pub(crate) use super::helpers::*;
    }
}
