// devela::media::font::bitmap
//
//! Bitmap fonts
//

#[cfg(test)]
mod tests;

mod bitmap; // FontBitmap
mod fonts; // FONT_BIT_3_3,  FONT_BIT_3_5, FONT_BIT_5_6

pub use {bitmap::*, fonts::*};
