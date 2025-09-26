// devela::text::char::impls::c
//
//!
//
// Note: `char` can only implement the trait, not the associated const methods.

use crate::{Char, UnicodeScalar};

impl UnicodeScalar for char {
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;

    /* encode */

    fn byte_len(self) -> usize {
        Char(self as u32).byte_len()
    }
    fn len_utf8(self) -> usize {
        self.len_utf8()
    }
    fn len_utf16(self) -> usize {
        self.len_utf16()
    }
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        self.encode_utf8(dst)
    }
    fn to_utf8_bytes(self) -> [u8; 4] {
        Char(self).to_utf8_bytes()
    }
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        self.encode_utf16(dst)
    }
    fn to_digit(self, radix: u32) -> Option<u32> {
        self.to_digit(radix)
    }
    fn to_ascii_uppercase(self) -> char {
        char::to_ascii_uppercase(&self)
    }
    fn to_ascii_lowercase(self) -> char {
        char::to_ascii_lowercase(&self)
    }

    /* queries */

    fn is_noncharacter(self) -> bool {
        Char(self as u32).is_noncharacter()
    }
    fn is_digit(self, radix: u32) -> bool {
        self.is_digit(radix)
    }
    fn is_control(self) -> bool {
        self.is_control()
    }
    fn is_nul(self) -> bool {
        self as u32 == 0
    }
    fn is_alphabetic(self) -> bool {
        self.is_alphabetic()
    }
    fn is_numeric(self) -> bool {
        self.is_numeric()
    }
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
    fn is_lowercase(self) -> bool {
        self.is_lowercase()
    }
    fn is_uppercase(self) -> bool {
        self.is_uppercase()
    }
    fn is_whitespace(self) -> bool {
        self.is_whitespace()
    }

    /* ascii queries*/

    fn is_ascii(self) -> bool {
        (self as u32) <= 0x7F
    }
}
