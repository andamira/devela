// devela::data::codec::integrity
//
#![doc = crate::_DOC_DATA_CODEC_INTEGRITY!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; integrity)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod adler; // Adler32
// mod crc; // Crc32 WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            adler::*,
            // crc::*,
        };
    }
}
