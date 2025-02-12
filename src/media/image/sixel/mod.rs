// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
#![doc = crate::doc_!(vendor: "icy_sixel")]
//

mod builder; // Sixel
mod dither; // Dither, DitherConf…
mod error; // SixelError
mod quant;
mod output; // Sixel, SixelMean, SixelQuality, SixelSplit

///
const SIXEL_PALETTE_MAX: usize = 256;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{builder::*, dither::*, error::*, output::*, quant::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
