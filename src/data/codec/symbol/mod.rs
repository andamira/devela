// devela/src/data/codec/symbol/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC_SYMBOL!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; symbol)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

// mod aztec;
mod braille; // BrailleByte
// mod code128;
// mod datamatrix;
// mod ean;
// mod pdf417;
// mod qr;
mod quadrant; // Quadrant4
// mod upc;
mod yijing; // YijingHexagram

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // aztec::_all::*,
            braille::BrailleByte,
            // code128::_all::G*
            // datamatrix::_all::*,
            // ean::_all::*,
            // pdf417::_all::*,
            // qr::_all::*,
            quadrant::Quadrant4,
            // upc::_all::*,
            yijing::YijingHexagram,
        };
    }
}
