// devela/src/media/visual/image/sixel/char.rs
//
//! Defines [`SixelChar`].
//

use crate::{Display, FmtResult, FmtWriter, Formatter, format_buf, is, unwrap};
use crate::{StringU8, YijingHexagram};

#[doc = crate::_tags!(image term)]
/// A sixel character.
#[doc = crate::_doc_meta!{
    location("media/visual/image", struct SixelChar),
    test_size_of(SixelChar = 1|8; niche !Option),
}]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SixelChar(u8);

#[rustfmt::skip]
impl SixelChar {
    const MASK: u8 = 0b111_111; // == 63

    /// Create a sixel character from the first 6 bits of the given `mask`.
    ///
    /// Bit 1 is the bottom pixel and bit 6 (== 32) is the top pixel.
    /// Bits 7 and 8 are ignored.
    pub const fn from_bitmask(mask: u8) -> Self { Self(mask & Self::MASK) }

    #[must_use]
    /// Get the 6-bit bitmask representation.
    pub const fn as_bitmask(self) -> u8 { self.0 }

    #[must_use]
    /// Get the sixel byte value.
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
    #[must_use]
    /// Returns the corresponding Unicode scalar.
    pub const fn as_char(self) -> char { self.as_byte() as char }

    #[must_use]
    /// Checks the equality of two chars.
    pub const fn eq(self, other: Self) -> bool { self.0 == other.0 }

    #[must_use]
    /// Returns `true` if all the pixels are unset.
    pub const fn is_empty(self) -> bool { self.eq(Self::EMPTY) }

    #[must_use]
    /// Returns `true` if all the pixels are set.
    pub const fn is_full(self) -> bool { self.eq(Self::FULL) }

    /// Returns the next sixel character in sequence, wrapping around from 63 to 0.
    pub const fn next(self) -> Self {
        is![self.eq(Self::FULL), Self::EMPTY, Self(self.0 - 1)]
    }

    /// Returns the previous sixel character in sequence, wrapping around from 0 to 63.
    pub const fn prev(self) -> Self {
        is![self.eq(Self::EMPTY), Self::FULL, Self(self.0 - 1)]
    }

    #[must_use]
    /// Returns the next sixel character in sequence, returning None if at maximum.
    pub const fn next_checked(self) -> Option<Self> {
        is![self.eq(Self::FULL), None, Some(Self(self.0 + 1))]
    }

    #[must_use]
    /// Returns the previous sixel character in sequence, returning None if at minimum.
    pub const fn prev_checked(self) -> Option<Self> {
        is![self.eq(Self::EMPTY), None, Some(Self(self.0 - 1))]
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
    #[must_use]
    /// Converts this sixel to a Braille pattern character.
    ///
    /// The sixel bits map directly to Braille dots 1-6:
    /// - Bit 0 (top) → Dot 1
    /// - Bit 5 (bottom) → Dot 6
    pub const fn to_braille(self) -> char {
        unwrap![some char::from_u32(0x2800 + self.as_bitmask() as u32)]
    }

    /* hexagram */

    /// Creates a sixel from an I Ching hexagram character.
    ///
    /// - Returns `None` if the character is not in the hexagram range (U+4DC0..=U+4DFF).
    /// - Standard mapping: filled pixels represent broken lines.
    pub const fn from_hexagram(hexagram: char) -> Option<Self> {
        unwrap![some_map YijingHexagram::decode(hexagram), |pat| Self::from_bitmask(pat)]
    }
    /// Converts to an I Ching hexagram character using standard mapping.
    ///
    /// - Standard mapping: filled pixels represent broken lines.
    pub const fn to_hexagram(self) -> char {
        unwrap![some YijingHexagram::encode(self.as_bitmask())]
    }

    /// Creates a sixel from an I Ching hexagram character.
    ///
    /// - Returns `None` if the character is not in the hexagram range (U+4DC0..=U+4DFF).
    /// - Inverted mapping: filled pixels represent unbroken lines.
    pub const fn from_hexagram_inverted(hexagram: char) -> Option<Self> {
        unwrap![some_map YijingHexagram::decode(hexagram), |pat| Self::from_bitmask(!pat)]
    }
    /// Converts to an I Ching hexagram character using inverted mapping.
    ///
    /// - Inverted mapping: filled pixels represent unbroken lines.
    pub const fn to_hexagram_inverted(self) -> char {
        unwrap![some YijingHexagram::encode(!self.as_bitmask() & Self::MASK)]
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
    /// # Examples
    /// ```ignore
    /// # use devela::{Ansi, SixelChar, write_at};
    /// let (mut offset, mut result) = (0, [0; 65]);
    /// write_at![result, +=offset, @Ansi::BLACK_BG, @Ansi::RED, b'@', @'⠁'];
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
    /// # Examples
    /// ```
    /// # use devela::SixelChar;
    /// assert_eq![SixelChar::from_bitmask(0b111001).to_string_box(), "■□□■■■"];
    /// ```
    pub const fn to_string_box(&self) -> StringU8<20> {
        let mut string = StringU8::<20>::new();
        let mask = self.as_bitmask();
        let mut i = 0;
        while i < 6 {
            is![mask & (1 << i) != 0, string.push('■'), string.push('□')];

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

#[cfg(test)]
mod _test {
    // use super::*;
    //
    // #[test]
    // fn sixel_char() {
    // }
}
