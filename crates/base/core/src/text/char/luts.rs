// devela_base_core::text::char::lut
//
//! Defines several ASCII LUTs.
//
// TOC
// - ASCII_CHARS
// - ASCII_BASE36_OFFSET
// - DECIMAL_PAIRS
// - DIGITS_BASE36
// - POWERS10

use crate::TextLut;

#[doc = concat!["# Unicode scalar related ", crate::_ABBR_LUT!(), "s."]]
impl TextLut {
    #[rustfmt::skip]
    /// Lookup table for fast ASCII code point to UTF-8 encoding.
    ///
    /// Used internally in [`Char`][crate::Char] and [`char7`][crate::char7].
    pub const ASCII_CHARS: [&str; 128] = [
        "\0", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\t", "\n",
        "\x0B", "\x0C", "\r", "\x0E", "\x0F", "\x10", "\x11", "\x12", "\x13", "\x14", "\x15",
        "\x16", "\x17", "\x18", "\x19", "\x1A", "\x1B", "\x1C", "\x1D", "\x1E", "\x1F",
        " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/",
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?",
        "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_",
        "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
        "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "\x7F",
    ];

    /// Lookup table for ASCII offsets in base36 encoding.
    ///
    /// Maps digits 0-36 to their ASCII offset from '0':
    /// - Digits 0-9: offset 0 (yields '0'-'9')
    /// - Digits 10-36: offset 7 (yields 'A'-'Z')
    ///
    /// Used internally in [`FontArt`].
    #[doc = crate::doclink!(custom devela "[`FontArt`]" "media/font/struct.FontArt.html")]
    pub const ASCII_BASE36_OFFSET: [u8; 37] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0-9: offset 0
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // 10-19: offset 7
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // 20-29: offset 7
        7, 7, 7, 7, 7, 7, 7, // 30-36: offset 7
    ];

    /// Precomputed two-digit decimal number strings (00-99).
    ///
    /// Used internally in [`Digits`][crate::Digits].
    pub const DECIMAL_PAIRS: &[u8; 200] = b"\
        0001020304050607080910111213141516171819\
        2021222324252627282930313233343536373839\
        4041424344454647484950515253545556575859\
        6061626364656667686970717273747576777879\
        8081828384858687888990919293949596979899";

    /// Lookup table for digit characters in bases 2 through 36.
    ///
    /// Used internally in [`Digits`][crate::Digits].
    pub const DIGITS_BASE36: [u8; 36] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    /// Precomputed powers of 10: `[10^0, 10^1, ..., 10^38]`
    ///
    /// Used internally in [`Digits`][crate::Digits].
    pub const POWERS10: [u128; 39] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
        10_000_000_000,
        100_000_000_000,
        1_000_000_000_000,
        10_000_000_000_000,
        100_000_000_000_000,
        1_000_000_000_000_000,
        10_000_000_000_000_000,
        100_000_000_000_000_000,
        1_000_000_000_000_000_000,
        10_000_000_000_000_000_000,
        100_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000_000_000_000_000,
        1_000_000_000_000_000_000_000_000_000_000_000_000,
        10_000_000_000_000_000_000_000_000_000_000_000_000,
        100_000_000_000_000_000_000_000_000_000_000_000_000,
    ];
    #[cfg(test)]
    const _POWERS10_ASSERT: () = {
        assert![size_of_val(&TextLut::POWERS10) == size_of::<u128>() * 39];
        assert![size_of_val(&TextLut::POWERS10) == 624];
    };
}
