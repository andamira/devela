// devela/src/data/codec/pack/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC_PACK!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; pack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

// mod archive; // Multi-resource packages
mod compress; // Size-reducing codecs
mod wrap; // Structured wrappers and chunked containers

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // archive::_all::*,
            compress::_all::*,
            wrap::_all::*,
        };
    }
}
