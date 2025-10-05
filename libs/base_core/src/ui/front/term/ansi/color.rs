// devela::ui::front::term::ansi::color
//
//! ANSI codes related to color.
//
// TOC
// - enum AnsiColor3
// - struct AnsiColor8
// - impl Ansi:
//   - 3-bit Color escape codes
//   - 8-bit Color escape codes
//   - 8-bit Grey escape codes
//   - RGB Color escape codes

use crate::{Ansi, AsciiDigits as AsciiD, Cmp};

/// ANSI 3-bit color codes, 8 colors.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
impl AnsiColor3 {
    /// Returns the ASCII byte representation of the 8-bit color number, with padding zeros.
    #[must_use]
    pub const fn to_ascii(&self) -> u8 {
        AsciiD(*self as u8).digits10_1()
    }
    /// Returns an `AnsiColor3` from an `u8` value.
    /// If `value` > 7 then returns Black.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        match value {
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
impl From<u8> for AnsiColor3 {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

/// ANSI 8-bit color codes, 256 colors.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiColor8(pub u8);

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
    pub const fn to_ascii(&self) -> [u8; 3] { AsciiD(self.0).digits10() }
}
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

// the bare color escape codes (module private)
mod C {
    use super::AnsiColor3 as A8;

    pub(super) const FG: u8 = b'3';
    pub(super) const BG: u8 = b'4';
    pub(super) const BRI_FG: u8 = b'9';
    pub(super) const BRI_BG: [u8; 2] = *b"10";
    //
    pub(super) const C8: [u8; 4] = *b"8;5;";
    pub(super) const RGB: [u8; 4] = *b"8;2;";
    //
    pub(super) const BLACK_FG: [u8; 2] = [FG, A8::Black.to_ascii()];
    pub(super) const RED_FG: [u8; 2] = [FG, A8::Red.to_ascii()];
    pub(super) const GREEN_FG: [u8; 2] = [FG, A8::Green.to_ascii()];
    pub(super) const YELLOW_FG: [u8; 2] = [FG, A8::Yellow.to_ascii()];
    pub(super) const BLUE_FG: [u8; 2] = [FG, A8::Blue.to_ascii()];
    pub(super) const MAGENTA_FG: [u8; 2] = [FG, A8::Magenta.to_ascii()];
    pub(super) const CYAN_FG: [u8; 2] = [FG, A8::Cyan.to_ascii()];
    pub(super) const WHITE_FG: [u8; 2] = [FG, A8::White.to_ascii()];

    pub(super) const BLACK_BG: [u8; 2] = [BG, A8::Black.to_ascii()];
    pub(super) const RED_BG: [u8; 2] = [BG, A8::Red.to_ascii()];
    pub(super) const GREEN_BG: [u8; 2] = [BG, A8::Green.to_ascii()];
    pub(super) const YELLOW_BG: [u8; 2] = [BG, A8::Yellow.to_ascii()];
    pub(super) const BLUE_BG: [u8; 2] = [BG, A8::Blue.to_ascii()];
    pub(super) const MAGENTA_BG: [u8; 2] = [BG, A8::Magenta.to_ascii()];
    pub(super) const CYAN_BG: [u8; 2] = [BG, A8::Cyan.to_ascii()];
    pub(super) const WHITE_BG: [u8; 2] = [BG, A8::White.to_ascii()];

    pub(super) const BRI_BLACK_FG: [u8; 2] = [BRI_FG, A8::Black.to_ascii()];
    pub(super) const BRI_RED_FG: [u8; 2] = [BRI_FG, A8::Red.to_ascii()];
    pub(super) const BRI_GREEN_FG: [u8; 2] = [BRI_FG, A8::Green.to_ascii()];
    pub(super) const BRI_YELLOW_FG: [u8; 2] = [BRI_FG, A8::Yellow.to_ascii()];
    pub(super) const BRI_BLUE_FG: [u8; 2] = [BRI_FG, A8::Blue.to_ascii()];
    pub(super) const BRI_MAGENTA_FG: [u8; 2] = [BRI_FG, A8::Magenta.to_ascii()];
    pub(super) const BRI_CYAN_FG: [u8; 2] = [BRI_FG, A8::Cyan.to_ascii()];
    pub(super) const BRI_WHITE_FG: [u8; 2] = [BRI_FG, A8::White.to_ascii()];

    pub(super) const BRI_BLACK_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Black.to_ascii()];
    pub(super) const BRI_RED_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Red.to_ascii()];
    pub(super) const BRI_GREEN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Green.to_ascii()];
    pub(super) const BRI_YELLOW_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Yellow.to_ascii()];
    pub(super) const BRI_BLUE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Blue.to_ascii()];
    pub(super) const BRI_MAGENTA_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Magenta.to_ascii()];
    pub(super) const BRI_CYAN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Cyan.to_ascii()];
    pub(super) const BRI_WHITE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::White.to_ascii()];
}

/// # 3-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    /// Code to set the the foreground `color`.
    #[must_use]
    pub const fn COLOR_FG(color: AnsiColor3) -> [u8; 5] {
        [b'\x1b', b'[', C::FG, color.to_ascii(), b'm']
    }
    /// Code to set the the foreground `color` bright.
    #[must_use]
    pub const fn COLOR_FG_BRIGHT(color: AnsiColor3) -> [u8; 5] {
        [b'\x1b', b'[', C::BRI_FG, color.to_ascii(), b'm']
    }

    /// Code to set the the background `color`.
    #[must_use]
    pub const fn COLOR_BG(color: AnsiColor3) -> [u8; 5] {
        [b'\x1b', b'[', C::BG, color.to_ascii(), b'm']
    }
    /// Code to set the the background `color` bright.
    #[must_use]
    pub const fn COLOR_BG_BRIGHT(color: AnsiColor3) -> [u8; 6] {
        [b'\x1b', b'[', C::BRI_BG[0], C::BRI_BG[1], color.to_ascii(), b'm']
    }

    /// Code to set the foreground color to `fg` and the background to `bg`.
    #[must_use]
    pub const fn COLORS(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
        [ b'\x1b', b'[', C::FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
    }
    /// Code to set the foreground color to bright `fg` and the background to bright `bg`.
    #[must_use]
    pub const fn COLORS_BRIGHT(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
        [
            b'\x1b', b'[',
            C::BRI_FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
            b'm',
        ]
    }

    /// Code to set the foreground color to bright `fg` and the background to `bg`.
    #[must_use]
    pub const fn COLORS_BRIGHT_FG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
        [ b'\x1b', b'[', C::BRI_FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
    }

    /// Code to set the foreground color to `fg` and the background to bright `bg`.
    #[must_use]
    pub const fn COLORS_BRIGHT_BG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
        [
            b'\x1b', b'[',
            C::FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
            b'm',
        ]
    }

    /// Code to set the foreground color to black.
    pub const BLACK: [u8; 5] = [b'\x1b', b'[', C::BLACK_FG[0], C::BLACK_FG[1], b'm'];
    /// Code to set the foreground color to red.
    pub const RED: [u8; 5] = [b'\x1b', b'[', C::RED_FG[0], C::RED_FG[1], b'm'];
    /// Code to set the foreground color to green.
    pub const GREEN: [u8; 5] = [b'\x1b', b'[', C::GREEN_FG[0], C::GREEN_FG[1], b'm'];
    /// Code to set the foreground color to yellow.
    pub const YELLOW: [u8; 5] = [b'\x1b', b'[', C::YELLOW_FG[0], C::YELLOW_FG[1], b'm'];
    /// Code to set the foreground color to blue.
    pub const BLUE: [u8; 5] = [b'\x1b', b'[', C::BLUE_FG[0], C::BLUE_FG[1], b'm'];
    /// Code to set the foreground color to magenta.
    pub const MAGENTA: [u8; 5] = [b'\x1b', b'[', C::MAGENTA_FG[0], C::MAGENTA_FG[1], b'm'];
    /// Code to set the foreground color to cyan.
    pub const CYAN: [u8; 5] = [b'\x1b', b'[', C::CYAN_FG[0], C::CYAN_FG[1], b'm'];
    /// Code to set the foreground color to white.
    pub const WHITE: [u8; 5] = [b'\x1b', b'[', C::WHITE_FG[0], C::WHITE_FG[1], b'm'];

    /// Code to set the background color to black.
    pub const BLACK_BG: [u8; 5] = [b'\x1b', b'[', C::BLACK_BG[0], C::BLACK_BG[1], b'm'];
    /// Code to set the background color to red.
    pub const RED_BG: [u8; 5] = [b'\x1b', b'[', C::RED_BG[0], C::RED_BG[1], b'm'];
    /// Code to set the background color to green.
    pub const GREEN_BG: [u8; 5] = [b'\x1b', b'[', C::GREEN_BG[0], C::GREEN_BG[1], b'm'];
    /// Code to set the background color to yellow.
    pub const YELLOW_BG: [u8; 5] = [b'\x1b', b'[', C::YELLOW_BG[0], C::YELLOW_BG[1], b'm'];
    /// Code to set the background color to blue.
    pub const BLUE_BG: [u8; 5] = [b'\x1b', b'[', C::BLUE_BG[0], C::BLUE_BG[1], b'm'];
    /// Code to set the background color to magenta.
    pub const MAGENTA_BG: [u8; 5] = [b'\x1b', b'[', C::MAGENTA_BG[0], C::MAGENTA_BG[1], b'm'];
    /// Code to set the background color to cyan.
    pub const CYAN_BG: [u8; 5] = [b'\x1b', b'[', C::CYAN_BG[0], C::CYAN_BG[1], b'm'];
    /// Code to set the background color to white.
    pub const WHITE_BG: [u8; 5] = [b'\x1b', b'[', C::WHITE_BG[0], C::WHITE_BG[1], b'm'];

    /// Code to set the foreground color to bright black.
    pub const BRIGHT_BLACK: [u8; 5] = [b'\x1b', b'[', C::BRI_BLACK_FG[0], C::BRI_BLACK_FG[1], b'm'];
    /// Code to set the foreground color to bright red.
    pub const BRIGHT_RED: [u8; 5] = [b'\x1b', b'[', C::BRI_RED_FG[0], C::BRI_RED_FG[1], b'm'];
    /// Code to set the foreground color to bright green.
    pub const BRIGHT_GREEN: [u8; 5] = [b'\x1b', b'[', C::BRI_GREEN_FG[0], C::BRI_GREEN_FG[1], b'm'];
    /// Code to set the foreground color to bright yellow.
    pub const BRIGHT_YELLOW: [u8; 5] = [
        b'\x1b', b'[', C::BRI_YELLOW_FG[0], C::BRI_YELLOW_FG[1], b'm'];
    /// Code to set the foreground color to bright blue.
    pub const BRIGHT_BLUE: [u8; 5] = [b'\x1b', b'[', C::BRI_BLUE_FG[0], C::BRI_BLUE_FG[1], b'm'];
    /// Code to set the foreground color to bright magenta.
    pub const BRIGHT_MAGENTA: [u8; 5] = [
        b'\x1b', b'[', C::BRI_MAGENTA_FG[0], C::BRI_MAGENTA_FG[1], b'm'];
    /// Code to set the foreground color to bright cyan.
    pub const BRIGHT_CYAN: [u8; 5] = [b'\x1b', b'[', C::BRI_CYAN_FG[0], C::BRI_CYAN_FG[1], b'm'];
    /// Code to set the foreground color to bright white.
    pub const BRIGHT_WHITE: [u8; 5] = [b'\x1b', b'[', C::BRI_WHITE_FG[0], C::BRI_WHITE_FG[1], b'm'];

    /// Code to set the background color to bright black.
    pub const BRIGHT_BLACK_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_BLACK_BG[0], C::BRI_BLACK_BG[1], C::BRI_BLACK_BG[2], b'm'
    ];
    /// Code to set the background color to bright red.
    pub const BRIGHT_RED_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_RED_BG[0], C::BRI_RED_BG[1], C::BRI_RED_BG[2], b'm',
    ];
    /// Code to set the background color to bright green.
    pub const BRIGHT_GREEN_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_GREEN_BG[0], C::BRI_GREEN_BG[1], C::BRI_GREEN_BG[2], b'm',
    ];
    /// Code to set the background color to bright yellow.
    pub const BRIGHT_YELLOW_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_YELLOW_BG[0], C::BRI_YELLOW_BG[1], C::BRI_YELLOW_BG[2], b'm',
    ];
    /// Code to set the background color to bright blue.
    pub const BRIGHT_BLUE_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_BLUE_BG[0], C::BRI_BLUE_BG[1], C::BRI_BLUE_BG[2], b'm',
    ];
    /// Code to set the background color to bright magenta.
    pub const BRIGHT_MAGENTA_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_MAGENTA_BG[0], C::BRI_MAGENTA_BG[1], C::BRI_MAGENTA_BG[2], b'm',
    ];
    /// Code to set the background color to bright cyan.
    pub const BRIGHT_CYAN_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_CYAN_BG[0], C::BRI_CYAN_BG[1], C::BRI_CYAN_BG[2], b'm',
    ];
    /// Code to set the background color to bright white.
    pub const BRIGHT_WHITE_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_WHITE_BG[0], C::BRI_WHITE_BG[1], C::BRI_WHITE_BG[2], b'm'
    ];
}

/// # 8-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    /// Code to set the foreground color to `fg` and the background to `bg`.
    #[must_use]
    pub const fn COLORS8(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 19] {
        const X: [u8; 4] = C::C8;
        let cf = fg.to_ascii();
        let cb = bg.to_ascii();
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3], cf[0], cf[1], cf[2],
            C::BG, X[0], X[1], X[2], X[3], cb[0], cb[1], cb[2],
            b'm',
        ]
    }

    /// Code to set the foreground color to `fg`.
    #[must_use]
    pub const fn COLOR8_FG(fg: AnsiColor8) -> [u8; 11] {
        const X: [u8; 4] = C::C8;
        let c = fg.to_ascii();
        [ b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
    }

    /// Code to set the background color to `bg`.
    #[must_use]
    pub const fn COLOR8_BG(bg: AnsiColor8) -> [u8; 11] {
        const X: [u8; 4] = C::C8;
        let c = bg.to_ascii();
        [ b'\x1b', b'[', C::BG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
    }
}

/// # 8-bit Grey escape codes
#[rustfmt::skip]
impl Ansi {
    /// Code to set the foreground and background to 24-point grayscale.
    ///
    /// A value of 0 is almost black, and 24 (or more) is almost white.
    #[must_use]
    pub const fn GRAY(fg: u8, bg: u8) -> [u8; 19] {
        const X: [u8; 4] = C::C8;
        let cf = AsciiD(Cmp(fg).min(23)).digits10();
        let cb = AsciiD(Cmp(bg).min(23)).digits10();
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3], cf[0], cf[1], cf[2],
            C::BG, X[0], X[1], X[2], X[3], cb[0], cb[1], cb[2],
            b'm',
        ]
    }

    /// ANSI gray foreground 0/23 8-bit color (4% white, 96% black).
    pub const GRAY0: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(0));
    /// ANSI gray foreground 1/23 8-bit color (8% white, 92% black).
    pub const GRAY1: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(1));
    /// ANSI gray foreground 2/23 8-bit color (12% white, 88% black).
    pub const GRAY2: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(2));
    /// ANSI gray foreground 3/23 8-bit color (16% white, 84% black).
    pub const GRAY3: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(3));
    /// ANSI gray foreground 4/23 8-bit color (20% white, 80% black).
    pub const GRAY4: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(4));
    /// ANSI gray foreground 5/23 8-bit color (24% white, 76% black).
    pub const GRAY5: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(5));
    /// ANSI gray foreground 6/23 8-bit color (28% white, 72% black).
    pub const GRAY6: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(6));
    /// ANSI gray foreground 7/23 8-bit color (32% white, 68% black).
    pub const GRAY7: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(7));
    /// ANSI gray foreground 8/23 8-bit color (36% white, 64% black).
    pub const GRAY8: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(8));
    /// ANSI gray foreground 9/23 8-bit color (40% white, 60% black).
    pub const GRAY9: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(9));
    /// ANSI gray foreground 10/23 8-bit color (44% white, 56% black).
    pub const GRAY10: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(10));
    /// ANSI gray foreground 11/23 8-bit color (48% white, 52% black).
    pub const GRAY11: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(11));
    /// ANSI gray foreground 12/23 8-bit color (52% white, 48% black).
    pub const GRAY12: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(12));
    /// ANSI gray foreground 13/23 8-bit color (56% white, 44% black).
    pub const GRAY13: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(13));
    /// ANSI gray foreground 14/23 8-bit color (60% white, 40% black).
    pub const GRAY14: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(14));
    /// ANSI gray foreground 15/23 8-bit color (64% white, 36% black).
    pub const GRAY15: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(15));
    /// ANSI gray foreground 16/23 8-bit color (68% white, 32% black).
    pub const GRAY16: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(16));
    /// ANSI gray foreground 17/23 8-bit color (72% white, 28% black).
    pub const GRAY17: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(17));
    /// ANSI gray foreground 18/23 8-bit color (76% white, 24% black).
    pub const GRAY18: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(18));
    /// ANSI gray foreground 19/23 8-bit color (80% white, 20% black).
    pub const GRAY19: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(19));
    /// ANSI gray foreground 20/23 8-bit color (84% white, 16% black).
    pub const GRAY20: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(20));
    /// ANSI gray foreground 21/23 8-bit color (88% white, 12% black).
    pub const GRAY21: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(21));
    /// ANSI gray foreground 22/23 8-bit color (92% white, 8% black).
    pub const GRAY22: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(22));
    /// ANSI gray foreground 23/23 8-bit color (96% white, 4% black).
    pub const GRAY23: [u8; 11] = Self::COLOR8_FG(AnsiColor8::gray_wrap(23));

    /// ANSI gray background 0/23 8-bit color (4% white, 96% black).
    pub const GRAY0_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(0));
    /// ANSI gray background 1/23 8-bit color (8% white, 92% black).
    pub const GRAY1_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(1));
    /// ANSI gray background 2/23 8-bit color (12% white, 88% black).
    pub const GRAY2_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(2));
    /// ANSI gray background 3/23 8-bit color (16% white, 84% black).
    pub const GRAY3_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(3));
    /// ANSI gray background 4/23 8-bit color (20% white, 80% black).
    pub const GRAY4_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(4));
    /// ANSI gray background 5/23 8-bit color (24% white, 76% black).
    pub const GRAY5_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(5));
    /// ANSI gray background 6/23 8-bit color (28% white, 72% black).
    pub const GRAY6_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(6));
    /// ANSI gray background 7/23 8-bit color (32% white, 68% black).
    pub const GRAY7_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(7));
    /// ANSI gray background 8/23 8-bit color (36% white, 64% black).
    pub const GRAY8_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(8));
    /// ANSI gray background 9/23 8-bit color (40% white, 60% black).
    pub const GRAY9_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(9));
    /// ANSI gray background 10/23 8-bit color (44% white, 56% black).
    pub const GRAY10_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(10));
    /// ANSI gray background 11/23 8-bit color (48% white, 52% black).
    pub const GRAY11_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(11));
    /// ANSI gray background 12/23 8-bit color (52% white, 48% black).
    pub const GRAY12_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(12));
    /// ANSI gray background 13/23 8-bit color (56% white, 44% black).
    pub const GRAY13_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(13));
    /// ANSI gray background 14/23 8-bit color (60% white, 40% black).
    pub const GRAY14_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(14));
    /// ANSI gray background 15/23 8-bit color (64% white, 36% black).
    pub const GRAY15_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(15));
    /// ANSI gray background 16/23 8-bit color (68% white, 32% black).
    pub const GRAY16_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(16));
    /// ANSI gray background 17/23 8-bit color (72% white, 28% black).
    pub const GRAY17_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(17));
    /// ANSI gray background 18/23 8-bit color (76% white, 24% black).
    pub const GRAY18_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(18));
    /// ANSI gray background 19/23 8-bit color (80% white, 20% black).
    pub const GRAY19_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(19));
    /// ANSI gray background 20/23 8-bit color (84% white, 16% black).
    pub const GRAY20_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(20));
    /// ANSI gray background 21/23 8-bit color (88% white, 12% black).
    pub const GRAY21_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(21));
    /// ANSI gray background 22/23 8-bit color (92% white, 8% black).
    pub const GRAY22_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(22));
    /// ANSI gray background 23/23 8-bit color (96% white, 4% black).
    pub const GRAY23_BG: [u8; 11] = Self::COLOR8_BG(AnsiColor8::gray_wrap(23));
}

/// # RGB Color escape codes
#[rustfmt::skip]
impl Ansi {
    /// Code to set the foreground color to `fg: [r, g, b]` values,
    /// and the background to `bg: [r, g, b]`.
    #[must_use]
    pub const fn RGB(fg: [u8; 3], bg: [u8; 3]) -> [u8; 35] {
        const X: [u8; 4] = C::RGB;
        let [fr, fg, fb] = fg;
        let [br, bg, bb] = bg;
        let [fr, fg, fb] = [AsciiD(fr).digits10(), AsciiD(fg).digits10(), AsciiD(fb).digits10()];
        let [br, bg, bb] = [AsciiD(br).digits10(), AsciiD(bg).digits10(), AsciiD(bb).digits10()];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            fr[0], fr[1], fr[2], b';', fg[0], fg[1], fg[2], b';', fb[0], fb[1], fb[2],
            C::BG, X[0], X[1], X[2], X[3],
            br[0], br[1], br[2], b';', bg[0], bg[1], bg[2], b';', bb[0], bb[1], bb[2],
            b'm',
        ]
    }

    /// Code to set the foreground color to `fg: [r, g, b]` values.
    #[must_use]
    pub const fn RGB_FG(fg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = fg;
        let [r, g, b] = [AsciiD(r).digits10(), AsciiD(g).digits10(), AsciiD(b).digits10()];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm'
        ]
    }

    /// Code to set the background color to `bg: [r, g, b]` values.
    #[must_use]
    pub const fn RGB_BG(bg: [u8; 3]) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = bg;
        let [r, g, b] = [AsciiD(r).digits10(), AsciiD(g).digits10(), AsciiD(b).digits10()];
        [
            b'\x1b', b'[',
            C::BG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm',
        ]
    }
}
