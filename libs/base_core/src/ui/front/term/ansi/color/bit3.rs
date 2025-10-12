// devela::ui::front::term::ansi::color
//
//! ANSI codes related to 3bit color.
//

use super::C;
use crate::{Ansi, Digits};

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
}
// # 3-bit Color escape codes constants
#[rustfmt::skip]
impl Ansi {
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
