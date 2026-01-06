// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod color; // Color
// mod spectral;
// #[cfg(feature = "linear")]
// mod xyz; // Xyz

crate::structural_mods! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::{
            color::*,
            // spectral::*,
        };
        // #[cfg(feature = "linear")]
        // pub use super::xyz::*;
    }
    _reexports {
        pub use devela_base_core::media::color::{
            GammaConst, Lum, // & aliases:
            Lightness, LinearLightness, Luma, Luminance,
            Rgb, Rgba, // & aliases:
            Rgb8, Rgba8, RgbaPre8, Rgb16, Rgba16, RgbaPre16,
            RgbF32, RgbaF32, RgbaPreF32, RgbF64, RgbaF64, RgbaPreF64,
            RgbLinF32, RgbaLinF32, RgbaLinPreF32, RgbLinF64, RgbaLinF64, RgbaLinPreF64,
        };
        #[cfg(feature = "std")]
        pub use devela_base_std::media::color::{
            Gamma,
        };
    }
}
