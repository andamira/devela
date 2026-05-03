// devela::data::codec::crypto
//
#![doc = crate::_DOC_DATA_CODEC_CRYPTO!()] // public
#![doc = crate::_doc!(modules: crate::data::codec; crypto)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests_sha;

mod _helper; // (_crypto_impl_hmac, _crypto_impl_otp, _hex)

mod digest; // Digest
mod error; // CryptoError
mod md5; // Md5
mod otp; // Otp
mod sha1; // Sha1 TODO (__crypto_impl_sha1)
mod sha2; // (__crypto_impl_sha2)

crate::structural_mods! { // _mods, crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            digest::*,
            error::*,
            md5::*,
            otp::Otp,
            sha1::*,
            sha2::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _helper::_hex,
        };
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            _helper::{__crypto_impl_hmac, __crypto_impl_otp},
            sha2::{__crypto_impl_sha2, __SHA2_64_K, __SHA2_32_K},
        };
    }
}
