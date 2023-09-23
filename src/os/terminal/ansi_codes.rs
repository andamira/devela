// devela::os::terminal::ansi
//
//! ANSI codes.
//
#![allow(non_snake_case)]

use crate::ascii::{ascii_d1, ascii_d2, ascii_d3, ascii_d4};

/// ANSI escape codes.
///
/// # List of escape codes
/// - [Screen][Self#screen-escape-codes]
/// - [Cursor][Self#cursor-escape-codes]
/// - [Font effects][Self#font-effects-escape-codes]
/// - [Color][Self#color-escape-codes]
pub struct Ansi;

/// # Screen escape codes
impl Ansi {
    /// Code to clear the screen.
    pub const CLEAR_SCREEN: &'static [u8] = b"\x1b[2J";

    /// Code to enable the alternative screen.
    pub const ENABLE_ALT_SCREEN: &'static [u8] = b"\x1b[1049h";

    /// Code to disable the alternative screen.
    pub const DISABLE_ALT_SCREEN: &'static [u8] = b"\x1b[1049l";
}

/// # Cursor escape codes
impl Ansi {
    /// Code to make the cursor invisible.
    pub const CURSOR_INVISIBLE: &'static [u8] = b"\x1b[?25l";
    /// Code to make the cursor visible.
    pub const CURSOR_VISIBLE: &'static [u8] = b"\x1b[?25h";

    /// Code to save the cursor position.
    pub const CURSOR_SAVE: &'static [u8] = b"\x1b[s";
    /// Code to estore the cursor position.
    pub const CURSOR_RESTORE: &'static [u8] = b"\x1b[u";

    /// Code to move the cursor to the home position (1, 1).
    pub const CURSOR_HOME: &'static [u8] = b"\x1b[H";

    /// Code to move the cursor to the specified 1-digit position (row, col).
    /// # Panics
    /// Panics in debug if either `row` or `col` > 9.
    #[inline]
    pub const fn CURSOR_MOVE1(row: u8, col: u8) -> [u8; 6] {
        [b'\x1b', b'[', ascii_d1(row), b';', ascii_d1(col), b'H']
    }
    /// Code to move the cursor to the specified 2-digit position (row, col).
    /// # Panics
    /// Panics in debug if either `row` or `col` > 99.
    #[inline]
    pub const fn CURSOR_MOVE2(row: u8, col: u8) -> [u8; 8] {
        let r: [u8; 2] = ascii_d2(row);
        let c: [u8; 2] = ascii_d2(col);
        [b'\x1b', b'[', r[0], r[1], b';', c[0], c[1], b'H']
    }
    /// Code to move the cursor to the specified 3-digit position (row, col).
    /// # Panics
    /// Panics in debug if either `row` or `col` > 999.
    #[inline]
    pub const fn CURSOR_MOVE3(row: u16, col: u16) -> [u8; 10] {
        let r: [u8; 3] = ascii_d3(row);
        let c: [u8; 3] = ascii_d3(col);
        [
            b'\x1b', b'[', r[0], r[1], r[2], b';', c[0], c[1], c[2], b'H',
        ]
    }
    /// Code to move the cursor to the specified 4-digit position (row, col).
    /// # Panics
    /// Panics in debug if either `row` or `col` > 9999.
    #[inline]
    pub const fn CURSOR_MOVE4(row: u16, col: u16) -> [u8; 12] {
        let r: [u8; 4] = ascii_d4(row);
        let c: [u8; 4] = ascii_d4(col);
        [
            b'\x1b', b'[', r[0], r[1], r[2], r[3], b';', c[0], c[1], c[2], c[3], b'H',
        ]
    }

    /// Code to move the cursor up by one line.
    pub const CURSOR_UP: &'static [u8] = b"\x1b[A";
    /// Code to move the cursor up by 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_UP1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'A']
    }
    /// Code to move the cursor up by 2-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_UP2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'A']
    }
    /// Code to move the cursor up by 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_UP3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'A']
    }
    /// Code to move the cursor up by 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9999.
    #[inline]
    pub const fn CURSOR_UP4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'A']
    }

    /// Code to move the cursor down by one line.
    pub const CURSOR_DOWN: &'static [u8] = b"\x1b[B";
    /// Code to move the cursor down by 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_DOWN1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'B']
    }
    /// Code to move the cursor down by 2-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_DOWN2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'B']
    }
    /// Code to move the cursor down by 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_DOWN3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'B']
    }
    /// Code to move the cursor down by 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_DOWN4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'B']
    }

    /// Code to move the cursor right by one column.
    pub const CURSOR_RIGHT: &'static [u8] = b"\x1b[C";
    /// Code to move the cursor right by 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_RIGHT1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'C']
    }
    /// Code to move the cursor right by 2-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_RIGHT2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'C']
    }
    /// Code to move the cursor right by 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_RIGHT3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'C']
    }
    /// Code to move the cursor right by 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9999.
    #[inline]
    pub const fn CURSOR_RIGHT4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'C']
    }

    /// Code to move the cursor left by one column.
    pub const CURSOR_LEFT: &'static [u8] = b"\x1b[D";
    /// Code to move the cursor left by 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_LEFT1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'D']
    }
    /// Code to move the cursor left by 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_LEFT2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'D']
    }
    /// Code to move the cursor left by 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_LEFT3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'D']
    }
    /// Code to move the cursor left by 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9999.
    #[inline]
    pub const fn CURSOR_LEFT4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'D']
    }
}

/// # Font effects escape codes
impl Ansi {
    /// Code to turn off all effects and colors.
    pub const RESET: &'static [u8] = b"\x1b[0m";

    /// Code to set bold effect.
    pub const BOLD: &'static [u8] = b"\x1b[1m";
    /// Code to unset bold and dim effects.
    pub const BOLD_OFF: &'static [u8] = b"\x1b[22m";

    /// Code to set dim effect.
    pub const DIM: &'static [u8] = b"\x1b[2m";
    /// Code to unset bold and dim effects.
    pub const DIM_OFF: &'static [u8] = b"\x1b[22m";

    /// Code to set italic effect.
    pub const ITALIC: &'static [u8] = b"\x1b[3m";
    /// Code to unset italic and fraktur effects.
    pub const ITALIC_OFF: &'static [u8] = b"\x1b[23m";

    /// Code to set underline effect.
    pub const UNDERLINE: &'static [u8] = b"\x1b[4m";
    // pub const UNDERLINE_DOUBLE: &'static [u8] = b"\x1b[21m"; // or bold_off
    /// Code to unset underline effect.
    pub const UNDERLINE_OFF: &'static [u8] = b"\x1b[24m";

    /// Code to set blink effect.
    pub const BLINK: &'static [u8] = b"\x1b[5m";
    /// Code to unset blink effect.
    pub const BLINK_OFF: &'static [u8] = b"\x1b[25m";
    // pub const BLINK_FAST: &'static [u8] = b"\x1b[6m";

    /// Code to set inverse effect.
    pub const INVERSE: &'static [u8] = b"\x1b[7m";
    /// Code to unset inverse effect.
    pub const INVERSE_OFF: &'static [u8] = b"\x1b[27m";

    /// Code to set crossed effect.
    pub const CROSSED: &'static [u8] = b"\x1b[9m";
    /// Code to unset crossed effect.
    pub const CROSSED_OFF: &'static [u8] = b"\x1b[29m";
}

// the bare color escape codes
mod Color {
    pub const FG: u8 = b'3';
    pub const BG: u8 = b'4';
    pub const BRIGHT_FG: u8 = b'9';
    pub const BRIGHT_BG: [u8; 2] = [b'1', b'0'];
    //
    pub const BLACK: u8 = b'0';
    pub const RED: u8 = b'1';
    pub const GREEN: u8 = b'2';
    pub const YELLOW: u8 = b'3';
    pub const BLUE: u8 = b'4';
    pub const MAGENTA: u8 = b'5';
    pub const CYAN: u8 = b'6';
    pub const WHITE: u8 = b'7';
    //
    pub const BLACK_FG: &[u8; 2] = &[FG, BLACK];
    pub const RED_FG: &[u8; 2] = &[FG, RED];
    pub const GREEN_FG: &[u8; 2] = &[FG, GREEN];
    pub const YELLOW_FG: &[u8; 2] = &[FG, YELLOW];
    pub const BLUE_FG: &[u8; 2] = &[FG, BLUE];
    pub const MAGENTA_FG: &[u8; 2] = &[FG, MAGENTA];
    pub const CYAN_FG: &[u8; 2] = &[FG, CYAN];
    pub const WHITE_FG: &[u8; 2] = &[FG, WHITE];

    pub const BLACK_BG: &[u8; 2] = &[BG, BLACK];
    pub const RED_BG: &[u8; 2] = &[BG, RED];
    pub const GREEN_BG: &[u8; 2] = &[BG, GREEN];
    pub const YELLOW_BG: &[u8; 2] = &[BG, YELLOW];
    pub const BLUE_BG: &[u8; 2] = &[BG, BLUE];
    pub const MAGENTA_BG: &[u8; 2] = &[BG, MAGENTA];
    pub const CYAN_BG: &[u8; 2] = &[BG, CYAN];
    pub const WHITE_BG: &[u8; 2] = &[BG, WHITE];

    pub const BRIGHT_BLACK_FG: &[u8; 2] = &[BRIGHT_FG, BLACK];
    pub const BRIGHT_RED_FG: &[u8; 2] = &[BRIGHT_FG, RED];
    pub const BRIGHT_GREEN_FG: &[u8; 2] = &[BRIGHT_FG, GREEN];
    pub const BRIGHT_YELLOW_FG: &[u8; 2] = &[BRIGHT_FG, YELLOW];
    pub const BRIGHT_BLUE_FG: &[u8; 2] = &[BRIGHT_FG, BLUE];
    pub const BRIGHT_MAGENTA_FG: &[u8; 2] = &[BRIGHT_FG, MAGENTA];
    pub const BRIGHT_CYAN_FG: &[u8; 2] = &[BRIGHT_FG, CYAN];
    pub const BRIGHT_WHITE_FG: &[u8; 2] = &[BRIGHT_FG, WHITE];

    pub const BRIGHT_BLACK_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], BLACK];
    pub const BRIGHT_RED_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], RED];
    pub const BRIGHT_GREEN_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], GREEN];
    pub const BRIGHT_YELLOW_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], YELLOW];
    pub const BRIGHT_BLUE_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], BLUE];
    pub const BRIGHT_MAGENTA_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], MAGENTA];
    pub const BRIGHT_CYAN_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], CYAN];
    pub const BRIGHT_WHITE_BG: &[u8; 3] = &[BRIGHT_BG[0], BRIGHT_BG[1], WHITE];
}

/// # Color escape codes
impl Ansi {
    /* 4-bit colors*/

    // all &[u8; 5] except bright background colors &[u8; 6]

    /// Code to set the foreground color to black.
    pub const BLACK: &'static [u8] = &[b'\x1b', b'[', Color::BLACK_FG[0], Color::BLACK_FG[1], b'm'];
    /// Code to set the foreground color to red.
    pub const RED: &'static [u8] = &[b'\x1b', b'[', Color::RED_FG[0], Color::RED_FG[1], b'm'];
    /// Code to set the foreground color to green.
    pub const GREEN: &'static [u8] = &[b'\x1b', b'[', Color::GREEN_FG[0], Color::GREEN_FG[1], b'm'];
    /// Code to set the foreground color to yellow.
    pub const YELLOW: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::YELLOW_FG[0],
        Color::YELLOW_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to blue.
    pub const BLUE: &'static [u8] = &[b'\x1b', b'[', Color::BLUE_FG[0], Color::BLUE_FG[1], b'm'];
    /// Code to set the foreground color to magenta.
    pub const MAGENTA: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::MAGENTA_FG[0],
        Color::MAGENTA_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to cyan.
    pub const CYAN: &'static [u8] = &[b'\x1b', b'[', Color::CYAN_FG[0], Color::CYAN_FG[1], b'm'];
    /// Code to set the foreground color to white.
    pub const WHITE: &'static [u8] = &[b'\x1b', b'[', Color::WHITE_FG[0], Color::WHITE_FG[1], b'm'];

    /// Code to set the background color to black.
    pub const BLACK_BG: &'static [u8] =
        &[b'\x1b', b'[', Color::BLACK_BG[0], Color::BLACK_BG[1], b'm'];
    /// Code to set the background color to red.
    pub const RED_BG: &'static [u8] = &[b'\x1b', b'[', Color::RED_BG[0], Color::RED_BG[1], b'm'];
    /// Code to set the background color to green.
    pub const GREEN_BG: &'static [u8] =
        &[b'\x1b', b'[', Color::GREEN_BG[0], Color::GREEN_BG[1], b'm'];
    /// Code to set the background color to yellow.
    pub const YELLOW_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::YELLOW_BG[0],
        Color::YELLOW_BG[1],
        b'm',
    ];
    /// Code to set the background color to blue.
    pub const BLUE_BG: &'static [u8] = &[b'\x1b', b'[', Color::BLUE_BG[0], Color::BLUE_BG[1], b'm'];
    /// Code to set the background color to magenta.
    pub const MAGENTA_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::MAGENTA_BG[0],
        Color::MAGENTA_BG[1],
        b'm',
    ];
    /// Code to set the background color to cyan.
    pub const CYAN_BG: &'static [u8] = &[b'\x1b', b'[', Color::CYAN_BG[0], Color::CYAN_BG[1], b'm'];
    /// Code to set the background color to white.
    pub const WHITE_BG: &'static [u8] =
        &[b'\x1b', b'[', Color::WHITE_BG[0], Color::WHITE_BG[1], b'm'];

    /// Code to set the foreground color to bright black.
    pub const BRIGHT_BLACK: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_BLACK_FG[0],
        Color::BRIGHT_BLACK_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright red.
    pub const BRIGHT_RED: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_RED_FG[0],
        Color::BRIGHT_RED_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright green.
    pub const BRIGHT_GREEN: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_GREEN_FG[0],
        Color::BRIGHT_GREEN_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright yellow.
    pub const BRIGHT_YELLOW: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_YELLOW_FG[0],
        Color::BRIGHT_YELLOW_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright blue.
    pub const BRIGHT_BLUE: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_BLUE_FG[0],
        Color::BRIGHT_BLUE_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright magenta.
    pub const BRIGHT_MAGENTA: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_MAGENTA_FG[0],
        Color::BRIGHT_MAGENTA_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright cyan.
    pub const BRIGHT_CYAN: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_CYAN_FG[0],
        Color::BRIGHT_CYAN_FG[1],
        b'm',
    ];
    /// Code to set the foreground color to bright white.
    pub const BRIGHT_WHITE: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_WHITE_FG[0],
        Color::BRIGHT_WHITE_FG[1],
        b'm',
    ];

    /// Code to set the background color to bright black.
    pub const BRIGHT_BLACK_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_BLACK_BG[0],
        Color::BRIGHT_BLACK_BG[1],
        Color::BRIGHT_BLACK_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright red.
    pub const BRIGHT_RED_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_RED_BG[0],
        Color::BRIGHT_RED_BG[1],
        Color::BRIGHT_RED_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright green.
    pub const BRIGHT_GREEN_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_GREEN_BG[0],
        Color::BRIGHT_GREEN_BG[1],
        Color::BRIGHT_GREEN_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright yellow.
    pub const BRIGHT_YELLOW_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_YELLOW_BG[0],
        Color::BRIGHT_YELLOW_BG[1],
        Color::BRIGHT_YELLOW_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright blue.
    pub const BRIGHT_BLUE_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_BLUE_BG[0],
        Color::BRIGHT_BLUE_BG[1],
        Color::BRIGHT_BLUE_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright magenta.
    pub const BRIGHT_MAGENTA_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_MAGENTA_BG[0],
        Color::BRIGHT_MAGENTA_BG[1],
        Color::BRIGHT_MAGENTA_BG[2],
        b'm',
    ]; // 6
    /// Code to set the background color to bright cyan.
    pub const BRIGHT_CYAN_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_CYAN_BG[0],
        Color::BRIGHT_CYAN_BG[1],
        Color::BRIGHT_CYAN_BG[2],
        b'm',
    ];
    /// Code to set the background color to bright white.
    pub const BRIGHT_WHITE_BG: &'static [u8] = &[
        b'\x1b',
        b'[',
        Color::BRIGHT_WHITE_BG[0],
        Color::BRIGHT_WHITE_BG[1],
        Color::BRIGHT_WHITE_BG[2],
        b'm',
    ];
}
