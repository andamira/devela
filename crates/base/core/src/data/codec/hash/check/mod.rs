// devela_base_core::data::codec::hash::check
//
#![doc = crate::_DOC_DATA_CODEC_HASH_CHECK!()] // private
#![doc = crate::_doc!(modules: crate::data::codec::hash; check)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod adler; // Adler32

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            adler::*,
        };
    }
}
