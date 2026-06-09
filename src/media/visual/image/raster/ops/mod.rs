// devela::media::visual::image::raster::ops
//
//! Raster operations
//

mod rotate; // Generic dense raster rotation

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            rotate::*,
        };
    }
}
