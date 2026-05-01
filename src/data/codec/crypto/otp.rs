#!/usr/bin/env -S rust-script -c --debug
//! ```cargo
//! [dependencies]
//! devela = { path = "../../../..", features = ["std", "time", "_docs_examples"]}
//! [features]
//! default = ["std", "time", "_docs_examples"]
//! _docs_examples = []
//! std = []
//! time = []
//! ```
// WAIT:[cargo-script](https://github.com/rust-lang/cargo/issues/12207)
//
// devela::data::codec::crypto::otp
//
//! Defines [`Otp`].
//

#![cfg_attr(feature = "_docs_examples", allow(unexpected_cfgs, reason = "example script"))]

use ::devela::{CryptoError, Sha1, unwrap};
::devela::_use_or_shim![_doc_location, _doc_vendor, _tags];

#[doc = _tags!(crypto hash)]
/// HMAC-based One‑Time Password generators (HOTP / TOTP).
#[doc = _doc_location!("data/codec/crypto")]
///
/// Implements [RFC 4226](https://tools.ietf.org/html/rfc4226) (HOTP)
/// and [RFC 6238](https://tools.ietf.org/html/rfc6238) (TOTP)
/// using SHA-1 as the underlying HMAC function.
///
/// # Examples
/// ```
/// # use devela::Otp;
/// let key = b"12345678901234567890";
/// let hotp = Otp::generate_hotp(key, 0, Otp::DEFAULT_DIGITS).unwrap();
/// assert_eq!(hotp, 755224);
/// ```
///
/// # Errors
///
/// The only error that can occur is [`CryptoError::LengthOverflow`] from the
/// underlying HMAC‑SHA1 computation, if the total message length (including
/// the padded key) exceeds SHA‑1’s maximum message length (≈ 2^64 bits).
/// In practice, for HOTP/TOTP with a small key and a fixed 8‑byte counter,
/// this will never happen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Otp;
impl Otp {
    /// Number of digits in the generated password.
    pub const DEFAULT_DIGITS: u32 = 6;
    /// Reference Unix second used as counter base for TOTP.
    pub const DEFAULT_EPOCH: u64 = 0;
    /// Time step in seconds for TOTP (usually 30).
    pub const DEFAULT_PERIOD: u64 = 30;

    /// Converts `unix_seconds` into a counter value based on `period`.
    /// The result is `unix_seconds / period`.
    pub const fn counter(unix_seconds: u64, period: u64) -> u64 {
        unix_seconds / period
    }
    /// Generates an HMAC-based One-Time Password (HOTP) using SHA-1.
    ///
    /// Returns a number in `0..10^digits`.
    ///
    /// The shared secret `key` is fed into an HMAC together with the 8‑byte
    /// big‑endian representation of `counter`. The resulting hash is then
    /// truncated to `digits` decimal digits (typically 6).
    pub const fn generate_hotp(key: &[u8], counter: u64, digits: u32) -> Result<u32, CryptoError> {
        let mac = unwrap![ok? Sha1::hmac(key, &counter.to_be_bytes())];
        let offset = (mac.0[19] & 0x0f) as usize;
        let code = u32::from_be_bytes([
            mac.0[offset] & 0x7f,
            mac.0[offset + 1],
            mac.0[offset + 2],
            mac.0[offset + 3],
        ]);
        Ok(code % 10u32.pow(digits))
    }
    /// Generates a Time-based One-Time Password (TOTP).
    ///
    /// Converts the current Unix timestamp `unix_seconds` into a counter
    /// by dividing by `period` (usually 30 seconds). This derived counter
    /// is then used to compute a HOTP via [`generate_hotp`][Self::generate_hotp].
    ///
    /// The `digits` parameter controls the length of the output code (commonly 6).
    pub const fn generate_totp(
        key: &[u8],
        unix_seconds: u64,
        period: u64,
        digits: u32,
    ) -> Result<u32, CryptoError> {
        let counter = Self::counter(unix_seconds, period);
        Self::generate_hotp(key, counter, digits)
    }
}

#[allow(unused, reason = "example script")]
#[cfg(all(feature = "std", feature = "time"))]
fn main() {
    use ::devela::{Base32, Otp, TimeUnixU32};

    // test here: https://authenticationtest.com/totpChallenge/
    const SECRET: &str = "I65VU7K5ZQL7WB4E";
    const PERIOD: u64 = 30;

    let mut key = [0u8; Base32::decoded_len_stripped(SECRET.as_bytes())];
    let key_len = Base32::decode_from_slice(SECRET.as_bytes(), &mut key).unwrap();
    let now = TimeUnixU32::now().seconds as u64;
    let otp = Otp::generate_totp(&key[..key_len], now, PERIOD, Otp::DEFAULT_DIGITS).unwrap();
    println!("{otp:06}");
}
