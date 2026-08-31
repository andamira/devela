// devela/src/data/codec/pack/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_PACK!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; pack)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ archive; // Multi-resource packages
    mod_ compress; // Size-reducing codecs
    mod_ wrap; // Structured wrappers and chunked containers
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // archive::_all::*,
            compress::_all::*,
            wrap::_all::*,
        };
    }
}
