// devela::media::visual::image::raster
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_RASTER!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; raster)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

mod borrow; // (4) <Raster[Bytes]<Mut|Ref>>
mod format; // (1+5) RasterFormat, (<Raster[Alpha|[Packed]Channels|SampleFormat|Transfer]>)
mod layout; // (1) RasterLayout
// mod macros; // (1) raster! WIP
mod traits; // (7) Raster<[Buf|View][Bytes]>, <Raster<Sample|View>Packed>>

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            borrow::*,
            format::*,
            layout::*,
            // macros::*,
            traits::*,
        };
    }
}
