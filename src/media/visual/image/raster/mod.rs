// devela/src/media/visual/image/raster/mod.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_RASTER!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; raster)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _test;

mod borrow; // Borrowed typed and byte raster views
mod coverage; // Quantized raster-sample coverage
mod element; // Coordinate-and-coverage rasterization output
mod format; // Raster sample and color-format semantics
mod grid; // Logical raster-cell geometry and traversal
mod layout; // Physical raster byte-storage layout
// mod macros; // TODO Raster type generators
// mod ops; // TODO Raster storage and image operations
mod traits; // Typed and byte raster access contracts

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            borrow::{RasterByteSlice, RasterSlice},
            coverage::Coverage8,
            element::RasterElement,
            format::RasterFormat,
            grid::RasterGrid,
            layout::RasterLayout,
            // macros::raster,
            // ops::_all::*,
            traits::{
                RasterView, RasterBuf, Raster,
                RasterViewBytes, RasterBufBytes,
                RasterSamplePacked, RasterViewPacked,
            },
        };
    }
    _crate_internals {
        pub(crate) use super::format::{
            RasterAlpha, RasterChannels, RasterPackedChannels, RasterSampleFormat, RasterTransfer,
        };
    }
}
