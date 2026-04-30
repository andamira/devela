// devela::media::visual::image::raster
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_RASTER!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; raster)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

mod macros; // WIP raster!
mod traits; // Raster[[Buf|View][Bytes]], Raster<Sample|View>Packed
// mod types; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            macros::*,
            traits::*,
            // types::*,
        };
    }
}
