// devela::media::visual::color
//
#![doc = crate::_DOC_MEDIA_VISUAL_COLOR!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; color)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

#[cfg(feature = "color")]
mod _helper; // (TEMP Norm)

mod color; // Color
mod depth; // ColorDepth

#[cfg(feature = "color")]
mod gamma; // Gamma
#[cfg(feature = "color")]
mod luminance; // Luma, Luminance
#[cfg(feature = "color")]
mod palette; // WIP
#[cfg(feature = "color")]
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

// #[cfg(feature = "linear")]
// mod xyz; // Xyz

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            color::Color,
            depth::*,
        };
        #[cfg(feature = "color")]
        pub use super::{
            gamma::*,
            luminance::*,
            palette::_all::*,
            rgb::_all::*,
        };
        // #[cfg(feature = "linear")]
        // pub use super::xyz::*;
    }
    _crate_internals {
        pub(crate) use super::{
            color::_media_visual_color_impl,
        };
        #[cfg(feature = "color")]
        pub(crate) use super::{
            _helper::*,
        };
    }
}
