// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
#![doc = crate::_doc!(vendor: "icy_sixel")]
//

mod builder; // Sixel
mod dither; // Dither, DitherConf…
mod error; // SixelError
mod quant;
mod output; // Sixel, SixelMean, SixelQuality, SixelSplit

///
const SIXEL_PALETTE_MAX: usize = 256;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{builder::*, dither::*, error::*, output::*, quant::*};
    }
}
