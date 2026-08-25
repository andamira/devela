// devela/src/data/codec/symbol/tile/octant.rs
//
//! Defines [`Octant`].
//

use crate::{Str, charu, is, slice, unwrap, whilst, write_at};

#[doc = crate::_tags!(codec namespace)]
/// Reversibly maps bytes to 2×4 Unicode octant tiles.
#[doc = crate::_doc_meta!{
    location("data/codec/symbol", struct Octant),
    test_size_of(Octant = 0),
}]
/// Bits map to filled octants as follows:
/// ```text
/// b7 b3
/// b6 b2
/// b5 b1
/// b4 b0
/// ```
/// A cleared bit is empty and a set bit is filled.
///
/// The empty pattern is encoded as a space.
/// The mapping is reversible for all 256 byte values.
///
/// # Example
/// ```
/// # use devela::Octant;
/// assert_eq!(Octant::encode(0x00), ' ');
/// assert_eq!(Octant::encode(0xF0), '▌');
/// assert_eq!(Octant::encode(0x0F), '▐');
/// assert_eq!(Octant::encode(0xFF), '█');
/// assert_eq!(Octant::decode('█'), Some(0xFF));
/// ```
#[derive(Debug)]
pub struct Octant;

impl Octant {
    const BASE: u32 = 0x1CD00;
    const END: u32 = 0x1CDE5;

    /// Number of representable octant patterns.
    pub const PATTERN_COUNT: usize = 256;

    /// Maximum UTF-8 length of one encoded octant character.
    pub const MAX_UTF8_LEN: usize = 4;

    /* encode */

    /// Returns the UTF-8 encoded length of `input`.
    #[must_use]
    pub const fn encoded_len(input: &[u8]) -> usize {
        unwrap![some_expect Self::encoded_len_checked(input), "encoded length overflow"]
    }
    /// Encodes a 2×4 octant pattern.
    #[must_use]
    pub const fn encode(pattern: u8) -> char {
        let mask = Self::pattern_to_unicode_mask(pattern);
        is! { let Some(ch) = Self::reused_char(mask), return ch }
        unwrap![some char::from_u32( Self::BASE + Self::dedicated_index(mask) as u32)]
    }
    /// Encodes a 2×4 octant pattern as a UTF-8-backed Unicode scalar.
    pub const fn encode_charu(pattern: u8) -> charu {
        charu::from_char(Self::encode(pattern))
    }
    /// Encodes octant patterns as UTF-8 characters into `output`.
    ///
    /// Returns the written prefix, or `None` if `output` is too small.
    pub const fn encode_to_slice<'a>(input: &[u8], output: &'a mut [u8]) -> Option<&'a str> {
        let len = unwrap![some? Self::encoded_len_checked(input)];
        is! { output.len() < len, return None }
        let mut offset = 0;
        whilst! { i in 0..input.len(); {
            write_at![output, +=offset, #Self::encode(input[i])];
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
    /// Decodes a Unicode octant tile into its 2×4 pattern.
    ///
    /// A space decodes to the empty pattern.
    #[must_use]
    pub const fn decode(ch: char) -> Option<u8> {
        if let Some(mask) = Self::reused_mask(ch) {
            return Some(Self::unicode_mask_to_pattern(mask));
        }
        let scalar = ch as u32;
        if scalar >= Self::BASE && scalar <= Self::END {
            let index = (scalar - Self::BASE) as u8;
            let mask = Self::mask_from_dedicated_index(index);
            Some(Self::unicode_mask_to_pattern(mask))
        } else {
            None
        }
    }
    /// Decodes Unicode octant tiles into patterns.
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

    const fn encoded_len_checked(input: &[u8]) -> Option<usize> {
        let mut len = 0_usize;
        whilst! { i in 0..input.len(); {
            len = unwrap![some? len.checked_add(Self::encode(input[i]).len_utf8())];
        }}
        Some(len)
    }
    /// Converts our column-major spatial pattern to Unicode octant numbering.
    const fn pattern_to_unicode_mask(p: u8) -> u8 {
        ((p & 0b1000_0000) >> 7)
            | ((p & 0b0000_1000) >> 2)
            | ((p & 0b0100_0000) >> 4)
            | ((p & 0b0000_0100) << 1)
            | ((p & 0b0010_0000) >> 1)
            | ((p & 0b0000_0010) << 4)
            | ((p & 0b0001_0000) << 2)
            | ((p & 0b0000_0001) << 7)
    }
    /// Converts Unicode octant numbering to our column-major pattern.
    const fn unicode_mask_to_pattern(m: u8) -> u8 {
        ((m & 0b0000_0001) << 7)
            | ((m & 0b0000_0010) << 2)
            | ((m & 0b0000_0100) << 4)
            | ((m & 0b0000_1000) >> 1)
            | ((m & 0b0001_0000) << 1)
            | ((m & 0b0010_0000) >> 4)
            | ((m & 0b0100_0000) >> 2)
            | ((m & 0b1000_0000) >> 7)
    }
    /// Returns the character reused for a Unicode octant mask.
    const fn reused_char(mask: u8) -> Option<char> {
        whilst! { i in 0..Self::REUSED.len(); {
            let (candidate, ch) = Self::REUSED[i];
            is! { candidate == mask, return Some(ch) }
            is! { candidate > mask, break }
        }}
        None
    }
    /// Returns the Unicode octant mask represented by a reused character.
    const fn reused_mask(ch: char) -> Option<u8> {
        whilst! { i in 0..Self::REUSED.len(); {
            let (mask, candidate) = Self::REUSED[i];
            is! { candidate == ch, return Some(mask) }
        }}
        None
    }
    /// Returns the offset of a non-reused mask in the dedicated Unicode range.
    const fn dedicated_index(mask: u8) -> u8 {
        let mut skipped = 0_u8;
        whilst! { i in 0..Self::REUSED.len(); {
            let reused = Self::REUSED[i].0;
            is! { reused >= mask, break }
            skipped += 1;
        }}
        mask - skipped
    }
    /// Restores a Unicode octant mask from its dedicated-range offset.
    const fn mask_from_dedicated_index(index: u8) -> u8 {
        let mut mask = index;
        whilst! { i in 0..Self::REUSED.len(); {
            let reused = Self::REUSED[i].0;
            is! { reused <= mask, mask += 1, break }
        }}
        mask
    }
    /// Octant masks represented by characters outside the dedicated range.
    ///
    /// Masks use Unicode's row-major octant numbering:
    /// ```text
    /// 1 2
    /// 3 4
    /// 5 6
    /// 7 8
    /// ```
    #[rustfmt::skip]
    const REUSED: [(u8, char); 26] = [
        (0x00, ' '),
        (0x01, '\u{1CEA8}'),
        (0x02, '\u{1CEAB}'),
        (0x03, '\u{1FB82}'),
        (0x05, '▘'),
        (0x0A, '▝'),
        (0x0F, '▀'),
        (0x14, '\u{1FBE6}'),
        (0x28, '\u{1FBE7}'),
        (0x3F, '\u{1FB85}'),
        (0x40, '\u{1CEA3}'),
        (0x50, '▖'),
        (0x55, '▌'),
        (0x5A, '▞'),
        (0x5F, '▛'),
        (0x80, '\u{1CEA0}'),
        (0xA0, '▗'),
        (0xA5, '▚'),
        (0xAA, '▐'),
        (0xAF, '▜'),
        (0xC0, '▂'),
        (0xF0, '▄'),
        (0xF5, '▙'),
        (0xFA, '▟'),
        (0xFC, '▆'),
        (0xFF, '█'),
    ];
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn canonical_patterns() {
        assert_eq!(Octant::encode(0x00), ' ');
        // Individual subcells, top-to-bottom by columns.
        assert_eq!(Octant::encode(0b1000_0000), '\u{1CEA8}'); // 1: TL
        assert_eq!(Octant::encode(0b0000_1000), '\u{1CEAB}'); // 2: TR
        assert_eq!(Octant::encode(0b0100_0000), '\u{1CD00}'); // 3
        assert_eq!(Octant::encode(0b0000_0100), '\u{1CD03}'); // 4
        assert_eq!(Octant::encode(0b0010_0000), '\u{1CD09}'); // 5
        assert_eq!(Octant::encode(0b0000_0010), '\u{1CD18}'); // 6
        assert_eq!(Octant::encode(0b0001_0000), '\u{1CEA3}'); // 7: BL
        assert_eq!(Octant::encode(0b0000_0001), '\u{1CEA0}'); // 8: BR
        // Existing block characters reused by the octant repertoire.
        assert_eq!(Octant::encode(0xF0), '▌');
        assert_eq!(Octant::encode(0x0F), '▐');
        assert_eq!(Octant::encode(0xCC), '▀');
        assert_eq!(Octant::encode(0x33), '▄');
        assert_eq!(Octant::encode(0xFF), '█');
        // Bounds of the dedicated range.
        assert_eq!(Octant::encode(0x40), '\u{1CD00}');
        assert_eq!(Octant::encode(0x7F), '\u{1CDE5}');
    }
    #[test]
    fn scalar_roundtrip_all_patterns() {
        for pattern in u8::MIN..=u8::MAX {
            let ch = Octant::encode(pattern);
            assert_eq!(Octant::decode(ch), Some(pattern));
        }
    }
    #[test]
    fn rejects_invalid() {
        assert_eq!(Octant::decode('A'), None);
        assert_eq!(Octant::decode('●'), None);
        assert_eq!(Octant::decode('\u{1CCFF}'), None);
        assert_eq!(Octant::decode('\u{1CDE6}'), None);
    }
    #[test]
    fn lengths() {
        assert_eq!(Octant::encoded_len(&[]), 0);
        assert_eq!(Octant::encoded_len(&[0x00]), 1); // space
        assert_eq!(Octant::encoded_len(&[0xF0]), 3); // ▌
        assert_eq!(Octant::encoded_len(&[0x80]), 4); // U+1CEA8
        assert_eq!(Octant::encoded_len(&[0x40]), 4); // U+1CD00
        assert_eq!(Octant::encoded_len(&[0x00, 0xF0, 0x80]), 8);
    }
    #[test]
    fn slice_roundtrip_all_patterns() {
        let mut input = [0_u8; 256];
        let mut i = 0;
        while i < input.len() {
            input[i] = i as u8;
            i += 1;
        }
        let mut encoded = [0; 256 * Octant::MAX_UTF8_LEN];
        let text = Octant::encode_to_slice(&input, &mut encoded).unwrap();
        assert_eq!(Octant::encoded_len(&input), text.len());
        let mut decoded = [0; 256];
        assert_eq!(Octant::decode_from_slice(text, &mut decoded), Some(input.as_slice()));
    }
    #[test]
    fn slice_rejects_invalid_input_and_small_buffers() {
        let mut encoded = [0; 3];
        assert_eq!(Octant::encode_to_slice(&[0x80], &mut encoded), None);
        let mut decoded = [0; 1];
        assert_eq!(Octant::decode_from_slice("▌▐", &mut decoded), None);
        let mut decoded = [0; 4];
        assert_eq!(Octant::decode_from_slice("▌A▐", &mut decoded), None);
    }
}
