// devela/src/data/codec/integrity/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_INTEGRITY!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; integrity)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod adler; // Adler-32 checksum
    mod crc; // Cyclic redundancy checks
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            adler::Adler32,
            crc::Crc,
        };
    }
}
