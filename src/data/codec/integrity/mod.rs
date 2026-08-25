// devela/src/data/codec/integrity/mod.rs
//
#![doc = crate::_DOC_DATA_CODEC_INTEGRITY!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; integrity)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod adler; // Adler-32 checksum
mod crc; // Cyclic redundancy checks

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            adler::Adler32,
            crc::Crc,
        };
    }
}
