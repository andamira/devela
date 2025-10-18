// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
#![doc = crate::_doc!(vendor: "icy_sixel")]
//

mod builder; // LegacySixel
mod dither; // LegacySixelDither
mod error; // LegacySixelError
mod quant;
mod output; // LegacySixelMean, LegacySixelQuality, LegacySixelSplit

///
const SIXEL_PALETTE_MAX: usize = 256;

crate::structural_mods! { // _mods
    _mods {
        // re-exports
        pub use devela_base_core::media::image::{
            SixelChar, SixelColor, SixelEncoder, SixelPalette,
        };

        pub use super::{builder::*, dither::*, error::*, output::*, quant::*};
    }
}
