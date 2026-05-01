// devela::data::codec::crypto::error
//
//!
//

use crate::{_impl_init, impl_trait};

#[doc = crate::_tags!(crypto error)]
/// An error from a cryptographic codec or primitive.
#[doc = crate::_doc_location!("data/codec/crypto")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CryptoError {
    /// The input length exceeds the algorithm's supported range.
    LengthOverflow,

    /// The input length is invalid for the selected operation.
    InvalidLength,

    /// Authentication, verification, or integrity checking failed.
    VerificationFailed,

    /// The key length is invalid for the selected algorithm.
    InvalidKeyLength,
}
impl CryptoError {
    /// Returns `true` for [`CryptoError::LengthOverflow`].
    pub const fn is_length_overflow(self) -> bool {
        matches!(self, Self::LengthOverflow)
    }
    /// Returns `true` for [`CryptoError::InvalidLength`].
    pub const fn is_invalid_length(self) -> bool {
        matches!(self, Self::InvalidLength)
    }
    /// Returns `true` for [`CryptoError::InvalidKeyLength`].
    pub const fn is_invalid_key_length(self) -> bool {
        matches!(self, Self::InvalidKeyLength)
    }
    /// Returns `true` for [`CryptoError::VerificationFailed`].
    pub const fn is_verification_failed(self) -> bool {
        matches!(self, Self::VerificationFailed)
    }
}
_impl_init![ConstInit: Self::LengthOverflow => CryptoError];
impl_trait![fmt::Display+Error for CryptoError |self, f| match self {
    Self::LengthOverflow => f.write_str("cryptographic input length overflow"),
    Self::InvalidLength => f.write_str("invalid cryptographic input length"),
    Self::VerificationFailed => f.write_str("cryptographic verification failed"),
    Self::InvalidKeyLength => f.write_str("invalid cryptographic key length"),
}];
