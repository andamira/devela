// devela/src/media/visual/image/raster/grid/mod.rs
//
//! Logical raster-cell geometry and traversal.
//

#[cfg(test)]
mod _test;

mod adam7; // Adam7 interlaced raster traversal
mod coord; // RasterCoordIter
mod define; // RasterGrid
mod interlace; // Bitmap raster interlacing methods

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            adam7::{Adam7Pass, Adam7Row, Adam7Rows},
            coord::RasterCoordIter,
            define::RasterGrid,
            interlace::{Interlace, InterlaceSet},
        };
    }
}
