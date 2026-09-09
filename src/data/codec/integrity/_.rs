// devela/src/data/codec/integrity/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_INTEGRITY!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; integrity)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Integrity codes derive compact values
//! used to detect changes or corruption in represented data.
//!
//! They verify structural fidelity rather than identity or authenticity.
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
