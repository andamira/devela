// devela/src/data/codec/symbol/tile/sextant.rs
//
//! Defines [`Sextant`].
//

use crate::{Str, charu, is, slice, unwrap, whilst, write_at};

#[doc = crate::_tags!(codec namespace)]
/// Reversibly maps 6-bit patterns to 2×3 Unicode sextant mosaics.
#[doc = crate::_doc_meta!{
    location("data/codec/symbol", struct Sextant),
    test_size_of(Sextant = 0),
}]
/// Bits map to filled sextants as follows:
/// ```text
/// b5 b2
/// b4 b1
/// b3 b0
/// ```
/// A cleared bit is empty and a set bit is filled.
///
/// The empty pattern is encoded as a space.
///
/// # Example
/// ```
/// # use devela::Sextant;
/// assert_eq!(Sextant::encode(0b000000), Some(' '));
/// assert_eq!(Sextant::encode(0b100000), Some('🬀'));
/// assert_eq!(Sextant::encode(0b111000), Some('▌'));
/// assert_eq!(Sextant::encode(0b111111), Some('█'));
/// assert_eq!(Sextant::decode('▐'), Some(0b000111));
/// ```
#[derive(Debug)]
pub struct Sextant;

impl Sextant {
    const BASE: u32 = 0x1FB00;
    const END: u32 = 0x1FB3B;

    /// Number of representable sextant patterns.
    pub const PATTERN_COUNT: usize = 64;

    /// Maximum UTF-8 length of one encoded sextant character.
    pub const MAX_UTF8_LEN: usize = 4;

    /* encode */

    /// Returns the UTF-8 encoded length of `input`.
    ///
    /// Returns `None` if any pattern is greater than `0b11_1111`.
    #[must_use]
    pub const fn encoded_len(input: &[u8]) -> Option<usize> {
        let mut len = 0_usize;
        whilst! { i in 0..input.len(); {
            let ch = unwrap![some? Self::encode(input[i])];
            len = unwrap![some? len.checked_add(ch.len_utf8())];
        }}
        Some(len)
    }
    /// Encodes a 2×3 sextant pattern.
    ///
    /// Returns `None` if `pattern > 0b11_1111`.
    #[must_use]
    pub const fn encode(pattern: u8) -> Option<char> {
        is! { pattern >= Self::PATTERN_COUNT as u8, return None }
        let mask = Self::pattern_to_unicode_mask(pattern);
        let scalar = match mask {
            0 => return Some(' '),
            21 => return Some('▌'), // sextants 1,3,5
            42 => return Some('▐'), // sextants 2,4,6
            63 => return Some('█'),
            1..=20 => Self::BASE + mask as u32 - 1,
            22..=41 => Self::BASE + mask as u32 - 2,
            43..=62 => Self::BASE + mask as u32 - 3,
            _ => return None,
        };
        char::from_u32(scalar)
    }
    /// Encodes a 2×3 sextant pattern as a UTF-8-backed Unicode scalar.
    #[must_use]
    pub const fn encode_charu(pattern: u8) -> Option<charu> {
        Some(charu::from_char(unwrap![some? Self::encode(pattern)]))
    }
    /// Encodes sextant patterns as UTF-8 characters into `output`.
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
    /// Decodes a Unicode sextant mosaic into its 2×3 pattern.
    ///
    /// A space decodes to the empty pattern.
    #[must_use]
    pub const fn decode(ch: char) -> Option<u8> {
        let scalar = ch as u32;
        let mask = match ch {
            ' ' => 0,
            '▌' => 21,
            '▐' => 42,
            '█' => 63,
            _ if scalar >= Self::BASE && scalar <= Self::END => {
                let index = (scalar - Self::BASE) as u8;
                is! { index < 20, index + 1, is! { index < 40, index + 2, index + 3 } }
            }
            _ => return None,
        };
        Some(Self::unicode_mask_to_pattern(mask))
    }
    /// Decodes Unicode sextant mosaics into patterns.
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

    /// Converts our column-major spatial pattern to Unicode sextant numbering.
    const fn pattern_to_unicode_mask(p: u8) -> u8 {
        ((p & 0b10_0000) >> 5)
            | ((p & 0b00_0100) >> 1)
            | ((p & 0b01_0000) >> 2)
            | ((p & 0b00_0010) << 2)
            | ((p & 0b00_1000) << 1)
            | ((p & 0b00_0001) << 5)
    }
    /// Converts Unicode sextant numbering to our column-major pattern.
    const fn unicode_mask_to_pattern(m: u8) -> u8 {
        ((m & 0b00_0001) << 5)
            | ((m & 0b00_0010) << 1)
            | ((m & 0b00_0100) << 2)
            | ((m & 0b00_1000) >> 2)
            | ((m & 0b01_0000) >> 1)
            | ((m & 0b10_0000) >> 5)
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn canonical_patterns() {
        assert_eq!(Sextant::encode(0b000000), Some(' '));
        // Individual subcells, top-to-bottom by columns.
        assert_eq!(Sextant::encode(0b100000), Some('🬀')); // 1: TL
        assert_eq!(Sextant::encode(0b000100), Some('🬁')); // 2: TR
        assert_eq!(Sextant::encode(0b010000), Some('🬃')); // 3: ML
        assert_eq!(Sextant::encode(0b000010), Some('🬇')); // 4: MR
        assert_eq!(Sextant::encode(0b001000), Some('🬏')); // 5: BL
        assert_eq!(Sextant::encode(0b000001), Some('🬞')); // 6: BR
        // Existing Block Elements reused by the sextant repertoire.
        assert_eq!(Sextant::encode(0b111000), Some('▌'));
        assert_eq!(Sextant::encode(0b000111), Some('▐'));
        assert_eq!(Sextant::encode(0b111111), Some('█'));
        // Horizontal thirds.
        assert_eq!(Sextant::encode(0b100100), Some('🬂'));
        assert_eq!(Sextant::encode(0b010010), Some('🬋'));
        assert_eq!(Sextant::encode(0b001001), Some('🬭'));
    }
    #[test]
    fn scalar_roundtrip_all_patterns() {
        for pattern in 0..64 {
            let ch = Sextant::encode(pattern).unwrap();
            assert_eq!(Sextant::decode(ch), Some(pattern));
        }
    }
    #[test]
    fn rejects_invalid() {
        assert_eq!(Sextant::encode(64), None);
        assert_eq!(Sextant::encode(u8::MAX), None);
        assert_eq!(Sextant::decode('A'), None);
        assert_eq!(Sextant::decode('▖'), None);
    }
    #[test]
    fn lengths() {
        assert_eq!(Sextant::encoded_len(&[]), Some(0));
        assert_eq!(Sextant::encoded_len(&[0]), Some(1));
        assert_eq!(Sextant::encoded_len(&[0b111000]), Some(3));
        assert_eq!(Sextant::encoded_len(&[0b100000]), Some(4));
        assert_eq!(Sextant::encoded_len(&[0, 0b111000, 0b100000]), Some(8));
    }
    #[test]
    fn slice_roundtrip_all_patterns() {
        let mut input = [0_u8; 64];
        let mut i = 0;
        while i < input.len() {
            input[i] = i as u8;
            i += 1;
        }
        let mut encoded = [0; 64 * Sextant::MAX_UTF8_LEN];
        let text = Sextant::encode_to_slice(&input, &mut encoded).unwrap();
        assert_eq!(Sextant::encoded_len(&input), Some(text.len()));
        let mut decoded = [0; 64];
        assert_eq!(Sextant::decode_from_slice(text, &mut decoded), Some(input.as_slice()));
    }
}
