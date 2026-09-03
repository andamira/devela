// devela/src/media/font/_.rs
//
#![doc = crate::_DOC_MEDIA_FONT!()] // public
#![doc = crate::_doc!(modules: crate::media; font)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

crate::mods_in! {
    mod_ art; // Fixed-size Unicode/text-art font representations
    mod_ bitmap; // Bitmap font data, glyph masks, views, storage and access
    // mod_ color; // Font-specific color structures
    mod_ format; // Font storage and interchange formats
    // mod_ generate; // Procedural construction, derivation and synthesis
    // mod_ inspect; // Descriptive inspection and optional quality policies
    // mod_ metric; // Objective dimensions and placement
    mod namespace; // Fonts
    // mod_ outline; // Contour-based glyph representation
    // mod_ semantic; // Format-independent typographic meaning
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            art::_all::*,
            bitmap::_all::*,
            // color::_all::*,
            format::_all::*,
            // generate::_all::*,
            // inspect::_all::*,
            // metric::_all::*,
            namespace::Fonts,
            // outline::_all::*,
            // semantic::_all::*,
        };
    }
}
