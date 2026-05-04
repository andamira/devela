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

use ::devela::{CryptoError, impl_trait, is, unwrap, whilst};
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
/// # use devela::{Otp, Sha1};
/// let key = b"jAaO2ynesPYCdTZV";
/// let hotp = Sha1::hotp(key, 0, Otp::DEFAULT_DIGITS).unwrap();
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

// Constants
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
}
impl Otp {
    const fn validate_digits(digits: u32) -> Result<(), CryptoError> {
        if digits < Self::MIN_DIGITS || digits > Self::MAX_DIGITS {
            Err(CryptoError::InvalidLength)
        } else {
            Ok(())
        }
    }
    /// Creates an OTP code with an explicit decimal digit width.
    ///
    /// The numeric `code` is stored without leading zeroes.
    /// The `digits` value controls display width.
    ///
    /// # Errors
    /// - Returns [`InvalidLength`][CryptoError::InvalidLength]
    ///   if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    /// - Returns [`InvalidParameter`][CryptoError::InvalidParameter]
    ///   if `code` does not fit within the requested digit width.
    pub const fn new(code: u32, digits: u32) -> Result<Self, CryptoError> {
        let modulo = unwrap![ok? Self::modulo(digits)];
        is![code as u64 >= modulo, Err(CryptoError::InvalidParameter), Ok(Self { code, digits })]
    }
    /// Creates an OTP code after reducing `code` modulo `10^digits`.
    ///
    /// This is useful for HOTP/TOTP dynamic truncation.
    ///
    /// # Errors
    /// Returns [`CryptoError::InvalidLength`] if `digits` is outside
    /// `MIN_DIGITS..=MAX_DIGITS`.
    pub const fn new_reduced(code: u64, digits: u32) -> Result<Self, CryptoError> {
        let modulo = unwrap![ok? Self::modulo(digits)];
        Ok(Self { code: (code % modulo) as u32, digits })
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

    /// Returns the decimal modulus for `digits`.
    ///
    /// That is, returns `10^digits`.
    ///
    /// # Errors
    /// Returns [`InvalidLength`][CryptoError::InvalidLength]
    /// if `digits` is outside `MIN_DIGITS..=MAX_DIGITS`.
    #[must_use]
    pub const fn modulo(digits: u32) -> Result<u64, CryptoError> {
        unwrap![ok? Self::validate_digits(digits)];
        let mut n = 1u64;
        whilst! { i in 0..digits; { n *= 10; }}
        Ok(n)
    }
    /// Derives the TOTP moving counter.
    ///
    /// Computes `(unix_seconds - epoch) / period`.
    ///
    /// # Errors
    /// Returns [`InvalidParameter`][CryptoError::InvalidParameter]
    /// if `period == 0` or if `unix_seconds < epoch`.
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

#[allow(unused, reason = "example script")]
#[cfg(all(feature = "std", feature = "time"))]
fn main() {
    use ::devela::{Base32, Otp, Sha1, TimeUnixU32};

    // test here: https://authenticationtest.com/totpChallenge/
    const SECRET: &str = "I65VU7K5ZQL7WB4E";
    const PERIOD: u64 = 30;

    let mut key = [0u8; Base32::decoded_len_stripped(SECRET.as_bytes())];
    let key_len = Base32::decode_from_slice(SECRET.as_bytes(), &mut key).unwrap();
    let now = TimeUnixU32::now().seconds as u64;
    let otp = Sha1::totp(&key[..key_len], now, Otp::DEFAULT_DIGITS).unwrap();
    println!("{otp}");
}

#[cfg(test)]
mod tests {
    use crate::{_hex, Otp, Sha1};
    #[test]
    fn otp_rejects_invalid_digits() {
        let key = _hex::<20>("3132333435363738393031323334353637383930");
        assert!(Sha1::hotp(&key, 0, 0).unwrap_err().is_invalid_length());
        assert!(Sha1::hotp(&key, 0, 11).unwrap_err().is_invalid_length());
    }
    #[test]
    fn otp_rejects_invalid_time_counter_parameters() {
        assert!(Otp::time_counter(100, 0, 0).unwrap_err().is_invalid_parameter());
        assert!(Otp::time_counter(99, 100, 30).unwrap_err().is_invalid_parameter());
    }
}
