// devela::sys::os::term::grid::color
//
//! Defines , [`TermColorKind`], [`TermColorMode`], [`TermColor`], [`TermColors`].
//

use crate::{AnsiColor, AnsiColor3, AnsiColor8};

#[doc = crate::_tags!(term color)]
/// The stored representation of a terminal color.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
#[repr(u8)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TermColorKind {
    /// Uses the terminal's default color.
    #[default]
    Default = 0,
    /// Uses an indexed terminal palette color.
    Indexed = 1,
    /// Uses a 24-bit RGB color.
    Rgb = 2,
    /// Reserved for future extension.
    Reserved = 3,
}
impl TermColorKind {
    /// Creates a color kind from its packed representation.
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::Indexed,
            2 => Self::Rgb,
            _ => Self::Reserved,
        }
    }
}

#[doc = crate::_tags!(term color)]
/// The composition mode of one terminal color.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
#[repr(u8)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TermColorMode {
    /// Replaces the color beneath it.
    #[default]
    Opaque = 0,
    /// Blends with the color beneath it.
    Blend = 1,
    /// Preserves the color beneath it.
    Transparent = 2,
    /// Selects a contrasting color from the resolved background.
    Contrast = 3,
}
impl TermColorMode {
    /// Creates a color mode from its packed representation.
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Opaque,
            1 => Self::Blend,
            2 => Self::Transparent,
            _ => Self::Contrast,
        }
    }
}

crate::bitfield! {
    #[doc = crate::_tags!(term color bit)]
    /// A packed terminal color and its composition mode.
    #[doc = crate::_doc_meta!{
        location("sys/os/term"),
        test_size_of(TermColor = 4|32),
    }]
    ///
    /// The low 24 bits store either an RGB value or an indexed palette value.
    /// The remaining defined bits select the representation and composition mode.
    #[must_use]
    pub struct TermColor(u32) {
        /// RGB or indexed-color payload.
        VALUE = 0..=23;
        /// [`TermColorKind`] discriminant.
        KIND = 24..=25;
        /// [`TermColorMode`] discriminant.
        MODE = 26..=27;
    }
    impl {
        /// Uses the terminal's default color.
        pub const DEFAULT: Self = Self::new();

        /// Creates an opaque indexed terminal color.
        pub const fn indexed(index: u8) -> Self {
            Self::new().with_value(index as u32).with_kind(TermColorKind::Indexed as u32)
        }
        /// Creates an opaque 24-bit RGB terminal color.
        pub const fn rgb(rgb: [u8; 3]) -> Self {
            let [r, g, b] = rgb;
            let value = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
            Self::new().with_value(value).with_kind(TermColorKind::Rgb as u32)
        }
        /// Creates a color using the terminal default and the given mode.
        pub const fn default_with_mode(mode: TermColorMode) -> Self {
            Self::new().with_mode(mode as u32)
        }
        /// Returns the stored color representation.
        pub const fn kind(self) -> TermColorKind {
            TermColorKind::from_u8(self.get_kind() as u8)
        }
        /// Returns the color's composition mode.
        pub const fn mode(self) -> TermColorMode {
            TermColorMode::from_u8(self.get_mode() as u8)
        }
        /// Returns this color with the given composition mode.
        pub const fn with_color_mode(self, mode: TermColorMode) -> Self {
            self.with_mode(mode as u32)
        }
        /// Returns whether this uses the terminal's default color.
        #[must_use]
        pub const fn is_default(self) -> bool { matches!(self.kind(), TermColorKind::Default) }

        /// Returns whether this uses an indexed terminal color.
        #[must_use]
        pub const fn is_indexed(self) -> bool { matches!(self.kind(), TermColorKind::Indexed) }

        /// Returns whether this uses a 24-bit RGB color.
        #[must_use]
        pub const fn is_rgb(self) -> bool { matches!(self.kind(), TermColorKind::Rgb) }

        /// Returns whether this color is opaque.
        #[must_use]
        pub const fn is_opaque(self) -> bool { matches!(self.mode(), TermColorMode::Opaque) }

        /// Returns whether this color preserves the color beneath it.
        #[must_use]
        pub const fn is_transparent(self) -> bool {
            matches!(self.mode(), TermColorMode::Transparent)
        }
        /// Returns the indexed palette value, when applicable.
        #[must_use]
        pub const fn index(self) -> Option<u8> {
            if self.is_indexed() { Some(self.get_value() as u8) } else { None }
        }
        /// Returns the RGB components, when applicable.
        #[must_use]
        pub const fn rgb_components(self) -> Option<[u8; 3]> {
            if self.is_rgb() {
                let value = self.get_value();
                Some([(value >> 16) as u8, (value >> 8) as u8, value as u8])
            } else {
                None
            }
        }
        /// Returns whether all currently reserved bits are clear.
        #[must_use]
        pub const fn is_canonical(self) -> bool {
            self.bits() & !Self::mask() == 0 && !matches!(self.kind(), TermColorKind::Reserved)
        }
        /// Returns this value with currently reserved bits cleared.
        pub const fn canonicalized(self) -> Self { Self::from_bits(self.bits() & Self::mask()) }

        /// Converts an ANSI color into a packed terminal color.
        ///
        /// Returns `None` for [`AnsiColor::None`], which represents absence
        /// rather than a resolved terminal color.
        #[must_use]
        pub const fn from_ansi(color: AnsiColor) -> Option<Self> {
            match color {
                AnsiColor::None => None,
                AnsiColor::Default => Some(Self::DEFAULT),
                AnsiColor::Dark(color) => Some(Self::indexed(color as u8)),
                AnsiColor::Bright(color) => Some(Self::indexed(color as u8 + 8)),
                AnsiColor::Palette(color) => Some(Self::indexed(color.0)),
                AnsiColor::Rgb(rgb) => Some(Self::rgb(rgb)),
            }
        }
        /// Converts an opaque color into its ANSI representation.
        ///
        /// Non-opaque and reserved colors return `None`.
        #[must_use]
        pub const fn to_ansi(self) -> Option<AnsiColor> {
            if !self.is_opaque() { return None; }
            match self.kind() {
                TermColorKind::Default => Some(AnsiColor::Default),
                TermColorKind::Indexed => {
                    let index = self.get_value() as u8;
                    match index {
                        0..=7 => Some(AnsiColor::Dark(AnsiColor3::from_u8(index))),
                        8..=15 => {
                            Some(AnsiColor::Bright(AnsiColor3::from_u8(index - 8)))
                        }
                        _ => Some(AnsiColor::Palette(AnsiColor8(index))),
                    }
                }
                TermColorKind::Rgb => match self.rgb_components() {
                    Some(rgb) => Some(AnsiColor::Rgb(rgb)),
                    None => None,
                },
                TermColorKind::Reserved => None,
            }
        }
    }
}

#[doc = crate::_tags!(term color)]
/// Packed foreground and background terminal colors.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    test_size_of(TermColors = 8|64),
}]
///
/// The foreground occupies the low 32 bits and the background the high 32 bits.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TermColors {
    bits: u64,
}
#[rustfmt::skip]
impl TermColors {
    /// Terminal-default foreground and background colors.
    pub const DEFAULT: Self = Self::new(TermColor::DEFAULT, TermColor::DEFAULT);

    /// Creates a foreground and background color pair.
    pub const fn new(fg: TermColor, bg: TermColor) -> Self {
        Self { bits: fg.bits() as u64 | ((bg.bits() as u64) << 32) }
    }
    /// Creates a color pair from its raw representation.
    pub const fn from_bits(bits: u64) -> Self { Self { bits } }

    /// Returns the raw packed representation.
    #[must_use]
    pub const fn bits(self) -> u64 { self.bits }

    /// Returns the foreground color.
    pub const fn fg(self) -> TermColor { TermColor::from_bits(self.bits as u32) }
    /// Returns the background color.
    pub const fn bg(self) -> TermColor { TermColor::from_bits((self.bits >> 32) as u32) }

    /// Returns this pair with a new foreground color.
    pub const fn with_fg(self, fg: TermColor) -> Self { Self::new(fg, self.bg()) }

    /// Returns this pair with a new background color.
    pub const fn with_bg(self, bg: TermColor) -> Self { Self::new(self.fg(), bg) }

    /// Replaces the foreground color.
    pub const fn set_fg(&mut self, fg: TermColor) { self.bits = Self::new(fg, self.bg()).bits; }
    /// Replaces the background color.
    pub const fn set_bg(&mut self, bg: TermColor) { self.bits = Self::new(self.fg(), bg).bits; }

    /// Returns this pair with foreground and background exchanged.
    pub const fn swapped(self) -> Self { Self::new(self.bg(), self.fg()) }

    /// Exchanges the foreground and background colors.
    pub const fn swap(&mut self) { self.bits = self.bits.rotate_left(32); }

    /// Returns whether both colors are canonical.
    #[must_use]
    pub const fn is_canonical(self) -> bool { self.fg().is_canonical() && self.bg().is_canonical() }

    /// Returns this pair with both colors canonicalized.
    pub const fn canonicalized(self) -> Self {
        Self::new(self.fg().canonicalized(), self.bg().canonicalized())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_representation_is_zero() {
        assert_eq!(TermColor::DEFAULT.bits(), 0);
        assert_eq!(TermColors::DEFAULT.bits(), 0);
        assert!(TermColor::DEFAULT.is_default());
        assert!(TermColor::DEFAULT.is_opaque());
    }
    #[test]
    fn indexed_roundtrip() {
        let color = TermColor::indexed(173);
        assert!(color.is_indexed());
        assert_eq!(color.index(), Some(173));
        assert_eq!(color.rgb_components(), None);
        assert!(color.is_canonical());
    }
    #[test]
    fn rgb_roundtrip() {
        let color = TermColor::rgb([0x12, 0x34, 0x56]);
        assert!(color.is_rgb());
        assert_eq!(color.rgb_components(), Some([0x12, 0x34, 0x56]));
        assert_eq!(color.get_value(), 0x12_34_56);
        assert!(color.is_canonical());
    }
    #[test]
    fn composition_mode_preserves_color() {
        let color = TermColor::rgb([10, 20, 30]).with_color_mode(TermColorMode::Transparent);
        assert_eq!(color.rgb_components(), Some([10, 20, 30]));
        assert!(color.is_transparent());
    }
    #[test]
    fn paired_colors() {
        let fg = TermColor::indexed(7);
        let bg = TermColor::rgb([1, 2, 3]);
        let colors = TermColors::new(fg, bg);
        assert_eq!(colors.fg(), fg);
        assert_eq!(colors.bg(), bg);
        assert_eq!(colors.swapped(), TermColors::new(bg, fg));
    }
    #[test]
    fn ansi_roundtrip() {
        let values = [
            AnsiColor::Default,
            AnsiColor::Dark(AnsiColor3::Red),
            AnsiColor::Bright(AnsiColor3::Blue),
            AnsiColor::Palette(AnsiColor8(203)),
            AnsiColor::Rgb([10, 20, 30]),
        ];
        for ansi in values {
            let color = TermColor::from_ansi(ansi).unwrap();
            assert_eq!(color.to_ansi(), Some(ansi));
        }
        assert_eq!(TermColor::from_ansi(AnsiColor::None), None);
    }
    #[test]
    fn reserved_bits_are_not_canonical() {
        let color = TermColor::from_bits(0xF000_0000);
        assert!(!color.is_canonical());
        assert_eq!(color.canonicalized(), TermColor::DEFAULT);
    }
}
