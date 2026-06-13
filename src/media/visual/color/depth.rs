// devela/src/media/visual/color/depth.rs
//
//! Defines [`ColorDepth`].
//

#[doc = crate::_tags!(color)]
/// Color vocabulary depth.
#[doc = crate::_doc_meta!{location("media/visual/color")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ColorDepth {
    /// No selectable color.
    #[default]
    Mono = 0,
    /// 2-color palette, 1 bit.
    Palette2 = 1,
    /// 8-color palette, 3 bits.
    Palette8 = 3,
    /// 16-color palette, 4 bits.
    Palette16 = 4,
    /// 256-color palette, 8 bits.
    Palette256 = 8,
    /// 16-bit RGB color, commonly RGB565.
    Rgb16 = 16,
    /// 24-bit RGB color.
    Rgb24 = 24,
}
#[allow(non_upper_case_globals)]
impl ColorDepth {
    /// An alias of `Palette8`, matching 3-bit ANSI color.
    pub const Ansi3: Self = Self::Palette8;
    /// An alias of `Palette16`, matching 4-bit ANSI color.
    pub const Ansi4: Self = Self::Palette16;
    /// An alias of `Palette256`, matching 8-bit ANSI color.
    pub const Ansi8: Self = Self::Palette256;
    /// An alias of `Rgb16`, commonly called high color.
    pub const High16: Self = Self::Rgb16;
    /// An alias of `Rgb24`, commonly called true color.
    pub const True24: Self = Self::Rgb24;
}
impl ColorDepth {
    /// Returns `true` if this supports any selectable color.
    #[must_use]
    pub const fn has_color(self) -> bool {
        !matches!(self, Self::Mono)
    }
    /// Returns `true` if this supports direct RGB color.
    #[must_use]
    pub const fn has_rgb(self) -> bool {
        matches!(self, Self::Rgb16 | Self::Rgb24)
    }
    /// Returns whether this depth can represent at least `other`.
    #[must_use]
    pub const fn supports(self, other: Self) -> bool {
        self as u8 >= other as u8
    }
    /// Returns the greatest supported depth not exceeding `bits`.
    ///
    /// For example, `7` truncates to [`Palette16`][Self::Palette16],
    /// and `23` truncates to [`Rgb16`][Self::Rgb16].
    #[must_use]
    pub const fn from_bits_trunc(bits: u8) -> Self {
        match bits {
            24..=u8::MAX => Self::Rgb24,
            16..=23 => Self::Rgb16,
            8..=15 => Self::Palette256,
            4..=7 => Self::Palette16,
            3 => Self::Palette8,
            1..=2 => Self::Palette2,
            _ => Self::Mono,
        }
    }
    /// Returns the nominal bit depth.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self as u8
    }
}
