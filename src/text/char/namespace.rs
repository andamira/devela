// devela::text::char::namespace
//
//! `Char` namespace.
//

#[doc = crate::TAG_NAMESPACE!()]
/// Unicode scalars-related operations.
///
/// See also [`ExtMem`][crate::ExtMem],
pub struct Char;

impl Char {
    /// Returns the number of bytes necessary to store the given unicode scalar `code`.
    #[must_use]
    pub const fn byte_len(code: u32) -> usize {
        match code {
            0..=0xFF => 1,
            0x100..=0xFFFF => 2,
            _ => 3,
        }
    }

    /// Returns `true` if the given unicode scalar `code` is a 7bit ASCII code.
    #[must_use]
    pub const fn is_7bit(code: u32) -> bool {
        code <= 0x7F
    }

    /// Returns `true` if the given unicode scalar `code` is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    // FIXME: make a version that checks for surrogates
    pub const fn is_noncharacter(code: u32) -> bool {
        // sub-block of 32 non-characters:
        (code >= 0xFDD0 && code <= 0xFDEF)
            // 2× non-characters at the end of each plane:
            || (code >= 0xFFFE && (code & 0xFF) == 0xFE)
            || (code >= 0xFFFE && (code & 0xFF) == 0xFF)
            // unallocated range (16 potential non-characters):
            || (code >= 0x2FE0 && code <= 0x2FEF)
        // surrogates (0xD800..=0xDFFF) are already filtered out in `char`.
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 2 bytes.
    #[must_use]
    pub const fn utf8_2bytes_len(code: [u8; 2]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 3 bytes.
    #[must_use]
    pub const fn utf8_3bytes_len(code: [u8; 3]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 4 bytes.
    #[must_use]
    pub const fn utf8_4bytes_len(code: [u8; 4]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[3] > 0) & (code[3] & 0b1100_0000 != 0b1000_0000)) as u8
    }

    /// Returns the number of bytes needed to encode the given unicode scalar `code` as UTF-8
    #[must_use] #[rustfmt::skip]
    pub const fn len_to_utf8(code: char) -> usize {
        let code = code as u32;
        if code < 0x80 { 1 } else if code < 0x800 { 2 } else if code < 0x10_000 { 3 } else { 4 }
    }

    /// Converts this `char` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(c: char) -> [u8; 4] {
        let c = c as u32;
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (0b0011_1111 & (c as u8));
                let z = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let y = 0b10_000000 | ((c >> 12) & 0b0011_1111) as u8;
                let x = 0b11110_000 | ((c >> 18) as u8);
                [x, y, z, w]
            }
        }
    }
}
