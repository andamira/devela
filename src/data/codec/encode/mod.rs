// devela::data::codec::encode
//
#![doc = crate::_DOC_DATA_CODEC_ENCODE!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; encode)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: encode)]
//

mod enums; // EncodingMode

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            enums::*,
        };
    }
}
