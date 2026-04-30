// devela::media::visual::image
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; image: format, raster, sixel)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
pub mod format; // WIP Png, Pnm, Qoi, …
pub mod raster; // Raster[[Buf|View][Bytes]], Raster<Sample|View>Packed, raster!

#[cfg(feature = "term")]
pub mod sixel; // SixelChar, SixelColor, SixelEncoder, SixelPalette

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{
            error::*,
        };
    }
    _pub_mods {
        pub use super::{
            raster::_all::*,
            format::_all::*,
        };
        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use super::sixel::_all::*;
    }
}
