// devela::data::codec::crypto
//
#![doc = crate::_DOC_DATA_CODEC_CRYPTO!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; crypto)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests_sha;

mod _helper; // (_crypto_impl_hmac, _hex)

mod digest; // Digest
mod error; // CryptoError
mod md5; // Md5
mod otp; // Otp
mod sha1; // Sha1
mod sha2; // Sha256, Sha512… (_crypto_impl_sha2)

crate::structural_mods! { // _mods, crate_internals
    _mods {
        pub use super::{
            digest::*,
            error::*,
            md5::*,
            otp::*,
            sha1::*,
            sha2::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _helper::*,
            sha2::{_crypto_impl_sha2, _SHA2_64_K, _SHA2_32_K},
        };
    }
}
