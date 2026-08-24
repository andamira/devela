// devela/src/media/visual/image/raster/grid/mod.rs
//
//! Logical raster-cell geometry and traversal.
//

#[cfg(test)]
mod _test;

mod adam7; // Adam7 interlaced raster traversal
mod define; // RasterGrid
mod coord; // RasterCoordIter

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            adam7::{Adam7Pass, Adam7Row, Adam7Rows},
            define::RasterGrid,
            coord::RasterCoordIter,
        };
    }
}
