// devela_base_core::media::image::sixel::char
//
//! Defines [`SixelChar`].
//

use crate::{Display, FmtResult, FmtWriter, Formatter, StringU8, TextLut, format_buf, is, unwrap};

#[doc = crate::_TAG_IMAGE!()]
#[doc = crate::_TAG_TERM!()]
/// A sixel character.
#[doc = crate::_doc_location!("media/image")]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SixelChar(u8);

impl SixelChar {
    const MASK: u8 = 0b111_111; // == 63

    /// Create a sixel character from the first 6 bits of the given `mask`.
    ///
    /// Bit 1 is the bottom pixel and bit 6 (== 32) is the top pixel.
    /// Bits 7 and 8 are ignored.
    #[inline(always)] #[rustfmt::skip]
    pub const fn from_bitmask(mask: u8) -> Self { Self(mask & Self::MASK) }

    /// Get the 6-bit bitmask representation.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn as_bitmask(self) -> u8 { self.0 }

    /// Get the sixel byte value.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn as_byte(self) -> u8 { self.0 + Self::MASK }

    /// Create a sixel character from a valid Unicode scalar, from '@' to '~'.
    pub const fn from_char(c: char) -> Option<Self> {
        let byte = c as u32;
        if byte >= Self::MASK as u32 && byte <= (Self::MASK + Self::MASK) as u32 {
            Some(Self((byte - Self::MASK as u32) as u8))
        } else {
            None
        }
    }
    /// Returns the corresponding Unicode scalar.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn as_char(self) -> char { self.as_byte() as char }

    /// Checks the equality of two chars.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn eq(self, other: Self) -> bool { self.0 == other.0 }

    /// Returns `true` if all the pixels are unset.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn is_empty(self) -> bool { self.eq(Self::EMPTY) }

    /// Returns `true` if all the pixels are set.
    #[must_use] #[inline(always)] #[rustfmt::skip]
    pub const fn is_full(self) -> bool { self.eq(Self::FULL) }

    /// Returns the next sixel character in sequence, wrapping around from 63 to 0.
    pub const fn next(self) -> Self {
        is![self.eq(Self::FULL); Self::EMPTY; Self(self.0 - 1)]
    }

    /// Returns the previous sixel character in sequence, wrapping around from 0 to 63.
    pub const fn prev(self) -> Self {
        is![self.eq(Self::EMPTY); Self::FULL; Self(self.0 - 1)]
    }

    /// Returns the next sixel character in sequence, returning None if at maximum.
    #[must_use]
    pub const fn next_checked(self) -> Option<Self> {
        is![self.eq(Self::FULL); None; Some(Self(self.0 + 1))]
    }

    /// Returns the previous sixel character in sequence, returning None if at minimum.
    #[must_use]
    pub const fn prev_checked(self) -> Option<Self> {
        is![self.eq(Self::EMPTY); None; Some(Self(self.0 - 1))]
    }
}

/// # Other conversions
impl SixelChar {
    /* bools */

    /// Create a sixel character from 6 pixels [top, 2, 3, 4, 5].
    #[allow(clippy::identity_op)]
    pub const fn from_bools(bools: [bool; 6]) -> Self {
        let value = (bools[0] as u8) * 32  // Top pixel = MSB
            + (bools[1] as u8) * 16
            + (bools[2] as u8) * 8
            + (bools[3] as u8) * 4
            + (bools[4] as u8) * 2
            + (bools[5] as u8) * 1; // Bottom pixel = LSB
        Self(value)
    }
    /// Extract the sixel bits as [top, 2, 3, 4, 5, bottom] bools.
    #[must_use]
    pub const fn to_bools(self) -> [bool; 6] {
        let value = self.as_bitmask();
        [
            value & 32 != 0, // Top pixel (MSB)
            value & 16 != 0,
            value & 8 != 0,
            value & 4 != 0,
            value & 2 != 0,
            value & 1 != 0, // Bottom pixel (LSB)
        ]
    }

    /* braille */

    /// Attempts to create a `SixelChar` from a Braille pattern Unicode character.
    ///
    /// Returns `Some(SixelChar)` if the character is in the Braille block (U+2800..=U+28FF), where
    /// dots 1-6 map to the sixel bits (dots 7-8 are ignored). Returns `None` for other characters.
    pub const fn from_braille(braille: char) -> Option<Self> {
        let b = braille as u32;
        if b >= 0x2800 && b <= 0x28FF {
            let bits = (b & Self::MASK as u32) as u8;
            Some(Self::from_bitmask(bits))
        } else {
            None
        }
    }
    /// Creates a `SixelChar` from any character, using the lower 6 bits of its codepoint.
    ///
    /// For Braille patterns (U+2800..=U+28FF), this preserves dots 1-6 and ignores dots 7-8.
    /// For other characters, the result may not be visually meaningful.
    pub const fn from_braille_unchecked(braille: char) -> Self {
        Self::from_bitmask((braille as u32 & Self::MASK as u32) as u8)
    }
    /// Converts this sixel to a Braille pattern character.
    ///
    /// The sixel bits map directly to Braille dots 1-6:
    /// - Bit 0 (top) → Dot 1
    /// - Bit 5 (bottom) → Dot 6
    #[must_use]
    pub const fn to_braille(self) -> char {
        unwrap![some char::from_u32(0x2800 + self.as_bitmask() as u32)]
    }

    /* hexagram */

    /// Creates a sixel from an I Ching hexagram character.
    ///
    /// - Returns `None` if the character is not in the hexagram range (U+4DC0..=U+4DFF).
    /// - Standard mapping: filled pixels represent broken lines.
    pub const fn from_hexagram(hexagram: char) -> Option<Self> {
        let h = hexagram as usize;
        if h >= 0x4DC0 && h <= 0x4DFF {
            Some(Self::from_bitmask(TextLut::CHAR_HEXAGRAM_TO_SIXEL[h - 0x4DC0]))
        } else {
            None
        }
    }
    /// Converts to an I Ching hexagram character using standard mapping.
    ///
    /// - Standard mapping: filled pixels represent broken lines.
    pub const fn to_hexagram(self) -> char {
        TextLut::SIXEL_TO_CHAR_HEXAGRAM[self.as_bitmask() as usize]
    }

    /// Creates a sixel from an I Ching hexagram character.
    ///
    /// - Returns `None` if the character is not in the hexagram range (U+4DC0..=U+4DFF).
    /// - Inverted mapping: filled pixels represent unbroken lines.
    pub const fn from_hexagram_inverted(hexagram: char) -> Option<Self> {
        let h = hexagram as usize;
        if h >= 0x4DC0 && h <= 0x4DFF {
            Some(Self::from_bitmask(!TextLut::CHAR_HEXAGRAM_TO_SIXEL[h - 0x4DC0]))
        } else {
            None
        }
    }
    /// Converts to an I Ching hexagram character using inverted mapping.
    ///
    /// - Inverted mapping: filled pixels represent unbroken lines.
    pub const fn to_hexagram_inverted(self) -> char {
        TextLut::SIXEL_TO_CHAR_HEXAGRAM[(!self.as_bitmask() & Self::MASK) as usize]
    }

    /* string */

    /// Converts this sixel to an ANSI-colored string showing multiple representations.
    ///
    /// The output displays:
    /// - The raw character (red)
    /// - Braille pattern equivalent (green)
    /// - Box representation (blue)
    /// - Hexagram character (cyan)
    /// - Binary bitmask representation
    ///
    /// Uses ANSI escape codes for coloring and resets formatting at the end.
    /// # Example
    /// ```
    /// # use devela_base_core::{Ansi, SixelChar, write_at};
    // /// let (mut offset, mut result) = (0, [0; 65]);
    // /// write_at![result, offset, @Ansi::BLACK_BG, @Ansi::RED, b'@', @'⠁'];
    /// assert_eq![
    ///     SixelChar::TOP.to_string_ansi(), // == "@⠁■□□□□□|000001"
    ///     "\u{1b}[40m\u{1b}[31m@\u{1b}[32m⠁\u{1b}[34m■□□□□□\u{1b}[0m\u{1b}[36m\u{1b}[0m|000001"
    /// ];
    /// ```
    #[allow(non_snake_case)]
    // TODO: make const to_string_ansi
    pub fn to_string_ansi(&self) -> StringU8<65> {
        use crate::{Ansi, lets};
        let mut buf = [0; 65];
        let c = self.as_char();
        let m = self.as_bitmask();
        let b = self.to_braille();
        // let h = self.to_hexagram(); // double-width
        let bx = self.to_string_box();
        lets![res = "\x1b[0m", @Ansi::{R=RED, B=BLUE, G=GREEN, W=CYAN, KB=BLACK_BG}];
        let args = format_args!["{KB}{R}{c}{G}{b}{B}{bx}{res}{W}{res}|{m:06b}"];
        let len = FmtWriter::format_len_unchecked(&mut buf, args);
        StringU8::<65>::_from_array_len_trusted(buf, len as u8)
    }

    /// Converts this sixel to a box representation showing pixel states.
    ///
    /// Each of the six pixels is represented as either:
    /// - `■` for filled/true pixels
    /// - `□` for empty/false pixels
    ///
    /// The output shows pixels from top to bottom.
    /// # Example
    /// ```
    /// # use devela_base_core::SixelChar;
    /// assert_eq![SixelChar::from_bitmask(0b111001).to_string_box(), "■□□■■■"];
    /// ```
    pub const fn to_string_box(&self) -> StringU8<20> {
        let mut string = StringU8::<20>::new();
        let mask = self.as_bitmask();
        let mut i = 0;
        while i < 6 {
            is![mask & (1 << i) != 0; string.push('■'); string.push('□')];
            i += 1;
        }
        assert![string.len() == 18];
        string
    }
}

/// # Constants
impl SixelChar {
    /// No pixels set. (?)
    pub const EMPTY: Self = Self::from_bitmask(0b000_000);
    /// All the six pixels set ().
    pub const FULL: Self = Self::from_bitmask(0b111_111);

    /// The bottom pixel (_).
    pub const BOT: Self = Self::from_bitmask(0b100_000); // MSB
    /// The 2 bottom pixels (o).
    pub const BOT2: Self = Self::from_bitmask(0b110_000);
    /// The 3 bottom pixels (w).
    pub const BOT3: Self = Self::from_bitmask(0b111_000);
    /// The 3 bottom pixels ({).
    pub const BOT4: Self = Self::from_bitmask(0b111_100);
    /// The 5 bottom pixels (}).
    pub const BOT5: Self = Self::from_bitmask(0b111_110);

    /// The top pixel (@).
    pub const TOP: Self = Self::from_bitmask(0b000_001); // LSB
    /// The 2 top pixels (B).
    pub const TOP2: Self = Self::from_bitmask(0b000_011);
    /// The 3 top pixels (F).
    pub const TOP3: Self = Self::from_bitmask(0b000_111);
    /// The 4 top pixels (N).
    pub const TOP4: Self = Self::from_bitmask(0b001_111);
    /// The 5 top pixels (^).
    pub const TOP5: Self = Self::from_bitmask(0b011_111);
}

impl Display for SixelChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let mut buf = [0u8; 16];
        let c = self.as_char();
        let m = self.as_bitmask();
        let b = self.to_braille();
        f.write_str(format_buf!(&mut buf, "[{c}{b}0b_{m:06b}]").unwrap())
    }
}

impl TextLut {
    /// Lookup table for fast conversion from sixel bitmask to I-Ching hexagram Unicode scalar.
    ///
    /// - Standard mapping: filled pixels represent broken lines.
    #[rustfmt::skip]
    // size: 256 bytes
    const SIXEL_TO_CHAR_HEXAGRAM: [char; 64] = [
        '\u{4DC0}','\u{4DEA}','\u{4DCD}','\u{4DE1}','\u{4DC8}','\u{4DC4}','\u{4DD9}','\u{4DCA}',
        '\u{4DC9}','\u{4DF9}','\u{4DE5}','\u{4DF5}','\u{4DFC}','\u{4DFA}','\u{4DE8}','\u{4DD2}',
        '\u{4DCC}','\u{4DF0}','\u{4DDD}','\u{4DF6}','\u{4DE4}','\u{4DFE}','\u{4DD5}','\u{4DE3}',
        '\u{4DD8}','\u{4DD0}','\u{4DD4}','\u{4DF2}','\u{4DE9}','\u{4DC2}','\u{4DDA}','\u{4DD7}',
        '\u{4DEB}','\u{4DDB}','\u{4DF1}','\u{4DDF}','\u{4DF8}','\u{4DEF}','\u{4DD1}','\u{4DED}',
        '\u{4DC5}','\u{4DEE}','\u{4DFF}','\u{4DE7}','\u{4DFB}','\u{4DDC}','\u{4DC3}','\u{4DC6}',
        '\u{4DE0}','\u{4DDE}','\u{4DF7}','\u{4DFD}','\u{4DF4}','\u{4DE6}','\u{4DF3}','\u{4DCE}',
        '\u{4DCB}','\u{4DEC}','\u{4DE2}','\u{4DCF}','\u{4DD3}','\u{4DC7}','\u{4DD6}','\u{4DC1}',
    ];

    /// Lookup table for fast conversion from I-Ching hexagram Unicode scalar to sixel bitmask.
    ///
    /// - Standard mapping: filled pixels represent broken lines.
    ///
    /// You have to subtract 0x4dc0 from char to get the index array.
    #[rustfmt::skip]
    // size: 64 bytes
    const CHAR_HEXAGRAM_TO_SIXEL: [u8; 64] = [
        // NEW ORDER, REVERSING THE BITS to have the correct top/down order
        0b_000000, 0b_111111, 0b_011101, 0b_101110, 0b_000101, 0b_101000, 0b_101111, 0b_111101,
        0b_000100, 0b_001000, 0b_000111, 0b_111000, 0b_010000, 0b_000010, 0b_110111, 0b_111011,
        0b_011001, 0b_100110, 0b_001111, 0b_111100, 0b_011010, 0b_010110, 0b_111110, 0b_011111,
        0b_011000, 0b_000110, 0b_011110, 0b_100001, 0b_101101, 0b_010010, 0b_110001, 0b_100011,
        0b_110000, 0b_000011, 0b_111010, 0b_010111, 0b_010100, 0b_001010, 0b_110101, 0b_101011,
        0b_001110, 0b_011100, 0b_000001, 0b_100000, 0b_111001, 0b_100111, 0b_101001, 0b_100101,
        0b_010001, 0b_100010, 0b_011011, 0b_110110, 0b_110100, 0b_001011, 0b_010011, 0b_110010,
        0b_100100, 0b_001001, 0b_101100, 0b_001101, 0b_001100, 0b_110011, 0b_010101, 0b_101010,
    ];

    #[cfg(test)]
    const _SIXEL_HEXAGRAM_CORRESPONDANCE: () = {
        let mut index = 0;
        while index < 64 {
            let bitmask = TextLut::CHAR_HEXAGRAM_TO_SIXEL[index];
            let char_from_bitmask = TextLut::SIXEL_TO_CHAR_HEXAGRAM[bitmask as usize];
            let char_code = char_from_bitmask as u32;
            let index_from_char = (char_code - 0x4DC0) as usize;

            // Verify that SIXEL_TO_CHAR_HEXAGRAM[bitmask] gives a character
            // whose code point corresponds to the original index
            assert!(index_from_char == index);

            // Verify that CHAR_HEXAGRAM_TO_SIXEL[index_from_char] gives back the original bitmask
            let roundtrip_bitmask = TextLut::CHAR_HEXAGRAM_TO_SIXEL[index_from_char];
            assert!(roundtrip_bitmask == bitmask);

            index += 1;
        }
    };
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn sixel_char() {
    // }
}
