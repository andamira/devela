// devela_base_core::media::visual::color
//
#![doc = crate::_DOC_MEDIA_VISUAL_COLOR!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; color)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod helpers; // (Norm)

mod gamma; // Gamma
mod luminance; // Luma, Luminance
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            gamma::*,
            luminance::*,
            rgb::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::helpers::*;
    }
}
