// devela::media::visual::color
//
#![doc = crate::_DOC_MEDIA_VISUAL_COLOR!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; color)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod helpers; // (TEMP Norm)

mod color; // Color
mod gamma; // GammaConst TEMP
mod luminance; // Luma, Luminance
mod palette; // WIP
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

#[cfg(feature = "std")]
mod gamma_std; // Gamma IMPROVE: merge with GammaConst

// #[cfg(feature = "linear")]
// mod xyz; // Xyz

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            color::*,
            gamma::*,
            luminance::*,
            palette::_all::*,
            rgb::_all::*,
        };
        #[cfg(feature = "std")]
        pub use super::gamma_std::Gamma;
        // #[cfg(feature = "linear")]
        // pub use super::xyz::*;
    }
    _crate_internals {
        pub(crate) use super::helpers::*;
    }
}
