//
//! Bitmap font data, glyph masks, views, storage and access.
//

crate::mods_in! {
    #[cfg(test)]
    #[cfg(feature = "bitfont")]
    mod _test;

    // items

    #[cfg(feature = "bitfont")]
    mod pixel;
    #[cfg(feature = "bitfont")]
    mod view;
    #[cfg(feature = "bitfont")]
    mod word;

    // assets

    #[cfg(feature = "bitfont")]
    mod fonts; // Fonts::BIT_3_3,  Fonts::BIT_3_5, Fonts::BIT_5_6

    #[cfg(feature = "dvbf")]
    // MAYBE: gate bundled fonts separately when more assets are added.
    mod termivela; // Fonts::TERMIVELA_*
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "bitfont")]
        pub use super::{
            pixel::{FontBitmapPixel, FontBitmapPixelIter},
            view::{FontBitmapView, GlyphBitmapView},
            word::FontBitmapWord,
        };
    }
}
