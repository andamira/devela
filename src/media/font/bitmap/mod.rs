// devela/src/media/font/bitmap/mod.rs
//
//! Bitmap font data, glyph masks, views, storage and access.
//

#[cfg(test)]
mod _test;

mod view; // FontBitmapView, GlyphBitmapView
mod word; // FontBitmapWord

mod fonts; // FONT_BIT_3_3,  FONT_BIT_3_5, FONT_BIT_5_6

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            view::*,
            word::*,
            fonts::*,
        };
    }
}
