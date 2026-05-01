// devela::data::codec::crypto
//
#![doc = crate::_DOC_DATA_CODEC_CRYPTO!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; crypto)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod _helper;

mod digest; // Digest
mod error; // CryptoError
// mod md5; // Md5 WIP
mod otp; // Otp
mod sha1; // Sha1

crate::structural_mods! { // _mods, crate_internals
    _mods {
        pub use super::{
            digest::*,
            error::*,
            // md5::*,
            otp::*,
            sha1::*,
        };
    }
    _crate_internals {
        #[cfg(test)]
        pub(crate) use super::_helper::*;
    }
}
