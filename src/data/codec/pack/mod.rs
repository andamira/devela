// devela::data::codec::pack
//
#![doc = crate::_DOC_DATA_CODEC_PACK!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; pack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod compress; // CompressionMode

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            compress::*,
        };
    }
}
