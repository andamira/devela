// devela/src/data/codec/symbol/tile/quadrant.rs
//
//! Defines [`Quadrant`].
//

use crate::{Str, charu, is, slice, unwrap, whilst, write_at};

#[doc = crate::_tags!(codec namespace)]
/// Reversibly maps 4-bit patterns to 2×2 Unicode quadrant characters.
#[doc = crate::_doc_meta!{
    location("data/codec/symbol", struct Quadrant),
    test_size_of(Quadrant = 0),
}]
///
/// Bits map to filled quadrants as follows:
/// ```text
/// b3 b1
/// b2 b0
/// ```
/// A cleared bit is empty and a set bit is filled.
///
/// The empty pattern is encoded as a space.
///
/// # Example
/// ```
/// # use devela::Quadrant;
/// assert_eq!(Quadrant::encode(0b0000), Some(' '));
/// assert_eq!(Quadrant::encode(0b1000), Some('▘'));
/// assert_eq!(Quadrant::encode(0b0001), Some('▗'));
/// assert_eq!(Quadrant::encode(0b1111), Some('█'));
/// assert_eq!(Quadrant::decode('▚'), Some(0b1001));
/// ```
#[derive(Debug)]
pub struct Quadrant;

impl Quadrant {
    /// Number of representable quadrant patterns.
    pub const PATTERN_COUNT: usize = 16;

    /// Maximum UTF-8 length of one encoded quadrant character.
    pub const MAX_UTF8_LEN: usize = 3;

    /* encode */

    /// Returns the UTF-8 encoded length of `input`.
    ///
    /// Returns `None` if any pattern is greater than `0b1111`.
    #[must_use]
    pub const fn encoded_len(input: &[u8]) -> Option<usize> {
        let mut len = 0usize;
        whilst! { i in 0..input.len(); {
            let ch = unwrap![some? Self::encode(input[i])];
            len = unwrap![some? len.checked_add(ch.len_utf8())];
        }}
        Some(len)
    }
    /// Encodes a 2×2 quadrant pattern.
    ///
    /// Returns `None` if `pattern > 0b1111`.
    #[must_use]
    pub const fn encode(pattern: u8) -> Option<char> {
        if pattern < Self::PATTERN_COUNT as u8 {
            Some(Self::PATTERN_TO_CHAR[pattern as usize])
        } else {
            None
        }
    }
    /// Encodes a 2×2 quadrant pattern as a UTF-8-backed Unicode scalar.
    #[must_use]
    pub const fn encode_charu(pattern: u8) -> Option<charu> {
        Some(charu::from_char(unwrap![some? Self::encode(pattern)]))
    }
    /// Encodes quadrant patterns as UTF-8 characters into `output`.
    ///
    /// Returns the written prefix, or `None` if a pattern is invalid
    /// or `output` is too small.
    pub const fn encode_to_slice<'a>(input: &[u8], output: &'a mut [u8]) -> Option<&'a str> {
        let len = unwrap![some? Self::encoded_len(input)];
        is! { output.len() < len, return None }
        let mut offset = 0;
        whilst! { i in 0..input.len(); {
            let ch = unwrap![some? Self::encode(input[i])];
            write_at![output, +=offset, #ch];
        }}
        unwrap![ok_some Str::from_utf8(slice![output, ..offset])]
    }

    /* decode */

    /// Returns the decoded pattern count of `input`.
    ///
    /// Returns `None` if `input` contains a character outside this mapping.
    #[must_use]
    pub const fn decoded_len(input: &str) -> Option<usize> {
        let mut chars = Str::chars(input);
        let mut len = 0;
        while let Some(ch) = chars.next_char() {
            unwrap![some? Self::decode(ch)];
            len += 1;
        }
        Some(len)
    }
    /// Decodes a Unicode quadrant character into its 2×2 pattern.
    ///
    /// A space decodes to the empty pattern.
    #[must_use]
    pub const fn decode(ch: char) -> Option<u8> {
        whilst! { pattern in 0..Self::PATTERN_COUNT; {
            is! { Self::PATTERN_TO_CHAR[pattern] == ch, return Some(pattern as u8) }
        }}
        None
    }
    /// Decodes Unicode quadrant characters into patterns.
    ///
    /// Returns the written output prefix, or `None` if `input` contains
    /// another character or `output` is too small.
    pub const fn decode_from_slice<'a>(input: &str, output: &'a mut [u8]) -> Option<&'a [u8]> {
        let len = unwrap![some? Self::decoded_len(input)];
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

    /// 2×2 quadrant pattern → Unicode character.
    #[rustfmt::skip]
    const PATTERN_TO_CHAR: [char; 16] = [
        ' ', '▗', '▝', '▐',
        '▖', '▄', '▞', '▟',
        '▘', '▚', '▀', '▜',
        '▌', '▙', '▛', '█',
    ];
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn canonical_patterns() {
        const EXPECTED: [char; 16] =
            [' ', '▗', '▝', '▐', '▖', '▄', '▞', '▟', '▘', '▚', '▀', '▜', '▌', '▙', '▛', '█'];
        for pattern in 0..16 {
            assert_eq!(Quadrant::encode(pattern as u8), Some(EXPECTED[pattern]));
        }
    }
    #[test]
    fn scalar_roundtrip_all_patterns() {
        for pattern in 0..16 {
            let ch = Quadrant::encode(pattern).unwrap();
            assert_eq!(Quadrant::decode(ch), Some(pattern));
        }
    }
    #[test]
    fn rejects_invalid() {
        assert_eq!(Quadrant::encode(16), None);
        assert_eq!(Quadrant::encode(u8::MAX), None);
        assert_eq!(Quadrant::decode('A'), None);
        assert_eq!(Quadrant::decode('░'), None);
        assert_eq!(Quadrant::decode('●'), None);
    }
    #[test]
    fn lengths() {
        assert_eq!(Quadrant::encoded_len(&[]), Some(0));
        assert_eq!(Quadrant::encoded_len(&[0]), Some(1));
        assert_eq!(Quadrant::encoded_len(&[1]), Some(3));
        assert_eq!(Quadrant::encoded_len(&[0, 1, 15, 0]), Some(8));
        assert_eq!(Quadrant::decoded_len(""), Some(0));
        assert_eq!(Quadrant::decoded_len(" ▗█ "), Some(4));
        assert_eq!(Quadrant::decoded_len("A"), None);
    }
    #[test]
    fn slice_roundtrip_all_patterns() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut encoded = [0; 16 * Quadrant::MAX_UTF8_LEN];
        let text = Quadrant::encode_to_slice(&input, &mut encoded).unwrap();
        assert_eq!(text, " ▗▝▐▖▄▞▟▘▚▀▜▌▙▛█");
        assert_eq!(text.len(), 46);
        assert_eq!(Quadrant::encoded_len(&input), Some(text.len()));
        let mut decoded = [0; 16];
        assert_eq!(Quadrant::decode_from_slice(text, &mut decoded), Some(input.as_slice()));
    }
    #[test]
    fn slice_rejects_invalid_input_and_small_buffers() {
        let mut encoded = [0; 2];
        assert_eq!(Quadrant::encode_to_slice(&[1], &mut encoded), None);
        let mut encoded = [0; 8];
        assert_eq!(Quadrant::encode_to_slice(&[16], &mut encoded), None);
        let mut decoded = [0; 1];
        assert_eq!(Quadrant::decode_from_slice("▘▗", &mut decoded), None);
        let mut decoded = [0; 4];
        assert_eq!(Quadrant::decode_from_slice("▘A▗", &mut decoded), None);
    }
}
