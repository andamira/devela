// devela/src/media/font/bitmap/mod.rs
//
//! Bitmap font data, glyph masks, views, storage and access.
//

#[cfg(test)]
mod _test;

mod view; // FontBitmapView, GlyphBitmapView
mod word; // FontBitmapWord

mod fonts; // Fonts::BIT_3_3,  Fonts::BIT_3_5, Fonts::BIT_5_6
mod termivela; // Fonts::TERMIVELA_*

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            view::*,
            word::*,
        };
    }
}
