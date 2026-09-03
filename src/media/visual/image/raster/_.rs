// devela/src/media/visual/image/raster/_.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_RASTER!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; raster: draw, grid)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

        mod borrow; // Borrowed typed and byte raster views
        mod coverage; // Quantized raster-sample coverage
    pub mod_ draw; // Rasterization of geometric primitives into covered cells
        mod element; // Coordinate-and-coverage rasterization output
        mod_ format; // Raster sample and color-format semantics
    pub mod_ grid; // Logical raster-cell geometry and traversal
        mod layout; // Physical raster byte-storage layout
        // mod macros; // TODO Raster type generators
        // mod_ ops; // WIP Raster storage and image operations
        mod traits; // Typed and byte raster access contracts
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[doc(inline)]
        pub use super::{
            borrow::{RasterByteSlice, RasterSlice},
            coverage::Coverage8,
            element::RasterElement,
            format::_all::RasterFormat,
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
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            draw::_all::*,
            grid::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::format::{
            RasterAlpha, RasterChannels, RasterPackedChannels, RasterSampleFormat, RasterTransfer,
        };
    }
}
