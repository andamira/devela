// devela::media::visual::image
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; image: sixel)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

// #[cfg(feature = "alloc")]
// mod buf; // WIP TEMP
mod error;

// #[cfg(feature = "alloc"] // TEMP
// mod png; // WIP
mod pnm; // WIP

mod raster; // RasterView
#[cfg(feature = "term")]
pub mod sixel; // SixelChar, SixelColor, SixelEncoder, SixelPalette

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{
            error::*,
            raster::*,
            pnm::*,
        };
        // #[cfg(feature = "alloc")]
        // pub use super::buf::*; // TEMP WIP
    }
    _pub_mods {
        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use super::sixel::_all::*;
    }
}
