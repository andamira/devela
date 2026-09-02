// devela/src/media/visual/image/raster/format/mod.rs
//
//! Raster image formats.
//

mod impl_const;

mod base; // RasterFormat
mod channels; //
mod packed; //
mod sample; //

crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::base::RasterFormat;
    }
    _crate_internals {
        pub(crate) use super::{
            base::{RasterAlpha, RasterTransfer},
            channels::RasterChannels,
            packed::RasterPackedChannels,
            sample::RasterSampleFormat,
        };
    }
}
