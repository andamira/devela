// devela/src/media/visual/image/raster/format/mod.rs
//
//! Raster image formats.
//

mod impl_const;

mod base; // RasterFormat, (RasterTransfer, RasterAlpha)
mod channels; // (RasterChannels)
mod packed; // (RasterPackedChannels)
mod sample; // (RasterSampleFormat)

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            channels::*,
            packed::*,
            sample::*,
        };
    }
}
