// devela_base_core::media::visual::image::sixel
//
//! Sixel encoding functionality.
//
// DOCS
// - https://en.wikipedia.org/wiki/Sixel
// - https://vt100.net/docs/vt3xx-gp/chapter14.html
// - https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Sixel-Graphics
// - https://www.digiater.nl/openvms/decus/vax90b1/krypton-nasa/all-about-sixels.text
// - https://nick-black.com/dankwiki/index.php/Sixel
// - https://www.arewesixelyet.com

mod char; // SixelChar
mod color; // SixelColor
mod encoder; // SixelEncoder
mod palette; // SixelPalette, SixelPaletteIter

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            char::*,
            color::*,
            encoder::*,
            palette::*,
        };
    }
}
