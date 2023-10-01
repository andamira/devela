// devela::ascii::char::char
//
// `char` can only implement the trait, not the associated const methods,
// (and that's the main reason we have the Char32 wrapper type).

use super::*;

impl UnicodeScalar for char {
    const MAX: Self = Self::MAX;

    /* encode */

    #[inline]
    fn byte_len(self) -> usize {
        char_byte_len(self as u32)
    }
    #[inline]
    fn len_utf8(self) -> usize {
        self.len_utf8()
    }
    #[inline]
    fn len_utf16(self) -> usize {
        self.len_utf16()
    }
    #[inline]
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        self.encode_utf8(dst)
    }
    #[inline]
    fn to_utf8_bytes(self) -> [u8; 4] {
        Char32(self).to_utf8_bytes()
    }
    #[inline]
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        self.encode_utf16(dst)
    }
    #[inline]
    fn to_digit(self, radix: u32) -> Option<u32> {
        self.to_digit(radix)
    }
    #[inline]
    fn to_ascii_uppercase(self) -> char {
        char::to_ascii_uppercase(&self)
    }
    #[inline]
    fn to_ascii_lowercase(self) -> char {
        char::to_ascii_lowercase(&self)
    }

    /* queries */

    #[inline]
    fn is_noncharacter(self) -> bool {
        char_is_noncharacter(self as u32)
    }
    #[inline]
    fn is_digit(self, radix: u32) -> bool {
        self.is_digit(radix)
    }
    #[inline]
    fn is_control(self) -> bool {
        self.is_control()
    }
    #[inline]
    fn is_nul(self) -> bool {
        self as u32 == 0
    }
    #[inline]
    fn is_alphabetic(self) -> bool {
        self.is_alphabetic()
    }
    #[inline]
    fn is_numeric(self) -> bool {
        self.is_numeric()
    }
    #[inline]
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
    #[inline]
    fn is_lowercase(self) -> bool {
        self.is_lowercase()
    }
    #[inline]
    fn is_uppercase(self) -> bool {
        self.is_uppercase()
    }
    #[inline]
    fn is_whitespace(self) -> bool {
        self.is_whitespace()
    }

    /* ascii queries*/

    #[inline]
    fn is_ascii(self) -> bool {
        (self as u32) <= 0x7F
    }
}
