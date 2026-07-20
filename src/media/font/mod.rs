// devela/src/media/font/mod.rs
//
#![doc = crate::_DOC_MEDIA_FONT!()] // public
#![doc = crate::_doc!(modules: crate::media; font)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod _helper;

mod art; // Fixed-size Unicode/text-art font representations
mod bitmap; // Bitmap font data, glyph masks, views, storage and access
// mod color; // Font-specific color structures
mod format; // Font storage and interchange formats
// mod generate; // Procedural construction, derivation and synthesis
// mod inspect; // Descriptive inspection and optional quality policies
// mod metric; // Objective dimensions and placement
// mod outline; // Contour-based glyph representation
// mod semantic; // Format-independent typographic meaning

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            art::_all::*,
            bitmap::_all::*,
            // color::_all::*,
            format::_all::*,
            // generate::_all::*,
            // inspect::_all::*,
            // metric::_all::*,
            // outline::_all::*,
            // semantic::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::_Font;
    }
}
