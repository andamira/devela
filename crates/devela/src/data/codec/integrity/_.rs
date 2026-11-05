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
    #[cfg(feature = "adler")]
    mod adler; // Adler-32 checksum
    #[cfg(feature = "crc")]
    mod crc; // Cyclic redundancy checks
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "adler")]
        pub use super::adler::Adler32;
        #[cfg(feature = "crc")]
        pub use super::crc::Crc;
    }
}
