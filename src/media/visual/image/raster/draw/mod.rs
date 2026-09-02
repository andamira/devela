// devela/src/media/visual/image/raster/draw/mod.rs
//
//! Rasterization of geometric primitives into covered raster cells.
//!
//! Rasterizers in this module convert geometry into [`RasterElement`] streams.
//! They do not choose paint, mutate sample storage, or perform compositing.
//!
//! [`RasterElement`]: crate::RasterElement
//

#[cfg(test)]
mod _test;

// mod ellipse; //
mod line; // Aliased raster-line traversal
// mod path; //
// mod rect; //
// mod triangle; //

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // ellipse::*,
            line::RasterLineIter,
            // path::*,
            // rect::*,
            // triangle::*,
        };
    }
}
