// devela/src/media/visual/draw/mod.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_DRAW!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; draw)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//!
//! This module describes drawing independently of any concrete image representation.
//! Software rasterization that converts geometry into covered raster cells
//! lives in [`image::raster::draw`][crate::media::visual::image::raster::draw].
//

// mod blend; // Source/destination compositing and blend operations
mod canvas; // Canvas, CanvasRead, CanvasTextel
// mod list; // Retained drawing operations
// mod paint; // Spatial sources of color and related drawing styles
// mod stroke; // Width, caps, joins, and dashing

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            canvas::*,
            // list::*,
            // paint::*,
            // stroke::*,
        };
    }
}
