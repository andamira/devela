// devela/src/data/codec/symbol/ean/thirteen.rs
//
//! Implements EAN-13 for [`Ean<13>`][crate::Ean].
//

use super::_helper;
use crate::{Ean, is, lets, read_at, unwrap, whilst, write_at};

impl Ean<13> {
    /// Number of digits preceding the check digit.
    pub const DATA_DIGITS: usize = 12;

    /// Number of logical modules, excluding quiet zones.
    pub const MODULES: usize = 95;

    /// A/B selection for the six left-side characters.
    ///
    /// Bits 5..=0 correspond to the six characters from left to right:
    /// `0 = A`, `1 = B`.
    #[rustfmt::skip]
    const LEFT_B_MASK: [u8; 10] = [
        0b00_0000, // 0: AAAAAA -- UPC-A form; Ean13 rejects leading zero
        0b00_1011, // 1: AABABB
        0b00_1101, // 2: AABBAB
        0b00_1110, // 3: AABBBA
        0b01_0011, // 4: ABAABB
        0b01_1001, // 5: ABBAAB
        0b01_1100, // 6: ABBBAA
        0b01_0101, // 7: ABABAB
        0b01_0110, // 8: ABABBA
        0b01_1010, // 9: ABBABA
    ];

    /* digits */

    /// Calculates the mandatory EAN-13 check digit.
    ///
    /// Returns `None` if a value is not decimal or the leading digit is zero.
    #[must_use]
    pub const fn check_digit(data: [u8; 12]) -> Option<u8> {
        is! { data[0] == 0, return None }
        _helper::ean_check_digit(&data)
    }
    /// Appends the calculated check digit.
    ///
    /// Returns `None` if a value is not decimal or the leading digit is zero.
    #[must_use]
    pub const fn with_check_digit(data: [u8; 12]) -> Option<[u8; 13]> {
        let check = unwrap![some? Self::check_digit(data)];
        let mut digits = [0; 13];
        write_at![digits, 0, @data, check];
        Some(digits)
    }
    /// Returns whether `digits` form a valid EAN-13 sequence.
    #[must_use]
    pub const fn is_valid(digits: [u8; 13]) -> bool {
        is! { digits[0] == 0, return false }
        let mut data = [0; 12];
        read_at![digits, 0, @data]; // IMPROVE?
        unwrap![some_or Self::check_digit(data), check => check == digits[12], false]
    }

    /* encoding */

    /// Encodes valid EAN-13 digits into their 95 logical modules.
    #[must_use]
    pub const fn encode(digits: [u8; 13]) -> Option<u128> {
        is! { !Self::is_valid(digits), return None }
        let parity = Self::LEFT_B_MASK[digits[0] as usize];
        let mut modules = _helper::GUARD as u128;
        // Six left-side digits. The leading digit itself is not emitted.
        whilst! { i in 0..6; {
            let digit = digits[i + 1];
            let pattern = if parity & (1 << (5 - i)) != 0 {
                _helper::set_b(digit)
            } else {
                _helper::SET_A[digit as usize]
            };
            modules = (modules << 7) | pattern as u128;
        }}
        modules = (modules << 5) | _helper::CENTER as u128;
        // Six right-side digits, including the check digit.
        whilst! { i in 7..13; {
            modules = (modules << 7) | _helper::set_c(digits[i]) as u128;
        }}
        Some((modules << 3) | _helper::GUARD as u128)
    }

    /* decoding */

    /// Decodes an exact 95-module EAN-13 pattern.
    ///
    /// Returns `None` for malformed guards, invalid digit patterns,
    /// invalid A/B parity, excess high bits, or an invalid check digit.
    #[must_use]
    pub const fn decode(modules: u128) -> Option<[u8; 13]> {
        // Canonical representation: only the low 95 bits may be occupied.
        is! { modules >> Self::MODULES != 0, return None }
        // Guards occupy:
        // 94..92  left
        // 49..45  centre
        //  2..0   right
        is! {
            ((modules >> 92) & 0b111) != _helper::GUARD as u128
                || ((modules >> 45) & 0b1_1111) != _helper::CENTER as u128
                || (modules & 0b111) != _helper::GUARD as u128,
            return None
        }
        lets! { mut digits = [0_u8; 13], mut parity = 0_u8 }
        // Decode the six left characters and recover their A/B pattern.
        whilst! { i in 0..6; {
            let shift = 85 - i * 7;
            let pattern = ((modules >> shift) & 0x7F) as u8;
            if let Some(digit) = _helper::decode_a(pattern) {
                digits[i + 1] = digit;
            } else if let Some(digit) = _helper::decode_b(pattern) {
                digits[i + 1] = digit;
                parity |= 1 << (5 - i);
            } else {
                return None;
            }
        }}
        digits[0] = unwrap![some? Self::leading_digit(parity)];
        // Decode the six number-set-C characters on the right.
        whilst! { i in 0..6; {
            let shift = 38 - i * 7;
            let pattern = ((modules >> shift) & 0x7F) as u8;
            digits[i + 7] = unwrap![some? _helper::decode_c(pattern)];
        }}
        is! { Self::is_valid(digits), Some(digits), None }
    }

    /* private */

    /// Decodes the implicit leading digit from the six-character A/B pattern.
    const fn leading_digit(parity: u8) -> Option<u8> {
        // Start at 1 deliberately: AAAAAA is the UPC-A-compatible form.
        whilst! { digit in 1..Self::LEFT_B_MASK.len(); {
            is! { Self::LEFT_B_MASK[digit] == parity, return Some(digit as u8) }
        }}
        None
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    type Ean13 = Ean<13>;

    #[test]
    fn canonical_symbol() {
        let digits = [4, 0, 0, 6, 3, 8, 1, 3, 3, 3, 9, 3, 1];
        let expected = 0x51A9_D7BD_12CD_50A1_42E9_0B35;
        assert_eq!(Ean13::encode(digits), Some(expected));
        assert_eq!(Ean13::decode(expected), Some(digits));
    }
    #[test]
    fn roundtrip_all_leading_digits() {
        for leading in 1..=9 {
            let data = [leading, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
            let digits = Ean13::with_check_digit(data).unwrap();
            let modules = Ean13::encode(digits).unwrap();
            assert_eq!(Ean13::decode(modules), Some(digits));
        }
    }
    #[test]
    fn rejects_upc_a_form() {
        assert_eq!(Ean13::check_digit([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1]), None,);
        assert!(!Ean13::is_valid([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]));
        assert_eq!(Ean13::leading_digit(0b000000), None);
    }
}
