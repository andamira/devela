// devela/src/data/codec/symbol/braille.rs
//
//! Defines [`BrailleByte`].
//

use crate::{Str, charu, is, slice, unwrap, whilst, write_at};

#[doc = crate::_tags!(codec namespace)]
/// Reversibly maps bytes to Unicode Braille-pattern characters.
#[doc = crate::_doc_meta!{
    location("data/codec", struct BrailleByte),
    test_size_of(BrailleByte = 0),
}]
///
/// Each Braille cell uses 3 UTF-8 bytes.
/// ```text
/// b7 b3
/// b6 b2
/// b5 b1
/// b4 b0
/// ```
/// The mapping is reversible for all 256 byte values.
///
/// # Example
/// ```
/// use devela::BrailleByte;
///
/// let input = [0x00, 0x55, 0xAA, 0xFF];
/// let mut encoded = [0; 12];
///
/// let braille = BrailleByte::encode_to_slice(&input, &mut encoded).unwrap();
/// assert_eq!(braille, "⠀⣒⠭⣿");
///
/// let mut decoded = [0; 4];
/// assert_eq!(BrailleByte::decode_from_slice(braille, &mut decoded).unwrap(), input);
/// ```
#[derive(Debug)]
pub struct BrailleByte;

impl BrailleByte {
    const BASE: u32 = 0x2800;
    const END: u32 = 0x28FF;

    /// UTF-8 length of one encoded Braille cell.
    pub const UTF8_LEN: usize = 3;

    /* encoding */

    /// Returns the UTF-8 encoded length for `input_len` bytes.
    #[must_use]
    pub const fn encoded_len(input_len: usize) -> usize {
        unwrap![some_expect input_len.checked_mul(Self::UTF8_LEN), "encoded length overflow"]
    }
    /// Encodes `byte` as one Unicode Braille-pattern character.
    #[must_use]
    pub const fn encode(byte: u8) -> char {
        unwrap![some char::from_u32(
            Self::BASE + Self::byte_to_pattern(byte) as u32
        )]
    }
    /// Encodes `byte` as a UTF-8-backed Unicode scalar.
    pub const fn encode_charu(byte: u8) -> charu {
        charu::from_char(Self::encode(byte))
    }
    /// Encodes `input` as Braille-pattern UTF-8 into `output`.
    ///
    /// Returns the written prefix as a string slice,
    /// or `None` if `output` is too small.
    pub const fn encode_to_slice<'a>(input: &[u8], output: &'a mut [u8]) -> Option<&'a str> {
        let len = unwrap![some? input.len().checked_mul(Self::UTF8_LEN)];
        is! { output.len() < len, return None }
        let mut offset = 0;
        whilst! { i in 0..input.len(); {
            write_at![output, +=offset, #Self::encode(input[i])];
        }}
        unwrap![ok_some Str::from_utf8(slice![output, ..offset])]
    }

    /* decoding */

    /// Returns the decoded byte length for a structurally valid encoded length.
    #[must_use]
    pub const fn decoded_len(input_len: usize) -> Option<usize> {
        is! { input_len.is_multiple_of(Self::UTF8_LEN), Some(input_len / Self::UTF8_LEN), None }
    }
    /// Decodes a Unicode Braille-pattern character into its represented byte.
    ///
    /// Returns `None` if `ch` is outside `U+2800..=U+28FF`.
    #[must_use]
    pub const fn decode(ch: char) -> Option<u8> {
        let scalar = ch as u32;
        if scalar >= Self::BASE && scalar <= Self::END {
            Some(Self::pattern_to_byte((scalar - Self::BASE) as u8))
        } else {
            None
        }
    }
    /// Decodes Braille-pattern `input` into bytes.
    ///
    /// Returns the written output prefix, or `None` if `input` contains a
    /// non-Braille character or `output` is too small.
    pub const fn decode_from_slice<'a>(input: &str, output: &'a mut [u8]) -> Option<&'a [u8]> {
        is! { !input.len().is_multiple_of(Self::UTF8_LEN), return None }
        let len = input.len() / Self::UTF8_LEN;
        is! { output.len() < len, return None }
        let mut chars = Str::chars(input);
        let mut offset = 0;
        while let Some(ch) = chars.next_char() {
            output[offset] = unwrap![some? Self::decode(ch)];
            offset += 1;
        }
        Some(slice![output, ..offset])
    }

    /* private*/

    const fn byte_to_pattern(b: u8) -> u8 {
        ((b & 0x80) >> 7)
            | ((b & 0x40) >> 5)
            | ((b & 0x20) >> 3)
            | ((b & 0x10) << 2)
            | (b & 0x08)
            | ((b & 0x04) << 2)
            | ((b & 0x02) << 4)
            | ((b & 0x01) << 7)
    }
    const fn pattern_to_byte(p: u8) -> u8 {
        ((p & 0x01) << 7)
            | ((p & 0x02) << 5)
            | ((p & 0x04) << 3)
            | ((p & 0x40) >> 2)
            | (p & 0x08)
            | ((p & 0x10) >> 2)
            | ((p & 0x20) >> 4)
            | ((p & 0x80) >> 7)
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn canonical_patterns() {
        assert_eq!(BrailleByte::encode(0x00), '⠀');
        assert_eq!(BrailleByte::encode(0x01), '⢀');
        assert_eq!(BrailleByte::encode(0x02), '⠠');
        assert_eq!(BrailleByte::encode(0x04), '⠐');
        assert_eq!(BrailleByte::encode(0x08), '⠈');
        assert_eq!(BrailleByte::encode(0x10), '⡀');
        assert_eq!(BrailleByte::encode(0x20), '⠄');
        assert_eq!(BrailleByte::encode(0x40), '⠂');
        assert_eq!(BrailleByte::encode(0x80), '⠁');
        assert_eq!(BrailleByte::encode(0xFF), '⣿');
    }
    #[test]
    fn lengths() {
        assert_eq!(BrailleByte::encoded_len(0), 0);
        assert_eq!(BrailleByte::encoded_len(4), 12);
        assert_eq!(BrailleByte::decoded_len(12), Some(4));
        assert_eq!(BrailleByte::decoded_len(11), None);
    }
    #[test]
    fn scalar_roundtrip_all_bytes() {
        for byte in 0..=u8::MAX {
            assert_eq!(BrailleByte::decode(BrailleByte::encode(byte)), Some(byte));
        }
    }
    #[test]
    fn rejects_non_braille() {
        assert_eq!(BrailleByte::decode('\0'), None);
        assert_eq!(BrailleByte::decode('A'), None);
        assert_eq!(BrailleByte::decode('\u{27FF}'), None);
        assert_eq!(BrailleByte::decode('\u{2900}'), None);
    }
    #[test]
    fn slice_roundtrip() {
        let input = b"hello";
        let mut encoded = [0; 15];
        let braille = BrailleByte::encode_to_slice(input, &mut encoded).unwrap();
        assert_eq!(braille, "⠎⢖⠞⠞⢾");
        assert_eq!(braille.len(), BrailleByte::encoded_len(input.len()));
        let mut decoded = [0; 5];
        let bytes = BrailleByte::decode_from_slice(braille, &mut decoded).unwrap();
        assert_eq!(bytes, input);
    }
    #[test]
    fn slice_rejects_invalid_input_and_small_buffers() {
        let mut small = [0; 2];
        assert_eq!(BrailleByte::encode_to_slice(b"x", &mut small), None);
        let mut output = [0; 4];
        assert_eq!(BrailleByte::decode_from_slice("abc", &mut output), None);
        assert_eq!(BrailleByte::decode_from_slice("⠁A", &mut output), None);
        let mut none = [];
        assert_eq!(BrailleByte::decode_from_slice("⠁", &mut none), None);
    }
}
