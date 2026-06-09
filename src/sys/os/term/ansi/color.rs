// devela::sys::os::term::ansi::color::color
//
//! Defines [`AnsiColor`], [`AnsiColor3`], [`AnsiColor8`].
//

use crate::Digits;

#[doc = crate::_tags!(term color)]
/// Complete ANSI color selection
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    test_size_of(AnsiColor = 4|32; niche Option),
}]
/// Covers all terminal color modes:
/// - no color / default
/// - 3-bit (dark / bright)
/// - 8-bit palette (256 colors)
/// - 24-bit truecolor (RGB)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AnsiColor {
    /// No color (explicitly unset).
    None,
    /// Terminal default color.
    Default,
    /// Standard 3-bit colors (ANSI dark set).
    Dark(AnsiColor3),
    /// Bright 3-bit colors (ANSI bright set).
    Bright(AnsiColor3),
    /// 8-bit indexed color (ANSI 256 colors).
    Palette(AnsiColor8),
    /// 24-bit true color RGB.
    Rgb([u8; 3]),
}

/// # Conversions
#[rustfmt::skip]
impl AnsiColor {
    const _CHECK_SIZE: () = const { assert![4 == size_of::<Self>(), "AnsiColor size != 4"] };

    /// Returns `true` if this is [`AnsiColor::None`].
    #[must_use] #[inline(always)]
    pub const fn is_none(self) -> bool { matches![self, Self::None] }
    /// Returns `true` if this is [`AnsiColor::Default`].
    #[must_use] #[inline(always)]
    pub const fn is_default(self) -> bool { matches![self, Self::Default] }

    /// Returns `true` if this is a 3-bit ANSI color (dark or bright).
    #[must_use] #[inline(always)]
    pub const fn is_3(self) -> bool { matches![self, Self::Dark(_) | Self::Bright(_)] }
    /// Returns `true` if this is a dark 3-bit ANSI color.
    #[must_use] #[inline(always)]
    pub const fn is_dark(self) -> bool { matches![self, Self::Dark(_)] }
    /// Returns `true` if this is a bright 3-bit ANSI color.
    #[must_use] #[inline(always)]
    pub const fn is_bright(self) -> bool { matches![self, Self::Bright(_)] }

    /// Returns `true` if this is an 8-bit ANSI palette color.
    #[must_use] #[inline(always)]
    pub const fn is_8(self) -> bool { matches![self, Self::Palette(_)] }
    /// Returns `true` if this is an 8-bit ANSI palette color.
    ///
    /// Alias of [`Self::is_8`].
    #[must_use] #[inline(always)]
    pub const fn is_palette(self) -> bool { matches![self, Self::Palette(_)] }
    /// Returns `true` if this is a 24-bit RGB color.
    #[must_use] #[inline(always)]
    pub const fn is_rgb(self) -> bool { matches![self, Self::Rgb(_)] }

    //

    /// Extracts the 3-bit color if present.
    ///
    /// Returns `None` for non-3-bit variants.
    #[must_use] #[inline(always)]
    pub const fn into_3(self) -> Option<AnsiColor3> {
        match self { Self::Dark(c) | Self::Bright(c) => Some(c), _ => None }
    }
    /// Extracts the 8-bit palette color if present.
    ///
    /// Returns `None` for non-palette variants.
    #[must_use] #[inline(always)]
    pub const fn into_8(self) -> Option<AnsiColor8> {
        match self { Self::Palette(c) => Some(c), _ => None }
    }
    /// Extracts the RGB value if present.
    ///
    /// Returns `None` for non-RGB variants.
    #[must_use] #[inline(always)]
    pub const fn into_rgb(self) -> Option<[u8; 3]> {
        match self { Self::Rgb(c) => Some(c), _ => None }
    }
}

/// # Color constants
#[allow(missing_docs, non_upper_case_globals)]
impl AnsiColor {
    pub const Black: Self = Self::Dark(AnsiColor3::Black);
    pub const Red: Self = Self::Dark(AnsiColor3::Red);
    pub const Green: Self = Self::Dark(AnsiColor3::Green);
    pub const Yellow: Self = Self::Dark(AnsiColor3::Yellow);
    pub const Blue: Self = Self::Dark(AnsiColor3::Blue);
    pub const Magenta: Self = Self::Dark(AnsiColor3::Magenta);
    pub const Cyan: Self = Self::Dark(AnsiColor3::Cyan);
    pub const White: Self = Self::Dark(AnsiColor3::White);

    pub const BlackBright: Self = Self::Bright(AnsiColor3::Black);
    pub const RedBright: Self = Self::Bright(AnsiColor3::Red);
    pub const GreenBright: Self = Self::Bright(AnsiColor3::Green);
    pub const YellowBright: Self = Self::Bright(AnsiColor3::Yellow);
    pub const BlueBright: Self = Self::Bright(AnsiColor3::Blue);
    pub const MagentaBright: Self = Self::Bright(AnsiColor3::Magenta);
    pub const CyanBright: Self = Self::Bright(AnsiColor3::Cyan);
    pub const WhiteBright: Self = Self::Bright(AnsiColor3::White);

    /* constants: abbr */
    pub const K: Self = Self::Black;
    pub const R: Self = Self::Red;
    pub const G: Self = Self::Green;
    pub const Y: Self = Self::Yellow;
    pub const B: Self = Self::Blue;
    pub const M: Self = Self::Magenta;
    pub const C: Self = Self::Cyan;
    pub const W: Self = Self::White;
    pub const KB: Self = Self::BlackBright;
    pub const RB: Self = Self::RedBright;
    pub const GB: Self = Self::GreenBright;
    pub const YB: Self = Self::YellowBright;
    pub const BB: Self = Self::BlueBright;
    pub const MB: Self = Self::MagentaBright;
    pub const CB: Self = Self::CyanBright;
    pub const WB: Self = Self::WhiteBright;
}

#[doc = crate::_tags!(term color)]
/// ANSI 3-bit color codes, 8 colors.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AnsiColor3 {
    /// 0 Black
    #[default]
    Black = 0,
    /// 1 Red
    Red = 1,
    /// 2 Green
    Green = 2,
    /// 3 Yellow
    Yellow = 3,
    /// 4 Blue
    Blue = 4,
    /// 5 Magenta
    Magenta = 5,
    /// 6 Cyan
    Cyan = 6,
    /// 7 White
    White = 7,
}
impl From<u8> for AnsiColor3 {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}
impl AnsiColor3 {
    /// Returns the ASCII byte representation of the 8-bit color number, with padding zeros.
    #[must_use]
    pub const fn to_ascii(&self) -> u8 {
        Digits(*self as u8).digits10_1()
    }
    /// Returns an `AnsiColor3` from an `u8` value.
    /// If `value` > 7 then returns Black.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Black,
            1 => Self::Red,
            2 => Self::Green,
            3 => Self::Yellow,
            4 => Self::Blue,
            5 => Self::Magenta,
            6 => Self::Cyan,
            7 => Self::White,
            _ => Self::Black,
        }
    }
}

#[doc = crate::_tags!(term color)]
/// ANSI 8-bit color codes, 256 colors.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnsiColor8(pub u8);

impl From<AnsiColor3> for AnsiColor8 {
    fn from(value: AnsiColor3) -> Self {
        Self::new(value)
    }
}
impl From<u8> for AnsiColor8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[rustfmt::skip]
impl AnsiColor8 {
    /// Creates a new `AnsiColor8` from an `AnsiColor3`.
    #[must_use]
    pub const fn new(color: AnsiColor3) -> Self { Self(color as u8) }

    /// Creates a new `AnsiColor8` from an `AnsiColor3` treated as *bright*.
    #[must_use]
    pub const fn bright(color: AnsiColor3) -> Self { Self(color as u8 + 8) }

    /// Creates a new `AnsiColor8` from a 216-color 6x6x6 RGB cube.
    /// The `r`, `g`, and `b` parameters should be in the range `0..=5`.
    ///
    /// Returns `None` if any parameter is `> 5`.
    #[must_use]
    pub const fn cube(r: u8, g: u8, b: u8) -> Option<Self> {
        match (r, g, b) {
            (0..=5, 0..=5, 0..=5) => Some(Self(16 + 36 * r + 6 * g + b)),
            _ => None,
        }
    }
    /// Creates a new `AnsiColor8` from a 216-color 6x6x6 RGB cube with
    /// `r`, `g`, `b` values between `0` and `5`.
    ///
    /// Returns the `fallback` color if any parameter is `> 5`.
    #[must_use]
    pub const fn cube_or(r: u8, g: u8, b: u8, fallback: Self) -> Self {
        match (r, g, b) {
            (0..=5, 0..=5, 0..=5) => Self(16 + 36 * r + 6 * g + b),
            _ => fallback,
        }
    }
    /// Creates a new `AnsiColor8` from a 216-color 6x6x6 RGB cube by wrapping values.
    ///
    /// Out-of-bounds values wrap via `% 6` (e.g., `6` → `0`, `7` → `1`).
    /// This is branchless and the fastest method.
    #[must_use]
    pub const fn cube_wrap(r: u8, g: u8, b: u8) -> Self {
        Self(16 + 36 * (r % 6) + 6 * (g % 6) + (b % 6))
    }

    /* Standard 24-color grayscale (faster, contiguous ANSI codes) */

    /// Creates a new `AnsiColor8` from a 24-color grayscale `value`, between
    /// `0` (almost black) and `23` (almost white).
    ///
    /// Returns `None` if `value > 23`.
    #[must_use]
    pub const fn gray(value: u8) -> Option<Self> {
        match value {
            0..=23 => Some(Self(value + 232)),
            _ => None,
        }
    }
    /// Creates a new `AnsiColor8` from a 24-color grayscale `value` between
    /// `0` (almost black) and `23` (almost white).
    ///
    /// Returns the `fallback` color if `value > 23`.
    #[must_use]
    pub const fn gray_or(value: u8, fallback: Self) -> Self {
        match value {
            0..=23 => Self(value + 232),
            _ => fallback,
        }
    }
    /// Creates a grayscale color by wrapping `value` via `% 24`.
    ///
    /// Values map to ANSI codes 232..=255 (e.g., `24` → `0`, `25` → `1`).
    ///
    /// This is branchless and the fastest method.
    #[must_use]
    pub const fn gray_wrap(value: u8) -> Self { Self(232 + (value % 24)) }

    /* Extended 26-color grayscale with pure black/white (slower, non-contiguous)*/

    /// Creates a new `AnsiColor8` from a 26-color grayscale `value`
    /// between `0` (pure black) and `25` (pure white).
    ///
    /// Returns `None` if `value > 23`.
    #[must_use]
    pub const fn bw(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::new(AnsiColor3::Black)),
            1..=24 => Some(Self(value - 1 + 232)),
            25 => Some(Self::new(AnsiColor3::White)),
            _ => None,
        }
    }
    /// Creates a new `AnsiColor8` from a 26-color grayscale `value`
    /// between `0` (pure black) and `25` (pure white).
    ///
    /// Returns the `fallback` color if `value > 25`.
    #[must_use]
    pub const fn bw_or(value: u8, fallback: Self) -> Self {
        match value {
            0 => Self::new(AnsiColor3::Black),
            1..=24 => Self(value - 1 + 232),
            25 => Self::new(AnsiColor3::White),
            _ => fallback,
        }
    }
    /// Creates a grayscale color by wrapping `value` via `% 26`, with exact black/white.
    ///
    /// - `0` → pure black (`AnsiColor3::Black`).
    /// - `25` → pure white (`AnsiColor3::White`).
    /// - Other values wrap (e.g., `26` → `0`, `27` → `1`).
    #[must_use]
    pub const fn bw_wrap(value: u8) -> Self {
        let wrapped = value % 26;
        match wrapped {
            0 => Self::new(AnsiColor3::Black),
            25 => Self::new(AnsiColor3::White),
            _ => Self(wrapped - 1 + 232),
        }
    }

    /// Returns the ASCII byte representation of the 8-bit color number, with leading zeros.
    #[must_use]
    pub const fn to_ascii(&self) -> [u8; 3] { Digits(self.0).digits10() }
}
