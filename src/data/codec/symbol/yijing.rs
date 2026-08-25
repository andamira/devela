// devela/src/data/codec/symbol/yijing.rs
//
//! Defines [`YijingHexagram`].
//

use crate::{Str, charu, is, slice, unwrap, whilst, write_at};

#[doc = crate::_tags!(codec namespace)]
/// Reversibly maps 6-bit patterns to Yijing hexagram characters.
#[doc = crate::_doc_meta!{
    location("data/codec", struct YijingHexagram),
    test_size_of(YijingHexagram = 0),
}]
/// Bits represent the six lines from top to bottom:
/// ```text
/// b5
/// b4
/// b3
/// b2
/// b1
/// b0
/// ```
/// A cleared bit represents an unbroken line and a set bit a broken line.
///
/// # Example
/// ```
/// # use devela::YijingHexagram;
/// assert_eq!(YijingHexagram::encode(0b000000), Some('䷀'));
/// assert_eq!(YijingHexagram::encode(0b111111), Some('䷁'));
/// assert_eq!(YijingHexagram::decode('䷀'), Some(0b000000));
/// ```
///
/// See: <https://en.wikipedia.org/wiki/Yijing_Hexagram_Symbols>.
#[derive(Debug)]
pub struct YijingHexagram;

impl YijingHexagram {
    const BASE: u32 = 0x4DC0;
    const END: u32 = 0x4DFF;

    /// Number of representable six-line patterns.
    pub const PATTERN_COUNT: usize = 64;

    /// UTF-8 length of one encoded hexagram.
    pub const UTF8_LEN: usize = 3;

    /* encode */

    /// Returns the UTF-8 encoded length for `input_len` patterns.
    #[must_use]
    pub const fn encoded_len(input_len: usize) -> usize {
        unwrap![some_expect input_len.checked_mul(Self::UTF8_LEN), "encoded length overflow"]
    }
    /// Encodes a six-line bit pattern as a Yijing hexagram.
    ///
    /// Returns `None` if `pattern > 0b11_1111`.
    #[must_use]
    pub const fn encode(pattern: u8) -> Option<char> {
        is! { pattern >= Self::PATTERN_COUNT as u8, return None }
        char::from_u32(Self::BASE + Self::PATTERN_TO_INDEX[pattern as usize] as u32)
    }
    /// Encodes a six-line bit pattern as a UTF-8-backed Unicode scalar.
    #[must_use]
    pub const fn encode_charu(pattern: u8) -> Option<charu> {
        Some(charu::from_char(unwrap![some? Self::encode(pattern)]))
    }
    /// Encodes six-line patterns as UTF-8 Yijing hexagrams.
    ///
    /// Returns the written prefix, or `None` if a pattern is invalid
    /// or `output` is too small.
    pub const fn encode_to_slice<'a>(input: &[u8], output: &'a mut [u8]) -> Option<&'a str> {
        let len = unwrap![some? input.len().checked_mul(Self::UTF8_LEN)];
        is! { output.len() < len, return None }
        let mut offset = 0;
        whilst! { i in 0..input.len(); {
            let ch = unwrap![some? Self::encode(input[i])];
            write_at![output, +=offset, #ch];
        }}
        unwrap![ok_some Str::from_utf8(slice![output, ..offset])]
    }

    /* decode */

    /// Returns the decoded pattern count for an encoded UTF-8 length.
    #[must_use]
    pub const fn decoded_len(input_len: usize) -> Option<usize> {
        is! { input_len.is_multiple_of(Self::UTF8_LEN), Some(input_len / Self::UTF8_LEN), None }
    }
    /// Decodes a Yijing hexagram into its six-line bit pattern.
    #[must_use]
    pub const fn decode(ch: char) -> Option<u8> {
        let scalar = ch as u32;
        if scalar >= Self::BASE && scalar <= Self::END {
            Some(Self::INDEX_TO_PATTERN[(scalar - Self::BASE) as usize])
        } else {
            None
        }
    }
    /// Decodes Yijing hexagrams into six-line patterns.
    ///
    /// Returns the written output prefix, or `None` if `input` contains
    /// another character or `output` is too small.
    pub const fn decode_from_slice<'a>(input: &str, output: &'a mut [u8]) -> Option<&'a [u8]> {
        let len = unwrap![some? Self::decoded_len(input.len())];
        is! { output.len() < len, return None }
        let mut chars = Str::chars(input);
        let mut offset = 0;
        while let Some(ch) = chars.next_char() {
            output[offset] = unwrap![some? Self::decode(ch)];
            offset += 1;
        }
        Some(slice![output, ..offset])
    }

    /* private */

    /// Unicode-order hexagram index → six-line pattern.
    #[rustfmt::skip]
    const INDEX_TO_PATTERN: [u8; 64] = [
        0b_000000, 0b_111111, 0b_011101, 0b_101110, 0b_000101, 0b_101000, 0b_101111, 0b_111101,
        0b_000100, 0b_001000, 0b_000111, 0b_111000, 0b_010000, 0b_000010, 0b_110111, 0b_111011,
        0b_011001, 0b_100110, 0b_001111, 0b_111100, 0b_011010, 0b_010110, 0b_111110, 0b_011111,
        0b_011000, 0b_000110, 0b_011110, 0b_100001, 0b_101101, 0b_010010, 0b_110001, 0b_100011,
        0b_110000, 0b_000011, 0b_111010, 0b_010111, 0b_010100, 0b_001010, 0b_110101, 0b_101011,
        0b_001110, 0b_011100, 0b_000001, 0b_100000, 0b_111001, 0b_100111, 0b_101001, 0b_100101,
        0b_010001, 0b_100010, 0b_011011, 0b_110110, 0b_110100, 0b_001011, 0b_010011, 0b_110010,
        0b_100100, 0b_001001, 0b_101100, 0b_001101, 0b_001100, 0b_110011, 0b_010101, 0b_101010,
    ];

    /// Six-line pattern → Unicode-order hexagram index.
    const PATTERN_TO_INDEX: [u8; 64] = Self::invert_patterns();

    const fn invert_patterns() -> [u8; 64] {
        let mut inverse = [0; 64];
        whilst! { index in 0..64; {
            inverse[Self::INDEX_TO_PATTERN[index] as usize] = index as u8;
        }}
        inverse
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn canonical_patterns() {
        assert_eq!(YijingHexagram::encode(0b000000), Some('䷀'));
        assert_eq!(YijingHexagram::encode(0b111111), Some('䷁'));
        assert_eq!(YijingHexagram::decode('䷀'), Some(0b000000));
        assert_eq!(YijingHexagram::decode('䷁'), Some(0b111111));
    }
    #[test]
    fn scalar_roundtrip_all_patterns() {
        for pattern in 0..64 {
            let ch = YijingHexagram::encode(pattern).unwrap();
            assert_eq!(YijingHexagram::decode(ch), Some(pattern));
        }
    }
    #[test]
    fn rejects_invalid() {
        assert_eq!(YijingHexagram::encode(64), None);
        assert_eq!(YijingHexagram::encode(u8::MAX), None);
        assert_eq!(YijingHexagram::decode('A'), None);
        assert_eq!(YijingHexagram::decode('\u{4DBF}'), None);
        assert_eq!(YijingHexagram::decode('\u{4E00}'), None);
    }
    #[test]
    fn slice_roundtrip() {
        let input = [0, 1, 2, 3, 31, 63];
        let mut encoded = [0; 18];
        let text = YijingHexagram::encode_to_slice(&input, &mut encoded).unwrap();
        assert_eq!(text.len(), YijingHexagram::encoded_len(input.len()));
        let mut decoded = [0; 6];
        assert_eq!(YijingHexagram::decode_from_slice(text, &mut decoded), Some(input.as_slice()));
    }
    #[test]
    fn lengths() {
        assert_eq!(YijingHexagram::encoded_len(4), 12);
        assert_eq!(YijingHexagram::decoded_len(12), Some(4));
        assert_eq!(YijingHexagram::decoded_len(11), None);
    }
}
