// devela::lex::char::fns

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
