// devela/src/data/codec/symbol/ean/eight.rs
//
//! Implements EAN-8 for [`Ean<8>`][crate::Ean].
//

use super::_helper;
use crate::{Ean, is, read_at, unwrap, whilst, write_at};

impl Ean<8> {
    /// Number of digits preceding the check digit.
    pub const DATA_DIGITS: usize = 7;

    /// Number of logical modules, excluding quiet zones.
    pub const MODULES: usize = 67;

    /* digits */

    /// Calculates the mandatory check digit for seven GTIN-8 data digits.
    ///
    /// Returns `None` if any input value is greater than 9.
    #[must_use]
    pub const fn check_digit(data: [u8; 7]) -> Option<u8> {
        _helper::ean_check_digit(&data)
    }
    /// Appends the calculated check digit to seven GTIN-8 data digits.
    ///
    /// Returns `None` if any input value is greater than 9.
    #[must_use]
    pub const fn with_check_digit(data: [u8; 7]) -> Option<[u8; 8]> {
        let check = unwrap![some? Self::check_digit(data)];
        let mut digits = [0; 8];
        write_at![digits, 0, @7 data, check];
        Some(digits)
    }
    /// Returns whether all digits are decimal and the check digit is correct.
    #[must_use]
    pub const fn is_valid(digits: [u8; 8]) -> bool {
        let data = read_at![digits, 0, @7];
        unwrap![some_or Self::check_digit(data), check => check == digits[7], false]
    }

    /* encoding */

    /// Encodes a valid EAN-8 digit sequence into its 67 logical modules.
    ///
    /// Returns `None` if a digit is not decimal or the check digit is invalid.
    #[must_use]
    pub const fn encode(digits: [u8; 8]) -> Option<u128> {
        is! { !Self::is_valid(digits), return None }
        let mut modules = _helper::GUARD as u128;
        whilst! { i in 0..4; {
            modules = (modules << 7) | _helper::SET_A[digits[i] as usize] as u128;
        }}
        modules = (modules << 5) | _helper::CENTER as u128;
        whilst! { i in i,..8; {
            modules = (modules << 7) | _helper::set_c(digits[i]) as u128;
        }}
        Some((modules << 3) | _helper::GUARD as u128)
    }

    /* decoding */

    /// Decodes an exact 67-module EAN-8 pattern.
    ///
    /// Returns `None` for malformed guards, invalid digit patterns,
    /// excess high bits, or an invalid check digit.
    #[must_use]
    pub const fn decode(modules: u128) -> Option<[u8; 8]> {
        is! { modules >> Self::MODULES != 0, return None } // The representation must be canonical
        // Guards.
        if ((modules >> 64) & 0b111) != _helper::GUARD as u128
            || ((modules >> 31) & 0b1_1111) != _helper::CENTER as u128
            || (modules & 0b111) != _helper::GUARD as u128
        {
            return None;
        }
        let mut digits = [0_u8; 8];
        // Left half: number set A.
        whilst! { i in 0..4; {
            let shift = 57 - i * 7;
            let pattern = ((modules >> shift) & 0x7F) as u8;
            digits[i] = unwrap![some? _helper::decode_a(pattern)];
        }}
        // Right half: number set C, the bitwise complement of A.
        whilst! { i in i,..8; {
            let shift = 24 - (i - 4) * 7;
            let pattern = ((modules >> shift) & 0x7F) as u8;
            digits[i] = unwrap![some? _helper::decode_c(pattern)];
        }}
        is! { Self::is_valid(digits), Some(digits), None }
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    type Ean8 = Ean<8>;

    #[test]
    fn check_digits() {
        assert_eq!(Ean8::check_digit([9, 6, 3, 8, 5, 0, 7]), Some(4));
        assert_eq!(Ean8::check_digit([0; 7]), Some(0));
        assert_eq!(Ean8::with_check_digit([9, 6, 3, 8, 5, 0, 7]), Some([9, 6, 3, 8, 5, 0, 7, 4]),);
        assert_eq!(Ean8::check_digit([0, 1, 2, 3, 4, 5, 10]), None);
    }
    #[test]
    fn validity() {
        assert!(Ean8::is_valid([9, 6, 3, 8, 5, 0, 7, 4]));
        assert!(Ean8::is_valid([0; 8]));
        assert!(!Ean8::is_valid([9, 6, 3, 8, 5, 0, 7, 3]));
        assert!(!Ean8::is_valid([9, 6, 3, 8, 5, 0, 10, 4]));
    }
    #[test]
    fn canonical_symbol() {
        let digits = [9, 6, 3, 8, 5, 0, 7, 4];
        let expected = 0x5_16BD_EB75_4EE5_12E5;
        assert_eq!(Ean8::encode(digits), Some(expected));
        assert_eq!(Ean8::decode(expected), Some(digits));
    }
    #[test]
    fn rejects_invalid_encoding_input() {
        assert_eq!(Ean8::encode([9, 6, 3, 8, 5, 0, 7, 3]), None);
        assert_eq!(Ean8::encode([9, 6, 3, 8, 5, 0, 10, 4]), None);
    }
    #[test]
    fn rejects_invalid_module_patterns() {
        let valid = Ean8::encode([9, 6, 3, 8, 5, 0, 7, 4]).unwrap();
        // Bits outside the canonical 67-bit representation.
        assert_eq!(Ean8::decode(valid | (1_u128 << 67)), None);
        // Corrupted left, centre and right guards.
        assert_eq!(Ean8::decode(valid ^ (1_u128 << 66)), None);
        assert_eq!(Ean8::decode(valid ^ (1_u128 << 33)), None);
        assert_eq!(Ean8::decode(valid ^ 1), None);
    }
    #[test]
    fn roundtrip_examples() {
        for data in [
            [0, 0, 0, 0, 0, 0, 0],
            [9, 6, 3, 8, 5, 0, 7],
            [1, 2, 3, 4, 5, 6, 7],
            [9, 9, 9, 9, 9, 9, 9],
        ] {
            let digits = Ean8::with_check_digit(data).unwrap();
            let modules = Ean8::encode(digits).unwrap();
            assert_eq!(Ean8::decode(modules), Some(digits));
        }
    }
}
