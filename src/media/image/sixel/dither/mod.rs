// devela::media::image::sixel::dither
//
//! Dither-related functionality.
//

mod conf;
mod dither;
mod palettes;

pub use dither::Dither;
pub(crate) use {conf::*, palettes::*};
