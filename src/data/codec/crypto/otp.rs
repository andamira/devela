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

use ::devela::{CryptoError, Sha1, Sha256, Sha512, impl_trait, unwrap};
::devela::_use_or_shim![_doc_location, _doc_vendor, _tags];

#[doc = _tags!(crypto hash)]
/// A generated one-time password code.
#[doc = _doc_location!("data/codec/crypto")]
///
/// Provides HOTP and TOTP constructors as defined by [RFC 4226] and [RFC 6238].
///
/// SHA-1 is the HOTP algorithm mandated by RFC 4226, and remains the common
/// interoperability default for TOTP. RFC 6238 also permits HMAC-SHA-256 and
/// HMAC-SHA-512 variants.
///
/// The numeric code is stored without leading zeroes;
/// use [`digits`][Self::digits] when formatting it for display.
///
/// # Examples
/// ```
/// # use devela::Otp;
/// let key = b"jAaO2ynesPYCdTZV";
/// let hotp = Otp::hotp_sha1(key, 0, Otp::DEFAULT_DIGITS).unwrap();
/// assert_eq!(hotp.code(), 1639);
/// # #[cfg(alloc)]
/// assert_eq!(hotp.to_string(), "001639");
/// ```
/// [RFC 4226]: https://tools.ietf.org/html/rfc4226
/// [RFC 6238]: https://tools.ietf.org/html/rfc6238
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Otp {
    code: u32,
    digits: u32,
}
impl_trait! { fmt::Display for Otp |self, f|
    write!(f, "{:0width$}", self.code, width = self.digits as usize)
}
impl Otp {
    /// Default number of decimal OTP digits.
    pub const DEFAULT_DIGITS: u32 = 6;
    /// Minimum supported decimal OTP digits.
    pub const MIN_DIGITS: u32 = 1;
    /// Maximum supported decimal OTP digits for a `u32` code.
    ///
    /// HOTP dynamic truncation produces a 31-bit value, so 10-digit codes are
    /// supported as display width but do not span the full 10-digit range.
    pub const MAX_DIGITS: u32 = 10;
    /// Default TOTP initial counter time, the Unix epoch.
    pub const DEFAULT_EPOCH: u64 = 0;
    /// Default TOTP time step in seconds.
    pub const DEFAULT_PERIOD: u64 = 30;

    const fn validate_digits(digits: u32) -> Result<(), CryptoError> {
        if digits < Self::MIN_DIGITS || digits > Self::MAX_DIGITS {
            Err(CryptoError::InvalidLength)
        } else {
            Ok(())
        }
    }

    /// Returns the numeric OTP code, without leading zeroes.
    #[must_use]
    pub const fn code(self) -> u32 {
        self.code
    }
    /// Returns the decimal digit width used by this OTP code.
    #[must_use]
    pub const fn digits(self) -> u32 {
        self.digits
    }

    /// Derives the TOTP moving counter.
    ///
    /// Computes `(unix_seconds - epoch) / period`.
    ///
    /// # Errors
    /// Returns [`CryptoError::InvalidParameter`] if `period == 0`
    /// or if `unix_seconds < epoch`.
    pub const fn time_counter(
        unix_seconds: u64,
        epoch: u64,
        period: u64,
    ) -> Result<u64, CryptoError> {
        if period == 0 || unix_seconds < epoch {
            Err(CryptoError::InvalidParameter)
        } else {
            Ok((unix_seconds - epoch) / period)
        }
    }
}
/// Sha-1 impls
impl Otp {
    /// Generates an HOTP code using HMAC-SHA-1.
    ///
    /// Computes `HOTP(K, C)` from the shared secret `key` and 8-byte big-endian
    /// counter, then applies dynamic truncation.
    ///
    /// The returned code is numeric and may have fewer visible digits than
    /// [`digits`][Self::digits]; format it with leading zeroes for display.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-1
    ///   computation exceeds SHA-1's input length limit.
    pub const fn hotp_sha1(key: &[u8], counter: u64, digits: u32) -> Result<Otp, CryptoError> {
        unwrap![ok? Self::validate_digits(digits)];
        let mac = unwrap![ok? Sha1::hmac(key, &counter.to_be_bytes())];
        let offset = (mac.0[19] & 0x0f) as usize;
        let code = u32::from_be_bytes([
            mac.0[offset] & 0x7f,
            mac.0[offset + 1],
            mac.0[offset + 2],
            mac.0[offset + 3],
        ]);
        let modulo = 10u64.pow(digits);
        Ok(Self { code: (code as u64 % modulo) as u32, digits })
    }

    /// Generates a TOTP code using HMAC-SHA-1 and the default TOTP parameters.
    ///
    /// Uses [`DEFAULT_EPOCH`][Self::DEFAULT_EPOCH] and [`DEFAULT_PERIOD`][Self::DEFAULT_PERIOD].
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-1
    ///   computation exceeds SHA-1's input length limit.
    pub const fn totp_sha1(
        key: &[u8],
        unix_seconds: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        Self::totp_sha1_with(key, unix_seconds, Self::DEFAULT_EPOCH, Self::DEFAULT_PERIOD, digits)
    }
    /// Generates a TOTP code using HMAC-SHA-1 and explicit TOTP parameters.
    ///
    /// Derives the moving counter from `unix_seconds`, `epoch`, and `period`,
    /// then computes HOTP-SHA1 with that counter.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidParameter`] if `period == 0` or if `unix_seconds < epoch`.
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-1
    ///   computation exceeds SHA-1's input length limit.
    pub const fn totp_sha1_with(
        key: &[u8],
        unix_seconds: u64,
        epoch: u64,
        period: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        let counter = unwrap![ok? Self::time_counter(unix_seconds, epoch, period)];
        Self::hotp_sha1(key, counter, digits)
    }
}
/// Sha-256 impls
impl Otp {
    /// Generates an HOTP code using HMAC-SHA-256.
    ///
    /// Computes `HOTP(K, C)` from the shared secret `key` and 8-byte big-endian
    /// counter, then applies dynamic truncation.
    ///
    /// The returned code is numeric and may have fewer visible digits than
    /// [`digits`][Self::digits]; format it with leading zeroes for display.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-256
    ///   computation exceeds SHA-256's input length limit.
    pub const fn hotp_sha256(key: &[u8], counter: u64, digits: u32) -> Result<Otp, CryptoError> {
        unwrap![ok? Self::validate_digits(digits)];
        let mac = unwrap![ok? Sha256::hmac(key, &counter.to_be_bytes())];
        let offset = (mac.0[Sha256::DIGEST_LEN - 1] & 0x0f) as usize;
        let code = u32::from_be_bytes([
            mac.0[offset] & 0x7f,
            mac.0[offset + 1],
            mac.0[offset + 2],
            mac.0[offset + 3],
        ]);
        let modulo = 10u64.pow(digits);
        Ok(Self { code: (code as u64 % modulo) as u32, digits })
    }

    /// Generates a TOTP code using HMAC-SHA-256 and the default TOTP parameters.
    ///
    /// Uses [`DEFAULT_EPOCH`][Self::DEFAULT_EPOCH] and [`DEFAULT_PERIOD`][Self::DEFAULT_PERIOD].
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-256
    ///   computation exceeds SHA-256's input length limit.
    pub const fn totp_sha256(
        key: &[u8],
        unix_seconds: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        Self::totp_sha256_with(key, unix_seconds, Self::DEFAULT_EPOCH, Self::DEFAULT_PERIOD, digits)
    }
    /// Generates a TOTP code using HMAC-SHA-256 and explicit TOTP parameters.
    ///
    /// Derives the moving counter from `unix_seconds`, `epoch`, and `period`,
    /// then computes HOTP-SHA-256 with that counter.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidParameter`] if `period == 0` or if `unix_seconds < epoch`.
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-256
    ///   computation exceeds SHA-256's input length limit.
    pub const fn totp_sha256_with(
        key: &[u8],
        unix_seconds: u64,
        epoch: u64,
        period: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        let counter = unwrap![ok? Self::time_counter(unix_seconds, epoch, period)];
        Self::hotp_sha256(key, counter, digits)
    }
}
/// Sha-512 impls
impl Otp {
    /// Generates an HOTP code using HMAC-SHA-512.
    ///
    /// Computes `HOTP(K, C)` from the shared secret `key` and 8-byte big-endian
    /// counter, then applies dynamic truncation.
    ///
    /// The returned code is numeric and may have fewer visible digits than
    /// [`digits`][Self::digits]; format it with leading zeroes for display.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-512
    ///   computation exceeds SHA-512's input length limit.
    pub const fn hotp_sha512(key: &[u8], counter: u64, digits: u32) -> Result<Otp, CryptoError> {
        unwrap![ok? Self::validate_digits(digits)];
        let mac = unwrap![ok? Sha512::hmac(key, &counter.to_be_bytes())];
        let offset = (mac.0[Sha512::DIGEST_LEN - 1] & 0x0f) as usize;
        let code = u32::from_be_bytes([
            mac.0[offset] & 0x7f,
            mac.0[offset + 1],
            mac.0[offset + 2],
            mac.0[offset + 3],
        ]);
        let modulo = 10u64.pow(digits);
        Ok(Self { code: (code as u64 % modulo) as u32, digits })
    }

    /// Generates a TOTP code using HMAC-SHA-512 and the default TOTP parameters.
    ///
    /// Uses [`DEFAULT_EPOCH`][Self::DEFAULT_EPOCH] and [`DEFAULT_PERIOD`][Self::DEFAULT_PERIOD].
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-512
    ///   computation exceeds SHA-512's input length limit.
    pub const fn totp_sha512(
        key: &[u8],
        unix_seconds: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        Self::totp_sha512_with(key, unix_seconds, Self::DEFAULT_EPOCH, Self::DEFAULT_PERIOD, digits)
    }
    /// Generates a TOTP code using HMAC-SHA-512 and explicit TOTP parameters.
    ///
    /// Derives the moving counter from `unix_seconds`, `epoch`, and `period`,
    /// then computes HOTP-SHA-512 with that counter.
    ///
    /// # Errors
    /// - Returns [`CryptoError::InvalidParameter`] if `period == 0` or if `unix_seconds < epoch`.
    /// - Returns [`CryptoError::InvalidLength`] if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`CryptoError::LengthOverflow`] if the underlying HMAC-SHA-512
    ///   computation exceeds SHA-512's input length limit.
    pub const fn totp_sha512_with(
        key: &[u8],
        unix_seconds: u64,
        epoch: u64,
        period: u64,
        digits: u32,
    ) -> Result<Self, CryptoError> {
        let counter = unwrap![ok? Self::time_counter(unix_seconds, epoch, period)];
        Self::hotp_sha512(key, counter, digits)
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
    let otp = Otp::totp_sha1(&key[..key_len], now, Otp::DEFAULT_DIGITS).unwrap();
    println!("{otp}");
}

#[cfg(test)]
mod tests {
    use super::{super::_hex, Otp};
    #[test]
    fn otp_rejects_invalid_digits() {
        let key = _hex::<20>("3132333435363738393031323334353637383930");
        assert!(Otp::hotp_sha1(&key, 0, 0).unwrap_err().is_invalid_length());
        assert!(Otp::hotp_sha1(&key, 0, 11).unwrap_err().is_invalid_length());
    }
    #[test]
    fn otp_rejects_invalid_time_counter_parameters() {
        assert!(Otp::time_counter(100, 0, 0).unwrap_err().is_invalid_parameter());
        assert!(Otp::time_counter(99, 100, 30).unwrap_err().is_invalid_parameter());
    }
    #[test]
    // https://datatracker.ietf.org/doc/html/rfc4226#page-32
    fn otp_sha1_rfc4226() {
        let key = _hex::<20>("3132333435363738393031323334353637383930");
        let hotp = Otp::hotp_sha1(&key, 0, Otp::DEFAULT_DIGITS).unwrap();
        assert_eq![hotp.digits(), 6];
        assert_eq![hotp.code(), 755224];
        assert_eq![Otp::hotp_sha1(&key, 1, Otp::DEFAULT_DIGITS).unwrap().code(), 287082];
        assert_eq![Otp::hotp_sha1(&key, 2, Otp::DEFAULT_DIGITS).unwrap().code(), 359152];
        assert_eq![Otp::hotp_sha1(&key, 3, Otp::DEFAULT_DIGITS).unwrap().code(), 969429];
        assert_eq![Otp::hotp_sha1(&key, 4, Otp::DEFAULT_DIGITS).unwrap().code(), 338314];
        assert_eq![Otp::hotp_sha1(&key, 5, Otp::DEFAULT_DIGITS).unwrap().code(), 254676];
        assert_eq![Otp::hotp_sha1(&key, 6, Otp::DEFAULT_DIGITS).unwrap().code(), 287922];
        assert_eq![Otp::hotp_sha1(&key, 7, Otp::DEFAULT_DIGITS).unwrap().code(), 162583];
        assert_eq![Otp::hotp_sha1(&key, 8, Otp::DEFAULT_DIGITS).unwrap().code(), 399871];
        assert_eq![Otp::hotp_sha1(&key, 9, Otp::DEFAULT_DIGITS).unwrap().code(), 520489];
    }
    /* https://www.rfc-editor.org/rfc/rfc6238#appendix-B */
    #[test]
    fn otp_sha1_rfc6238() {
        let key = b"12345678901234567890"; // 20
        assert_eq![Otp::totp_sha1_with(key, 59, 0, 30, 8).unwrap().code(), 94287082];
        assert_eq![Otp::totp_sha1_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 07081804];
        assert_eq![Otp::totp_sha1_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 14050471];
        assert_eq![Otp::totp_sha1_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 89005924];
        assert_eq![Otp::totp_sha1_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 69279037];
        assert_eq![Otp::totp_sha1_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 65353130];
    }
    #[test]
    fn otp_sha256_rfc6238() {
        let key = b"12345678901234567890123456789012"; // 32
        assert_eq![Otp::totp_sha256_with(key, 59, 0, 30, 8).unwrap().code(), 46119246];
        assert_eq![Otp::totp_sha256_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 68084774];
        assert_eq![Otp::totp_sha256_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 67062674];
        assert_eq![Otp::totp_sha256_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 91819424];
        assert_eq![Otp::totp_sha256_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 90698825];
        assert_eq![Otp::totp_sha256_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 77737706];
    }
    #[test]
    fn otp_sha512_rfc6238() {
        let key = b"1234567890123456789012345678901234567890123456789012345678901234"; // 64
        assert_eq![Otp::totp_sha512_with(key, 59, 0, 30, 8).unwrap().code(), 90693936];
        assert_eq![Otp::totp_sha512_with(key, 1_111_111_109, 0, 30, 8).unwrap().code(), 25091201];
        assert_eq![Otp::totp_sha512_with(key, 1_111_111_111, 0, 30, 8).unwrap().code(), 99943326];
        assert_eq![Otp::totp_sha512_with(key, 1_234_567_890, 0, 30, 8).unwrap().code(), 93441116];
        assert_eq![Otp::totp_sha512_with(key, 2_000_000_000, 0, 30, 8).unwrap().code(), 38618901];
        assert_eq![Otp::totp_sha512_with(key, 20_000_000_000, 0, 30, 8).unwrap().code(), 47863826];
    }
}
