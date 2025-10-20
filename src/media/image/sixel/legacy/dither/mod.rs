// devela::media::image::sixel::dither
//
//! LegacySixelDither-related functionality.
//

mod conf;
mod dither;
mod palettes;

pub use dither::LegacySixelDither;
pub(crate) use {conf::*, palettes::*};
