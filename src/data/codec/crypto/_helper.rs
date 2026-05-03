// devela::data::codec::crypto::_helper
//
//! Defines `_hex`, `__crypto_impl_hmac!`, `__crypto_impl_otp`.
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
#[doc(hidden)]
#[macro_export]
macro_rules! __crypto_impl_hmac {
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
#[doc(hidden)]
pub use __crypto_impl_hmac;

#[doc(hidden)]
#[macro_export]
macro_rules! __crypto_impl_otp {
    ($otp:path, $sha:ident, $doc:literal) => { $crate::paste! {
        #[doc = $doc " impls"]
        impl $sha {
            #[doc = "Generates an HOTP code using HMAC-" $doc "."]
            ///
            /// Computes `HOTP(K, C)` from the shared secret `key` and 8-byte big-endian
            /// counter, then applies dynamic truncation.
            ///
            /// The returned code is numeric and may have fewer visible digits than
            /// [`digits`][crate::Otp::digits]; format it with leading zeroes for display.
            ///
            /// # Errors
            /// - Returns [`InvalidLength`][crate::CryptoError::InvalidLength]
            ///   if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
            /// - Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow] if the
            #[doc = "underlying HMAC-" $doc " computation exceeds " $doc "'s input length limit."]
            pub const fn hotp(key: &[u8], counter: u64, digits: u32)
                -> Result<$otp, $crate::CryptoError> {
                $crate::unwrap![ok? $otp::validate_digits(digits)];
                let mac = $crate::unwrap![ok? <$sha>::hmac(key, &counter.to_be_bytes())];
                let offset = (mac.0[<$sha>::DIGEST_LEN - 1] & 0x0f) as usize;
                let code = u32::from_be_bytes([
                    mac.0[offset] & 0x7f,
                    mac.0[offset + 1],
                    mac.0[offset + 2],
                    mac.0[offset + 3],
                ]);
                let modulo = 10u64.pow(digits);
                Ok($otp::from_code_digits((code as u64 % modulo) as u32, digits))
            }

            #[doc = "Generates a TOTP code using HMAC-" $doc " and the default TOTP parameters."]
            ///
            /// Uses [`DEFAULT_EPOCH`][crate::Otp::DEFAULT_EPOCH]
            /// and [`DEFAULT_PERIOD`][crate::Otp::DEFAULT_PERIOD].
            ///
            /// # Errors
            /// - Returns [`InvalidLength`][crate::CryptoError::InvalidLength]
            ///   if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
            /// - Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow]
            #[doc = "underlying HMAC-" $doc " computation exceeds " $doc "'s input length limit."]
            pub const fn totp(key: &[u8], unix_seconds: u64, digits: u32)
                -> Result<$otp, $crate::CryptoError> {
                $sha::totp_with(key, unix_seconds,
                    $otp::DEFAULT_EPOCH, $otp::DEFAULT_PERIOD, digits)
            }
            #[doc = "Generates a TOTP code using HMAC-" $doc " and explicit TOTP parameters."]
            ///
            /// Derives the moving counter from `unix_seconds`, `epoch`, and `period`,
            #[doc = "then computes HOTP-" $doc " with that counter."]
            ///
            /// # Errors
            /// - Returns [`InvalidParameter`][crate::CryptoError::InvalidParameter]
            ///   if `period == 0` or if `unix_seconds < epoch`.
            /// - Returns [`InvalidLength`][crate::CryptoError::InvalidLength]
            ///   if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
            /// - Returns [`LengthOverflow`][crate::CryptoError::LengthOverflow]
            #[doc = "underlying HMAC-" $doc " computation exceeds " $doc "'s input length limit."]
            pub const fn totp_with(key: &[u8], unix_seconds: u64, epoch: u64,
                period: u64, digits: u32) -> Result<$otp, $crate::CryptoError> {
                let counter = $crate::unwrap![ok? $otp::time_counter(unix_seconds, epoch, period)];
                $sha::hotp(key, counter, digits)
            }
        }
    }};
}
#[doc(hidden)]
pub use __crypto_impl_otp;
