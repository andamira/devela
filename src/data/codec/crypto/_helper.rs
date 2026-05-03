// devela::data::codec::crypto::_helper
//
//! Defines `_crypto_impl_hmac!`, `_hex`.
//

pub(crate) const fn _hex<const N: usize>(s: &str) -> [u8; N] {
    use devela::{Base, Rfc4648};
    type Hex = Base<16, false, false, true, Rfc4648>;
    let input = s.as_bytes();
    assert!(input.len() == N * 2);
    let mut out = [0u8; N];
    let written = match Hex::decode_from_slice(input, &mut out) {
        Some(written) => written,
        None => panic!("invalid hex"),
    };
    assert!(written == N);
    out
}

/// Implements the standard HMAC construction for a concrete digest type.
///
/// The hash type must provide `BLOCK_LEN`, `digest_bytes`, `new`, `update`, and `finalize`.
/// The generated method is allocation-free and const-friendly.
macro_rules! _crypto_impl_hmac {
    ($Self:ident, $digest:ty) => {
        $crate::paste! {
            /// Computes the HMAC of `message` using the given `key`.
            ///
            /// If the key is longer than the block size, it is first hashed.
            ///
            /// # Errors
            /// Returns [`LengthOverflow`] if key hashing or the
            /// inner/outer digest computation exceeds the hash input length limit.
            ///
            /// [`LengthOverflow`]: crate::CryptoError::LengthOverflow
            pub const fn hmac(key: &[u8], message: &[u8]) -> Result<$digest, $crate::CryptoError> {
                // 1. Prepare a key block.
                let mut key_block = [0u8; $Self::BLOCK_LEN];
                if key.len() > $Self::BLOCK_LEN {
                    // hash the key and pad the rest with zeros
                    let hashed = $crate::unwrap![ok? $Self::digest_bytes(key)].into_array();
                    $crate::whilst! { i in 0..hashed.len(); { key_block[i] = hashed[i]; }}
                } else {
                    // copy the key and pad with zeros
                    $crate::whilst! { i in 0..key.len(); { key_block[i] = key[i]; }}
                }
                // 2. Inner and outer padding
                const IPAD: u8 = 0x36;
                const OPAD: u8 = 0x5c;
                let mut ipad = [0u8; $Self::BLOCK_LEN];
                let mut opad = [0u8; $Self::BLOCK_LEN];
                $crate::whilst! { i in 0..$Self::BLOCK_LEN; {
                    ipad[i] = key_block[i] ^ IPAD;
                    opad[i] = key_block[i] ^ OPAD;
                }}
                // 3. Inner hash: H(ipad || message)
                let mut inner = $Self::new();
                $crate::unwrap![ok? inner.update(&ipad)];
                $crate::unwrap![ok? inner.update(message)];
                let inner_hash = inner.finalize().into_array();
                // 4. Outer hash: H(opad || inner_hash)
                let mut outer = $Self::new();
                $crate::unwrap![ok? outer.update(&opad)];
                $crate::unwrap![ok? outer.update(&inner_hash)];
                Ok(outer.finalize())
            }
        }
    };
}
pub(crate) use _crypto_impl_hmac;
