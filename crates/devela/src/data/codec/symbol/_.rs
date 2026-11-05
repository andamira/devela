//
#![doc = crate::_DOC_DATA_CODEC_SYMBOL!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; symbol)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Symbol codecs map data to and from structured marks
//! intended for visual or symbolic representation.
//!
//! The symbols may be textual, tiled, barcoded,
//! or otherwise visually encoded.
//

crate::mods_in! {
    // mod aztec;

    #[cfg(feature = "braille")]
    mod braille; // BrailleByte
    // mod code128;
    // mod datamatrix;
    #[cfg(feature = "ean")]
    mod_ ean; // European Article Number barcodes
    // mod pdf417;
    // mod qr;
    #[cfg(feature = "unitile")]
    mod_ tile; // Unicode codecs for subdivided character-cell tiles
    // mod upc;
    #[cfg(feature = "yijing")]
    mod yijing; // YijingHexagram
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     // aztec::_all::*,
        //     // code128::_all::G*
        //     // datamatrix::_all::*,
        //     // pdf417::_all::*,
        //     // qr::_all::*,
        //     // upc::_all::*,
        // };
        #[cfg(feature = "braille")]
        pub use super::braille::BrailleByte;
        #[cfg(feature = "ean")]
        pub use super::ean::_all::Ean;
        #[cfg(feature = "unitile")]
        pub use super::tile::_all::{Octant, Quadrant, Sextant};
        #[cfg(feature = "yijing")]
        pub use super::yijing::YijingHexagram;
    }
}
