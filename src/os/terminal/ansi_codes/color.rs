// devela::os::terminal::ansi::color
//
//! ANSI codes related to color.
//

use super::{ascii_d1, ascii_d3, Ansi};

/// ANSI 3-bit color codes, 8 colors.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AnsiColor8 {
    #[default]
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl AnsiColor8 {
    /// Returns the ASCII representation of the 8-bit color number, with padding zeros.
    pub const fn to_ascii(&self) -> u8 {
        ascii_d1(*self as u8)
    }

    /// Returns an `AnsiColor8` from an `u8` value.
    /// If `value` > 7 then returns Black.
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
impl From<u8> for AnsiColor8 {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

/// ANSI 8-bit color codes, 256 colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiColor256(pub u8);

impl AnsiColor256 {
    /// Creates a new `AnsiColor256` from a standard `AnsiColor8`.
    pub const fn new(color: AnsiColor8) -> Self {
        Self(color as u8)
    }

    /// Creates a new `AnsiColor256` from a bright `AnsiColor8`.
    pub const fn new_bright(color: AnsiColor8) -> Self {
        Self(color as u8 + 8)
    }

    /// Creates a new `AnsiColor256` from a 216-color 6x6x6 RGB cube.
    /// The `r`, `g`, and `b` parameters should be in the range `0..=5`.
    ///
    /// # Panics
    /// Panics in debug if either parameter is `> 5`.
    pub const fn new_cube(r: u8, g: u8, b: u8) -> Self {
        assert!(r < 6 && g < 6 && b < 6);
        Self(16 + 36 * r + 6 * g + b)
    }

    /// Creates a new `AnsiColor256` from a 24-color grayscale `value`.
    /// The `value` parameter should be in the range `0..=23`.
    ///
    /// # Panics
    /// Panics in debug if `value > 23`.
    pub const fn new_gray(value: u8) -> Self {
        debug_assert!(value < 24);
        Self(value + 232)
    }

    /// Returns the ASCII representation of the 8-bit color number, with leading zeros.
    pub const fn to_ascii(&self) -> [u8; 3] {
        ascii_d3(self.0 as u16)
    }
}
impl From<AnsiColor8> for AnsiColor256 {
    fn from(value: AnsiColor8) -> Self {
        Self::new(value)
    }
}
impl From<u8> for AnsiColor256 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

// the bare color escape codes
mod C {
    use super::AnsiColor8 as A8;

    pub const FG: u8 = b'3';
    pub const BG: u8 = b'4';
    pub const BRI_FG: u8 = b'9';
    pub const BRI_BG: [u8; 2] = *b"10";
    //
    pub const C256: [u8; 4] = *b"8;5;";
    pub const RGB: [u8; 4] = *b"8;2;";
    //
    pub const BLACK_FG: [u8; 2] = [FG, A8::Black.to_ascii()];
    pub const RED_FG: [u8; 2] = [FG, A8::Red.to_ascii()];
    pub const GREEN_FG: [u8; 2] = [FG, A8::Green.to_ascii()];
    pub const YELLOW_FG: [u8; 2] = [FG, A8::Yellow.to_ascii()];
    pub const BLUE_FG: [u8; 2] = [FG, A8::Blue.to_ascii()];
    pub const MAGENTA_FG: [u8; 2] = [FG, A8::Magenta.to_ascii()];
    pub const CYAN_FG: [u8; 2] = [FG, A8::Cyan.to_ascii()];
    pub const WHITE_FG: [u8; 2] = [FG, A8::White.to_ascii()];

    pub const BLACK_BG: [u8; 2] = [BG, A8::Black.to_ascii()];
    pub const RED_BG: [u8; 2] = [BG, A8::Red.to_ascii()];
    pub const GREEN_BG: [u8; 2] = [BG, A8::Green.to_ascii()];
    pub const YELLOW_BG: [u8; 2] = [BG, A8::Yellow.to_ascii()];
    pub const BLUE_BG: [u8; 2] = [BG, A8::Blue.to_ascii()];
    pub const MAGENTA_BG: [u8; 2] = [BG, A8::Magenta.to_ascii()];
    pub const CYAN_BG: [u8; 2] = [BG, A8::Cyan.to_ascii()];
    pub const WHITE_BG: [u8; 2] = [BG, A8::White.to_ascii()];

    pub const BRI_BLACK_FG: [u8; 2] = [BRI_FG, A8::Black.to_ascii()];
    pub const BRI_RED_FG: [u8; 2] = [BRI_FG, A8::Red.to_ascii()];
    pub const BRI_GREEN_FG: [u8; 2] = [BRI_FG, A8::Green.to_ascii()];
    pub const BRI_YELLOW_FG: [u8; 2] = [BRI_FG, A8::Yellow.to_ascii()];
    pub const BRI_BLUE_FG: [u8; 2] = [BRI_FG, A8::Blue.to_ascii()];
    pub const BRI_MAGENTA_FG: [u8; 2] = [BRI_FG, A8::Magenta.to_ascii()];
    pub const BRI_CYAN_FG: [u8; 2] = [BRI_FG, A8::Cyan.to_ascii()];
    pub const BRI_WHITE_FG: [u8; 2] = [BRI_FG, A8::White.to_ascii()];

    pub const BRI_BLACK_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Black.to_ascii()];
    pub const BRI_RED_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Red.to_ascii()];
    pub const BRI_GREEN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Green.to_ascii()];
    pub const BRI_YELLOW_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Yellow.to_ascii()];
    pub const BRI_BLUE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Blue.to_ascii()];
    pub const BRI_MAGENTA_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Magenta.to_ascii()];
    pub const BRI_CYAN_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::Cyan.to_ascii()];
    pub const BRI_WHITE_BG: [u8; 3] = [BRI_BG[0], BRI_BG[1], A8::White.to_ascii()];
}

/// # 4-bit Color escape codes
impl Ansi {
    /// Code to set the foreground color to `fg` and the background to `bg`.
    #[rustfmt::skip]
    pub const fn COLORS(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 8] {
        [ b'\x1b', b'[', C::FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
    }

    /// Code to set the foreground color to bright `fg` and the background to bright `bg`.
    #[rustfmt::skip]
    pub const fn COLORS_BRIGHT(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 9] {
        [
            b'\x1b', b'[',
            C::BRI_FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
            b'm',
        ]
    }

    /// Code to set the foreground color to bright `fg` and the background to `bg`.
    #[rustfmt::skip]
    pub const fn COLORS_BRIGHT_FG(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 8] {
        [ b'\x1b', b'[', C::BRI_FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
    }

    /// Code to set the foreground color to `fg` and the background to bright `bg`.
    #[rustfmt::skip]
    pub const fn COLORS_BRIGHT_BG(fg: AnsiColor8, bg: AnsiColor8) -> [u8; 9] {
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
    #[rustfmt::skip]
    pub const BRIGHT_YELLOW: [u8; 5] = [
        b'\x1b', b'[', C::BRI_YELLOW_FG[0], C::BRI_YELLOW_FG[1], b'm'];
    /// Code to set the foreground color to bright blue.
    pub const BRIGHT_BLUE: [u8; 5] = [b'\x1b', b'[', C::BRI_BLUE_FG[0], C::BRI_BLUE_FG[1], b'm'];
    /// Code to set the foreground color to bright magenta.
    #[rustfmt::skip]
    pub const BRIGHT_MAGENTA: [u8; 5] = [
        b'\x1b', b'[', C::BRI_MAGENTA_FG[0], C::BRI_MAGENTA_FG[1], b'm'];
    /// Code to set the foreground color to bright cyan.
    pub const BRIGHT_CYAN: [u8; 5] = [b'\x1b', b'[', C::BRI_CYAN_FG[0], C::BRI_CYAN_FG[1], b'm'];
    /// Code to set the foreground color to bright white.
    pub const BRIGHT_WHITE: [u8; 5] = [b'\x1b', b'[', C::BRI_WHITE_FG[0], C::BRI_WHITE_FG[1], b'm'];

    /// Code to set the background color to bright black.
    #[rustfmt::skip]
    pub const BRIGHT_BLACK_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_BLACK_BG[0], C::BRI_BLACK_BG[1], C::BRI_BLACK_BG[2], b'm'
    ];
    /// Code to set the background color to bright red.
    #[rustfmt::skip]
    pub const BRIGHT_RED_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_RED_BG[0], C::BRI_RED_BG[1], C::BRI_RED_BG[2], b'm',
    ];
    /// Code to set the background color to bright green.
    #[rustfmt::skip]
    pub const BRIGHT_GREEN_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_GREEN_BG[0], C::BRI_GREEN_BG[1], C::BRI_GREEN_BG[2], b'm',
    ];
    /// Code to set the background color to bright yellow.
    #[rustfmt::skip]
    pub const BRIGHT_YELLOW_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_YELLOW_BG[0], C::BRI_YELLOW_BG[1], C::BRI_YELLOW_BG[2], b'm',
    ];
    /// Code to set the background color to bright blue.
    #[rustfmt::skip]
    pub const BRIGHT_BLUE_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_BLUE_BG[0], C::BRI_BLUE_BG[1], C::BRI_BLUE_BG[2], b'm',
    ];
    /// Code to set the background color to bright magenta.
    #[rustfmt::skip]
    pub const BRIGHT_MAGENTA_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_MAGENTA_BG[0], C::BRI_MAGENTA_BG[1], C::BRI_MAGENTA_BG[2], b'm',
    ];
    /// Code to set the background color to bright cyan.
    #[rustfmt::skip]
    pub const BRIGHT_CYAN_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_CYAN_BG[0], C::BRI_CYAN_BG[1], C::BRI_CYAN_BG[2], b'm',
    ];
    /// Code to set the background color to bright white.
    #[rustfmt::skip]
    pub const BRIGHT_WHITE_BG: [u8; 6] = [
        b'\x1b', b'[', C::BRI_WHITE_BG[0], C::BRI_WHITE_BG[1], C::BRI_WHITE_BG[2], b'm'
    ];
}

/// # 8-bit Color escape codes
impl Ansi {
    /// Code to set the foreground color to `fg` and the background to `bg`.
    #[rustfmt::skip]
    pub const fn COLORS256(fg: AnsiColor256, bg: AnsiColor256) -> [u8; 19] {
        const X: [u8; 4] = C::C256;
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
    #[rustfmt::skip]
    pub const fn COLOR256_FG(fg: AnsiColor256) -> [u8; 11] {
        const X: [u8; 4] = C::C256;
        let c = fg.to_ascii();
        [ b'\x1b', b'[', C::FG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
    }

    /// Code to set the background color to `bg`.
    #[rustfmt::skip]
    pub const fn COLOR256_BG(bg: AnsiColor256) -> [u8; 11] {
        const X: [u8; 4] = C::C256;
        let c = bg.to_ascii();
        [ b'\x1b', b'[', C::BG, X[0], X[1], X[2], X[3], c[0], c[1], c[2], b'm' ]
    }

    /// Ansi gray 0/23 8-bit color (4% white).
    pub const GRAY0: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(0));
    /// Ansi gray 1/23 8-bit color (8% white).
    pub const GRAY1: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(1));
    /// Ansi gray 2/23 8-bit color (12% white).
    pub const GRAY2: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(2));
    /// Ansi gray 3/23 8-bit color (16% white).
    pub const GRAY3: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(3));
    /// Ansi gray 4/23 8-bit color (20% white).
    pub const GRAY4: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(4));
    /// Ansi gray 5/23 8-bit color (24% white).
    pub const GRAY5: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(5));
    /// Ansi gray 6/23 8-bit color (28% white).
    pub const GRAY6: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(6));
    /// Ansi gray 7/23 8-bit color (32% white).
    pub const GRAY7: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(7));
    /// Ansi gray 8/23 8-bit color (36% white).
    pub const GRAY8: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(8));
    /// Ansi gray 9/23 8-bit color (40% white).
    pub const GRAY9: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(9));
    /// Ansi gray 10/23 8-bit color (44% white).
    pub const GRAY10: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(10));
    /// Ansi gray 11/23 8-bit color (48% white).
    pub const GRAY11: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(11));
    /// Ansi gray 12/23 8-bit color (52% white).
    pub const GRAY12: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(12));
    /// Ansi gray 13/23 8-bit color (56% white).
    pub const GRAY13: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(13));
    /// Ansi gray 14/23 8-bit color (60% white).
    pub const GRAY14: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(14));
    /// Ansi gray 15/23 8-bit color (64% white).
    pub const GRAY15: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(15));
    /// Ansi gray 16/23 8-bit color (68% white).
    pub const GRAY16: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(16));
    /// Ansi gray 17/23 8-bit color (72% white).
    pub const GRAY17: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(17));
    /// Ansi gray 18/23 8-bit color (76% white).
    pub const GRAY18: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(18));
    /// Ansi gray 19/23 8-bit color (80% white).
    pub const GRAY19: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(19));
    /// Ansi gray 20/23 8-bit color (84% white).
    pub const GRAY20: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(20));
    /// Ansi gray 21/23 8-bit color (88% white).
    pub const GRAY21: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(21));
    /// Ansi gray 22/23 8-bit color (92% white).
    pub const GRAY22: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(22));
    /// Ansi gray 23/23 8-bit color (96% white).
    pub const GRAY23: [u8; 11] = Self::COLOR256_FG(AnsiColor256::new_gray(23));
}

/// # RGB Color escape codes
impl Ansi {
    /// Code to set the foreground color to `fg: [r, g, b]` values,
    /// and the background to `bg: [r, g, b]`.
    #[rustfmt::skip]
    pub const fn RGB(fg: [u8; 3], bg: [u8; 3]) -> [u8; 35] {
        const X: [u8; 4] = C::RGB;
        let [fr, fg, fb] = [ascii_d3(fg[0] as u16), ascii_d3(fg[1] as u16), ascii_d3(fg[2] as u16)];
        let [br, bg, bb] = [ascii_d3(bg[0] as u16), ascii_d3(bg[1] as u16), ascii_d3(bg[2] as u16)];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            fr[0], fr[1], fr[2], b';', fg[0], fg[1], fg[2], b';', fb[0], fb[1], fb[2],
            C::BG, X[0], X[1], X[2], X[3],
            br[0], br[1], br[2], b';', bg[0], bg[1], bg[2], b';', bb[0], bb[1], bb[2],
            b'm',
        ]
    }

    #[rustfmt::skip]
    pub const fn RGB_FG(r: u8, g: u8, b: u8) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = [ascii_d3(r as u16), ascii_d3(g as u16), ascii_d3(b as u16)];
        [
            b'\x1b', b'[',
            C::FG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm'
        ]
    }

    #[rustfmt::skip]
    pub const fn RGB_BG(r: u8, g: u8, b: u8) -> [u8; 19] {
        const X: [u8; 4] = C::RGB;
        let [r, g, b] = [ascii_d3(r as u16), ascii_d3(g as u16), ascii_d3(b as u16)];
        [
            b'\x1b', b'[',
            C::BG, X[0], X[1], X[2], X[3],
            r[0], r[1], r[2], b';', g[0], g[1], g[2], b';', b[0], b[1], b[2],
            b'm',
        ]
    }
}
