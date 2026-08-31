// devela/src/data/codec/symbol/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_SYMBOL!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; symbol)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod aztec;
    mod braille; // BrailleByte
    // mod code128;
    // mod datamatrix;
    mod_ ean; // European Article Number barcodes
    // mod pdf417;
    // mod qr;
    mod_ tile; // Unicode codecs for subdivided character-cell tiles
    // mod upc;
    mod yijing; // YijingHexagram
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // aztec::_all::*,
            braille::BrailleByte,
            // code128::_all::G*
            // datamatrix::_all::*,
            ean::_all::Ean,
            // pdf417::_all::*,
            // qr::_all::*,
            tile::_all::{Octant, Quadrant, Sextant},
            // upc::_all::*,
            yijing::YijingHexagram,
        };
    }
}
