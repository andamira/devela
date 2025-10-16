// devela_base_core::ui::front::term::ansi::color::bit8
//
//! ANSI codes related to 8-bit palette color.
//

use super::C;
use crate::{_ansi_consts, Ansi, AnsiColor3, Cmp, Digits};

/// ANSI 8-bit color codes, 256 colors.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub const fn to_ascii(&self) -> [u8; 3] { Digits(self.0).digits10() }
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

/// # 8-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    _ansi_consts! {
        /// Code to set the foreground color to `fg` and the background to `bg`.
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
        pub const fn COLOR8_FG(fg: AnsiColor8) -> [u8; 11] {
            const X: [u8; 4] = C::C8;
            let c = fg.to_ascii();
            [ b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
        }

        /// Code to set the background color to `bg`.
        pub const fn COLOR8_BG(bg: AnsiColor8) -> [u8; 11] {
            const X: [u8; 4] = C::C8;
            let c = bg.to_ascii();
            [ b'\x1b', b'[', C::BG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
        }
    }
}

/// # 8-bit Grey escape codes
#[rustfmt::skip]
impl Ansi {
    _ansi_consts! {
        /// Code to set the foreground and background to 24-point grayscale.
        ///
        /// A value of 0 is almost black, and 24 (or more) is almost white.
        pub const fn GRAY(fg: u8, bg: u8) -> [u8; 19] {
            const X: [u8; 4] = C::C8;
            let cf = Digits(Cmp(fg).min(23)).digits10();
            let cb = Digits(Cmp(bg).min(23)).digits10();
            [
                b'\x1b', b'[',
                C::FG, X[0], X[1], X[2], X[3], cf[0], cf[1], cf[2],
                C::BG, X[0], X[1], X[2], X[3], cb[0], cb[1], cb[2],
                b'm',
            ]
        }
    }
}
// # 8-bit Grey constants
#[rustfmt::skip]
impl Ansi {
    // /// ANSI gray foreground 0/23 8-bit color (4% white, 96% black).
    // pub const bGRAY0: [u8; 11] = @ Ansi::bCOLOR8_FG(AnsiColor8::gray_wrap(0));
    _ansi_consts! {
        /// ANSI gray foreground 0/23 8-bit color (4% white, 96% black).
        pub const GRAY0: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(0));
        /// ANSI gray foreground 1/23 8-bit color (8% white, 92% black).
        pub const GRAY1: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(1));
        /// ANSI gray foreground 2/23 8-bit color (12% white, 88% black).
        pub const GRAY2: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(2));
        /// ANSI gray foreground 3/23 8-bit color (16% white, 84% black).
        pub const GRAY3: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(3));
        /// ANSI gray foreground 4/23 8-bit color (20% white, 80% black).
        pub const GRAY4: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(4));
        /// ANSI gray foreground 5/23 8-bit color (24% white, 76% black).
        pub const GRAY5: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(5));
        /// ANSI gray foreground 6/23 8-bit color (28% white, 72% black).
        pub const GRAY6: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(6));
        /// ANSI gray foreground 7/23 8-bit color (32% white, 68% black).
        pub const GRAY7: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(7));
        /// ANSI gray foreground 8/23 8-bit color (36% white, 64% black).
        pub const GRAY8: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(8));
        /// ANSI gray foreground 9/23 8-bit color (40% white, 60% black).
        pub const GRAY9: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(9));
        /// ANSI gray foreground 10/23 8-bit color (44% white, 56% black).
        pub const GRAY10: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(10));
        /// ANSI gray foreground 11/23 8-bit color (48% white, 52% black).
        pub const GRAY11: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(11));
        /// ANSI gray foreground 12/23 8-bit color (52% white, 48% black).
        pub const GRAY12: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(12));
        /// ANSI gray foreground 13/23 8-bit color (56% white, 44% black).
        pub const GRAY13: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(13));
        /// ANSI gray foreground 14/23 8-bit color (60% white, 40% black).
        pub const GRAY14: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(14));
        /// ANSI gray foreground 15/23 8-bit color (64% white, 36% black).
        pub const GRAY15: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(15));
        /// ANSI gray foreground 16/23 8-bit color (68% white, 32% black).
        pub const GRAY16: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(16));
        /// ANSI gray foreground 17/23 8-bit color (72% white, 28% black).
        pub const GRAY17: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(17));
        /// ANSI gray foreground 18/23 8-bit color (76% white, 24% black).
        pub const GRAY18: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(18));
        /// ANSI gray foreground 19/23 8-bit color (80% white, 20% black).
        pub const GRAY19: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(19));
        /// ANSI gray foreground 20/23 8-bit color (84% white, 16% black).
        pub const GRAY20: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(20));
        /// ANSI gray foreground 21/23 8-bit color (88% white, 12% black).
        pub const GRAY21: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(21));
        /// ANSI gray foreground 22/23 8-bit color (92% white, 8% black).
        pub const GRAY22: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(22));
        /// ANSI gray foreground 23/23 8-bit color (96% white, 4% black).
        pub const GRAY23: [u8; 11] = Ansi::COLOR8_FG(AnsiColor8::gray_wrap(23));

        /// ANSI gray background 0/23 8-bit color (4% white, 96% black).
        pub const GRAY0_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(0));
        /// ANSI gray background 1/23 8-bit color (8% white, 92% black).
        pub const GRAY1_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(1));
        /// ANSI gray background 2/23 8-bit color (12% white, 88% black).
        pub const GRAY2_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(2));
        /// ANSI gray background 3/23 8-bit color (16% white, 84% black).
        pub const GRAY3_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(3));
        /// ANSI gray background 4/23 8-bit color (20% white, 80% black).
        pub const GRAY4_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(4));
        /// ANSI gray background 5/23 8-bit color (24% white, 76% black).
        pub const GRAY5_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(5));
        /// ANSI gray background 6/23 8-bit color (28% white, 72% black).
        pub const GRAY6_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(6));
        /// ANSI gray background 7/23 8-bit color (32% white, 68% black).
        pub const GRAY7_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(7));
        /// ANSI gray background 8/23 8-bit color (36% white, 64% black).
        pub const GRAY8_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(8));
        /// ANSI gray background 9/23 8-bit color (40% white, 60% black).
        pub const GRAY9_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(9));
        /// ANSI gray background 10/23 8-bit color (44% white, 56% black).
        pub const GRAY10_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(10));
        /// ANSI gray background 11/23 8-bit color (48% white, 52% black).
        pub const GRAY11_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(11));
        /// ANSI gray background 12/23 8-bit color (52% white, 48% black).
        pub const GRAY12_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(12));
        /// ANSI gray background 13/23 8-bit color (56% white, 44% black).
        pub const GRAY13_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(13));
        /// ANSI gray background 14/23 8-bit color (60% white, 40% black).
        pub const GRAY14_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(14));
        /// ANSI gray background 15/23 8-bit color (64% white, 36% black).
        pub const GRAY15_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(15));
        /// ANSI gray background 16/23 8-bit color (68% white, 32% black).
        pub const GRAY16_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(16));
        /// ANSI gray background 17/23 8-bit color (72% white, 28% black).
        pub const GRAY17_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(17));
        /// ANSI gray background 18/23 8-bit color (76% white, 24% black).
        pub const GRAY18_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(18));
        /// ANSI gray background 19/23 8-bit color (80% white, 20% black).
        pub const GRAY19_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(19));
        /// ANSI gray background 20/23 8-bit color (84% white, 16% black).
        pub const GRAY20_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(20));
        /// ANSI gray background 21/23 8-bit color (88% white, 12% black).
        pub const GRAY21_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(21));
        /// ANSI gray background 22/23 8-bit color (92% white, 8% black).
        pub const GRAY22_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(22));
        /// ANSI gray background 23/23 8-bit color (96% white, 4% black).
        pub const GRAY23_BG: [u8; 11] = Ansi::COLOR8_BG(AnsiColor8::gray_wrap(23));
    }
}

/// # 8-bit palette colors
#[rustfmt::skip]
impl Ansi {
    _ansi_consts! {
        /// Sets the given palette color. (OSC 4)
        // \x1b]4;{index};rgb:{rr:02x}/{gg:02x}/{bb:02x}\x07
        pub const fn SET_PALETTE(index: u8, color: [u8; 3]) -> [u8; 21] {
            let i = Digits(index).digits10();
            let [r, g, b] = color;
            let [r, g, b] = [Digits(r).digits16(), Digits(g).digits16(), Digits(b).digits16()];
            [
                b'\x1b', b'[', b'4', b';', i[0], i[1], i[2], b';',
                b'r', b'g', b'b', b':',
                r[0], r[1], b'/', g[0], g[1], b'/', b[0], b[1], b'\x07'
            ]
        }
        /// Resets the given palette color to the default one. (OSC 104)
        pub const fn RESET_PALETTE(index: u8) -> [u8; 10] {
            let i = Digits(index).digits10();
            [b'\x1b', b'[', b'1', b'0', b'4', b';', i[0], i[1], i[2], b'\x07']
        }
    }
    _ansi_consts! {
        /// Resets all the palette colors to the default ones. (OSC 104)
        pub const RESET_PALETTE_ALL: [u8; 6] = "\x1b]104\x07", *b"\x1b]104\x07";
    }
}
