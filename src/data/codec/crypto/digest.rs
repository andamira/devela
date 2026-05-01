// devela::data::codec::crypto::digest
//
//! Defines [`Digest`].
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
