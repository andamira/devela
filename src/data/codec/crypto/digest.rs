// devela::data::codec::crypto::digest
//
//! Defines [`Digest`] and [`digest!`].
//

use crate::{is, whilst};

#[doc = crate::_tags!(crypto hash)]
/// A fixed-size message digest.
#[doc = crate::_doc_location!("data/codec/crypto")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Digest<const N: usize>(pub [u8; N]);

#[rustfmt::skip]
impl<const N: usize> Digest<N> {
    /// The digest size in bytes.
    pub const LEN: usize = N;

    /// Creates a digest from bytes.
    pub const fn new(bytes: [u8; N]) -> Self { Self(bytes) }

    /// Returns the digest bytes.
    #[must_use] pub const fn as_array(&self) -> &[u8; N] { &self.0 }
    /// Returns the digest bytes by value.
    #[must_use] pub const fn into_array(self) -> [u8; N] { self.0 }
    /// Returns the digest bytes.
    #[must_use] pub const fn as_slice(&self) -> &[u8; N] { &self.0 }
    /// Returns the digest size in bytes.
    #[must_use] pub const fn len(&self) -> usize { N }
    /// Returns `true` if the digest has zero length.
    #[must_use] pub const fn is_empty(&self) -> bool { N == 0 }

    /// Returns `true` if both digests contain the same bytes.
    ///
    /// This is compile-time friendly.
    /// It does not promise hardened side-channel behavior at runtime.
    #[must_use]
    pub const fn eq(&self, other: &Self) -> bool {
        let mut diff = 0u8;
        whilst! { i in 0..N; { diff |= self.0[i] ^ other.0[i]; }}
        diff == 0
    }
    /// Returns `true` if this digest equals `bytes`.
    ///
    /// This is compile-time friendly.
    /// It does not promise hardened side-channel behavior at runtime.
    #[must_use]
    pub const fn eq_slice(&self, bytes: &[u8]) -> bool {
        is! { bytes.len() != N, return false }
        let mut diff = 0u8;
        whilst! { i in 0..N; { diff |= self.0[i] ^ bytes[i]; }}
        diff == 0
    }
}
#[rustfmt::skip]
mod impls_traits {
    use crate::{ConstInit, Digest};
    impl<const N: usize> ConstInit for Digest<N> {
        const INIT: Self = Self([0; N]);
    }
    impl<const N: usize> From<[u8; N]> for Digest<N> {
        fn from(bytes: [u8; N]) -> Self { Self(bytes) }
    }
    impl<const N: usize> From<Digest<N>> for [u8; N] {
        fn from(digest: Digest<N>) -> Self { digest.0 }
    }
    impl<const N: usize> AsRef<[u8]> for Digest<N> {
        fn as_ref(&self) -> &[u8] { &self.0 }
    }
}

#[doc = crate::_tags!(crypto hash construction)]
/// Defines a selected cryptographic message-digest state type.
#[doc = crate::_doc_location!("data/codec/crypto")]
///
/// Generates a concrete allocation-free, const-friendly digest type from a
/// whitelisted algorithm selector.
///
/// The generated type includes streaming update/finalize methods, one-shot
/// digesting, HMAC support when applicable, and OTP helpers when the digest
/// output is suitable for HOTP truncation.
///
/// # Examples
/// ```
/// use devela::{Digest, digest};
/// digest! {
///     /// Incremental SHA-256 state for this module.
///     pub struct MySha256: Sha256
/// }
/// let digest = MySha256::digest_bytes(b"abc").unwrap();
/// assert_eq!(
///     digest,
///     Digest([
///         0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea,
///         0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
///         0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
///         0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
///     ]),
/// );
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! digest· {
    ( // supported hashes: Sha1, Sha256, Sha512, Sha224, Sha384, Sha512_224, Sha512_256
    $(#[$attr:meta])* $vis:vis struct $name:ident: Md5 $(;)?) => {
        $crate::__crypto_impl_md5! { $(#[$attr])* $vis struct $name }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha1 $(;)?) => {
        $crate::__crypto_impl_sha1! { $(#[$attr])* $vis struct $name }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha256 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u32, doc: "SHA-256",
            digest_bits: 256, digest_len: 32, output_words: 8, output_tail_bytes: 0,
            initial_state: [
                0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A,
                0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19]
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha512 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u64, doc: "SHA-512",
            digest_bits: 512, digest_len: 64, output_words: 8, output_tail_bytes: 0,
            initial_state: [
                0x6A09_E667_F3BC_C908, 0xBB67_AE85_84CA_A73B,
                0x3C6E_F372_FE94_F82B, 0xA54F_F53A_5F1D_36F1,
                0x510E_527F_ADE6_82D1, 0x9B05_688C_2B3E_6C1F,
                0x1F83_D9AB_FB41_BD6B, 0x5BE0_CD19_137E_2179]
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha224 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u32, doc: "SHA-224",
            digest_bits: 224, digest_len: 28, output_words: 7, output_tail_bytes: 0,
            initial_state: [
                0xC105_9ED8, 0x367C_D507, 0x3070_DD17, 0xF70E_5939,
                0xFFC0_0B31, 0x6858_1511, 0x64F9_8FA7, 0xBEFA_4FA4]
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha384 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u64, doc: "SHA-384",
            digest_bits: 384, digest_len: 48, output_words: 6, output_tail_bytes: 0,
            initial_state: [
                0xCBBB_9D5D_C105_9ED8, 0x629A_292A_367C_D507,
                0x9159_015A_3070_DD17, 0x152F_ECD8_F70E_5939,
                0x6733_2667_FFC0_0B31, 0x8EB4_4A87_6858_1511,
                0xDB0C_2E0D_64F9_8FA7, 0x47B5_481D_BEFA_4FA4]
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha512_224 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u64, doc: "SHA-512/224",
            digest_bits: 224, digest_len: 28, output_words: 3, output_tail_bytes: 4,
            initial_state: [
                0x8C3D_37C8_1954_4DA2, 0x73E1_9966_89DC_D4D6,
                0x1DFA_B7AE_32FF_9C82, 0x679D_D514_582F_9FCF,
                0x0F6D_2B69_7BD4_4DA8, 0x77E3_6F73_04C4_8942,
                0x3F9D_85A8_6A1D_36C8, 0x1112_E6AD_91D6_92A1]
        }
    };
    ($(#[$attr:meta])* $vis:vis struct $name:ident: Sha512_256 $(;)?) => {
        $crate::__crypto_impl_sha2! { $(#[$attr])* $vis struct $name; word: u64, doc: "SHA-512/256",
            digest_bits: 256, digest_len: 32, output_words: 4, output_tail_bytes: 0,
            initial_state: [
                0x2231_2194_FC2B_F72C, 0x9F55_5FA3_C84C_64C2,
                0x2393_B86B_6F53_B151, 0x9638_7719_5940_EABD,
                0x9628_3EE2_A88E_FFE3, 0xBE5E_1E25_5386_3992,
                0x2B01_99FC_2C85_B8AA, 0x0EB7_2DDC_81C5_2CA2]
        }
    };
    // unsupported hash error
    ($(#[$attr:meta])* $vis:vis struct $name:ident: $hash:ident $(;)?) => {
        compile_error! { concat!("Hash function not supported: ", stringify!($hash)) }
    };
}
#[doc(inline)]
pub use digest· as digest;
