//
//! Bitmap font data, glyph masks, views, storage and access.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod pixel;
    mod view;
    mod word;

    mod fonts; // Fonts::BIT_3_3,  Fonts::BIT_3_5, Fonts::BIT_5_6
    mod termivela; // Fonts::TERMIVELA_*
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            pixel::{FontBitmapPixel, FontBitmapPixelIter},
            view::{FontBitmapView, GlyphBitmapView},
            word::FontBitmapWord,
        };
    }
}
