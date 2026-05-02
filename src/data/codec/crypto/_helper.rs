// devela::data::codec::crypto::_helper
//
//! Defines `_hex`, `_crypto_impl_hmac!`.
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

macro_rules! _crypto_impl_hmac {
    ($Self:ident, $digest:ty, $name:literal) => {
        $crate::paste! {
            #[doc = "Computes the " $name " of `message` using the given `key`."]
            ///
            /// If the key is longer than the block size, it is first hashed.
            ///
            /// # Errors
            /// Returns [`CryptoError::LengthOverflow`] if key hashing or the
            /// inner/outer digest computation exceeds the hash input length limit.
            pub const fn hmac(key: &[u8], message: &[u8]) -> Result<$digest, CryptoError> {
                // 1. Prepare a key block.
                let mut key_block = [0u8; $Self::BLOCK_LEN];
                if key.len() > $Self::BLOCK_LEN {
                    // hash the key and pad the rest with zeros
                    let hashed = unwrap![ok? $Self::digest_bytes(key)].into_array();
                    whilst! { i in 0..hashed.len(); { key_block[i] = hashed[i]; }}
                } else {
                    // copy the key and pad with zeros
                    whilst! { i in 0..key.len(); { key_block[i] = key[i]; }}
                }
                // 2. Inner and outer padding
                const IPAD: u8 = 0x36;
                const OPAD: u8 = 0x5c;
                let mut ipad = [0u8; $Self::BLOCK_LEN];
                let mut opad = [0u8; $Self::BLOCK_LEN];
                whilst! { i in 0..$Self::BLOCK_LEN; {
                    ipad[i] = key_block[i] ^ IPAD;
                    opad[i] = key_block[i] ^ OPAD;
                }}
                // 3. Inner hash: H(ipad || message)
                let mut inner = $Self::new();
                unwrap![ok? inner.update(&ipad)];
                unwrap![ok? inner.update(message)];
                let inner_hash = inner.finalize().into_array();
                // 4. Outer hash: H(opad || inner_hash)
                let mut outer = $Self::new();
                unwrap![ok? outer.update(&opad)];
                unwrap![ok? outer.update(&inner_hash)];
                Ok(outer.finalize())
            }
        }
    };
}
pub(crate) use _crypto_impl_hmac;
