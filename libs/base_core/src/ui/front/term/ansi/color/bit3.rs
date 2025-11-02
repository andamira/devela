// devela_base_core::ui::front::term::ansi::color
//
//! ANSI codes related to 3bit color.
//

use super::C;
use crate::{_ansi_consts, Ansi, Digits};

/// ANSI 3-bit color codes, 8 colors.
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

impl From<u8> for AnsiColor3 {
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

/// # 3-bit Color escape codes
#[rustfmt::skip]
impl Ansi {
    _ansi_consts! {
        /// Code to set the the foreground `color`.
        pub const fn COLOR_FG(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::FG, color.to_ascii(), b'm']
        }
        /// Code to set the the foreground `color` bright.
        pub const fn COLOR_FG_BRIGHT(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::BRI_FG, color.to_ascii(), b'm']
        }

        /// Code to set the the background `color`.
        pub const fn COLOR_BG(color: AnsiColor3) -> [u8; 5] {
            [b'\x1b', b'[', C::BG, color.to_ascii(), b'm']
        }
        /// Code to set the the background `color` bright.
        pub const fn COLOR_BG_BRIGHT(color: AnsiColor3) -> [u8; 6] {
            [b'\x1b', b'[', C::BRI_BG[0], C::BRI_BG[1], color.to_ascii(), b'm']
        }

        /// Code to set the foreground color to `fg` and the background to `bg`.
        pub const fn COLORS(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
            [ b'\x1b', b'[', C::FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
        }
        /// Code to set the foreground color to bright `fg` and the background to bright `bg`.
        pub const fn COLORS_BRIGHT(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
            [
                b'\x1b', b'[',
                C::BRI_FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
                b'm',
            ]
        }

        /// Code to set the foreground color to bright `fg` and the background to `bg`.
        pub const fn COLORS_BRIGHT_FG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 8] {
            [ b'\x1b', b'[', C::BRI_FG, fg.to_ascii(), b';', C::BG, bg.to_ascii(), b'm' ]
        }

        /// Code to set the foreground color to `fg` and the background to bright `bg`.
        pub const fn COLORS_BRIGHT_BG(fg: AnsiColor3, bg: AnsiColor3) -> [u8; 9] {
            [
                b'\x1b', b'[',
                C::FG, fg.to_ascii(), b';', C::BRI_BG[0], C::BRI_BG[1], bg.to_ascii(),
                b'm',
            ]
        }
    }
}
// # 3-bit Color escape codes constants
#[rustfmt::skip]
impl Ansi {
    _ansi_consts! {
        /// Code to set the foreground color to black.
        pub const BLACK: [u8; 5] = "\x1b[30m", *b"\x1b[30m";
        /// Code to set the foreground color to red.
        pub const RED: [u8; 5] = "\x1b[31m", *b"\x1b[31m";
        /// Code to set the foreground color to green.
        pub const GREEN: [u8; 5] = "\x1b[32m", *b"\x1b[32m";
        /// Code to set the foreground color to yellow.
        pub const YELLOW: [u8; 5] = "\x1b[33m", *b"\x1b[33m";
        /// Code to set the foreground color to blue.
        pub const BLUE: [u8; 5] = "\x1b[34m", *b"\x1b[34m";
        /// Code to set the foreground color to magenta.
        pub const MAGENTA: [u8; 5] = "\x1b[35m", *b"\x1b[35m";
        /// Code to set the foreground color to cyan.
        pub const CYAN: [u8; 5] = "\x1b[36m", *b"\x1b[36m";
        /// Code to set the foreground color to white.
        pub const WHITE: [u8; 5] = "\x1b[37m", *b"\x1b[37m";

        /// Code to set the background color to black.
        pub const BLACK_BG: [u8; 5] = "\x1b[40m", *b"\x1b[40m";
        /// Code to set the background color to red.
        pub const RED_BG: [u8; 5] = "\x1b[41m", *b"\x1b[41m";
        /// Code to set the background color to green.
        pub const GREEN_BG: [u8; 5] = "\x1b[42m", *b"\x1b[42m";
        /// Code to set the background color to yellow.
        pub const YELLOW_BG: [u8; 5] = "\x1b[43m", *b"\x1b[43m";
        /// Code to set the background color to blue.
        pub const BLUE_BG: [u8; 5] = "\x1b[44m", *b"\x1b[44m";
        /// Code to set the background color to magenta.
        pub const MAGENTA_BG: [u8; 5] = "\x1b[45m", *b"\x1b[45m";
        /// Code to set the background color to cyan.
        pub const CYAN_BG: [u8; 5] = "\x1b[46m", *b"\x1b[46m";
        /// Code to set the background color to white.
        pub const WHITE_BG: [u8; 5] = "\x1b[47m", *b"\x1b[47m";

        /// Code to set the foreground color to bright black.
        pub const BRIGHT_BLACK: [u8; 5] = "\x1b[90m", *b"\x1b[90m";
        /// Code to set the foreground color to bright red.
        pub const BRIGHT_RED: [u8; 5] = "\x1b[91m", *b"\x1b[91m";
        /// Code to set the foreground color to bright green.
        pub const BRIGHT_GREEN: [u8; 5] = "\x1b[92m", *b"\x1b[92m";
        /// Code to set the foreground color to bright yellow.
        pub const BRIGHT_YELLOW: [u8; 5] = "\x1b[93m", *b"\x1b[93m";
        /// Code to set the foreground color to bright blue.
        pub const BRIGHT_BLUE: [u8; 5] = "\x1b[94m", *b"\x1b[94m";
        /// Code to set the foreground color to bright magenta.
        pub const BRIGHT_MAGENTA: [u8; 5] = "\x1b[95m", *b"\x1b[95m";
        /// Code to set the foreground color to bright cyan.
        pub const BRIGHT_CYAN: [u8; 5] = "\x1b[96m", *b"\x1b[96m";
        /// Code to set the foreground color to bright white.
        pub const BRIGHT_WHITE: [u8; 5] = "\x1b[97m", *b"\x1b[97m";

        /// Code to set the background color to bright black.
        pub const BRIGHT_BLACK_BG: [u8; 6] = "\x1b[100m", *b"\x1b[100m";
        /// Code to set the background color to bright red.
        pub const BRIGHT_RED_BG: [u8; 6] = "\x1b[101m", *b"\x1b[101m";
        /// Code to set the background color to bright green.
        pub const BRIGHT_GREEN_BG: [u8; 6] = "\x1b[102m", *b"\x1b[102m";
        /// Code to set the background color to bright yellow.
        pub const BRIGHT_YELLOW_BG: [u8; 6] = "\x1b[103m", *b"\x1b[103m";
        /// Code to set the background color to bright blue.
        pub const BRIGHT_BLUE_BG: [u8; 6] = "\x1b[104m", *b"\x1b[104m";
        /// Code to set the background color to bright magenta.
        pub const BRIGHT_MAGENTA_BG: [u8; 6] = "\x1b[105m", *b"\x1b[105m";
        /// Code to set the background color to bright cyan.
        pub const BRIGHT_CYAN_BG: [u8; 6] = "\x1b[106m", *b"\x1b[106m";
        /// Code to set the background color to bright white.
        pub const BRIGHT_WHITE_BG: [u8; 6] = "\x1b[107m", *b"\x1b[107m";
    }
}
