// devela::char::fns

/// Returns `true` if the given unicode scalar `code` is a 7bit ASCII code.
#[inline]
pub const fn char_is_7bit(code: u32) -> bool {
    code <= 0x7F
}

/// Returns the number of bytes necessary to store the given unicode scalar `code`.
#[inline]
pub const fn char_byte_len(code: u32) -> usize {
    match code {
        0..=0xFF => 1,
        0x100..=0xFFFF => 2,
        _ => 3,
    }
}

/// Returns `true` if the given unicode scalar `code` is a [noncharacter][0]
///
/// [0]: https://www.unicode.org/glossary/#noncharacter
#[inline]
pub const fn char_is_noncharacter(code: u32) -> bool {
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
/// utf-8 encoded in 2 bytes.
#[inline]
pub(crate) const fn char_utf8_2bytes_len(code: [u8; 2]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
}

/// Returns the number of bytes needed to store the given unicode scalar `code`,
/// utf-8 encoded in 3 bytes.
#[inline]
pub(crate) const fn char_utf8_3bytes_len(code: [u8; 3]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
}

/// Returns the number of bytes needed to store the given unicode scalar `code`,
/// utf-8 encoded in 4 bytes.
#[inline]
pub(crate) const fn char_utf8_4bytes_len(code: [u8; 4]) -> u8 {
    1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
        + ((code[3] > 0) & (code[3] & 0b1100_0000 != 0b1000_0000)) as u8
}
