// devela::media::image::sixel::legacy
//
//! Legacy implementation originally based on `libsixel`.
#![doc = crate::_doc!(vendor: "icy_sixel")]
//

///
const SIXEL_PALETTE_MAX: usize = 256;

mod builder; // LegacySixel
mod dither; // LegacySixelDither
mod error; // LegacySixelError
mod quant;
mod output; // LegacySixelMean, LegacySixelQuality, LegacySixelSplit

use quant::*;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{builder::*, dither::*, error::*, output::*};
    }
}
