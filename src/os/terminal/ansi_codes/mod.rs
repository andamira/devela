// devela::os::terminal::ansi
//
//! ANSI codes.
//
#![allow(non_snake_case)]

use crate::ascii::{ascii_d1, ascii_d2, ascii_d3, ascii_d4};

mod color;
pub use color::{AnsiColor256, AnsiColor8};

mod print;

/// ANSI escape codes.
///
/// # List of escape codes
/// - [Color (4-bit)][Self#4-bit-color-escape-codes]
/// - [Color (8-bit)][Self#8-bit-color-escape-codes]
/// - [Color (rgb)][Self#rgb-color-escape-codes]
/// - [Screen][Self#screen-escape-codes]
/// - [Erase][Self#erase-escape-codes]
/// - [Cursor][Self#cursor-escape-codes]
/// - [Font effects][Self#font-effects-escape-codes]
/// - [Print method](Self#print-method)
pub struct Ansi;

impl Ansi {
    /// Control Sequence Introducer (CSI).
    ///
    /// <https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences>
    pub const CSI: [u8; 2] = *b"\x1b[";
}

/// # Screen escape codes
impl Ansi {
    /// Code to enable the alternative screen.
    pub const ENABLE_ALT_SCREEN: [u8; 7] = *b"\x1b[1049h";

    /// Code to disable the alternative screen.
    pub const DISABLE_ALT_SCREEN: [u8; 7] = *b"\x1b[1049l";
}

/// # Erase escape codes
impl Ansi {
    /// Code to erase from the cursor to the end of the line.
    pub const ERASE_END_LINE: [u8; 3] = *b"\x1b[K"; // also "\x1b[0K"

    /// Code to erase from the cursor to the start of the line.
    pub const ERASE_START_LINE: [u8; 4] = *b"\x1b[1K";

    /// Code to erase the entire line.
    pub const ERASE_LINE: [u8; 4] = *b"\x1b[2K";

    /// Code to erase from the cursor to the end of the screen.
    pub const ERASE_END_SCREEN: [u8; 3] = *b"\x1b[J"; // also "\x1b[0J"

    /// Code to erase from the cursor to the start of the screen.
    pub const ERASE_START_SCREEN: [u8; 4] = *b"\x1b[1J";

    /// Code to erase the entire screen.
    pub const ERASE_SCREEN: [u8; 4] = *b"\x1b[2J";
}

/// # Cursor escape codes
impl Ansi {
    /// Code to make the cursor invisible.
    pub const CURSOR_INVISIBLE: [u8; 6] = *b"\x1b[?25l";
    /// Code to make the cursor visible.
    pub const CURSOR_VISIBLE: [u8; 6] = *b"\x1b[?25h";

    /// Code to save the cursor position.
    pub const CURSOR_SAVE: [u8; 3] = *b"\x1b[s";
    /// Code to restore the cursor position.
    pub const CURSOR_RESTORE: [u8; 3] = *b"\x1b[u";

    /// Code to move the cursor to the home position (1, 1).
    pub const CURSOR_HOME: [u8; 3] = *b"\x1b[H";

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
    pub const CURSOR_UP: [u8; 3] = *b"\x1b[A";

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
    pub const CURSOR_DOWN: [u8; 3] = *b"\x1b[B";

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
    pub const CURSOR_RIGHT: [u8; 3] = *b"\x1b[C";

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
    pub const CURSOR_LEFT: [u8; 3] = *b"\x1b[D";

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

    /// Code to move the cursor to the beginning of the next line.
    pub const CURSOR_NEXT_LINE: [u8; 3] = *b"\x1b[E";
    /// Code to move the cursor to the beginning of the next 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_NEXT_LINE1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'E']
    }
    /// Code to move the cursor to the beginning of the next 2-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_NEXT_LINE2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'E']
    }
    /// Code to move the cursor to the beginning of the next 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_NEXT_LINE3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'E']
    }
    /// Code to move the cursor to the beginning of the next 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_NEXT_LINE4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'E']
    }

    /// Code to move the cursor to the beginning of the previous line.
    pub const CURSOR_PREV_LINE: [u8; 3] = *b"\x1b[E";
    /// Code to move the cursor to the beginning of the previous 1-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 9.
    #[inline]
    pub const fn CURSOR_PREV_LINE1(n: u8) -> [u8; 4] {
        [b'\x1b', b'[', ascii_d1(n), b'E']
    }
    /// Code to move the cursor to the beginning of the previous 2-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 99.
    #[inline]
    pub const fn CURSOR_PREV_LINE2(n: u8) -> [u8; 5] {
        let n: [u8; 2] = ascii_d2(n);
        [b'\x1b', b'[', n[0], n[1], b'E']
    }
    /// Code to move the cursor to the beginning of the previous 3-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_PREV_LINE3(n: u16) -> [u8; 6] {
        let n: [u8; 3] = ascii_d3(n);
        [b'\x1b', b'[', n[0], n[1], n[2], b'E']
    }
    /// Code to move the cursor to the beginning of the previous 4-digit `n` lines.
    /// # Panics
    /// Panics in debug if `n` > 999.
    #[inline]
    pub const fn CURSOR_PREV_LINE4(n: u16) -> [u8; 7] {
        let n: [u8; 4] = ascii_d4(n);
        [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'E']
    }
}

/// # Font effects escape codes
impl Ansi {
    /// Code to turn off all effects and colors.
    pub const RESET: [u8; 4] = *b"\x1b[0m";

    /// Code to set bold effect.
    pub const BOLD: [u8; 4] = *b"\x1b[1m";
    /// Code to unset bold and dim effects.
    pub const BOLD_OFF: [u8; 5] = *b"\x1b[22m";

    /// Code to set dim effect.
    pub const DIM: [u8; 4] = *b"\x1b[2m";
    /// Code to unset bold and dim effects.
    pub const DIM_OFF: [u8; 5] = *b"\x1b[22m";

    /// Code to set italic effect.
    pub const ITALIC: [u8; 4] = *b"\x1b[3m";
    /// Code to unset italic and fraktur effects.
    pub const ITALIC_OFF: [u8; 5] = *b"\x1b[23m";

    /// Code to set underline effect.
    pub const UNDERLINE: [u8; 4] = *b"\x1b[4m";
    // pub const UNDERLINE_DOUBLE: [u8; 5] = b"\x1b[21m"; // or bold_off
    /// Code to unset underline effect.
    pub const UNDERLINE_OFF: [u8; 5] = *b"\x1b[24m";

    /// Code to set blink effect.
    pub const BLINK: [u8; 4] = *b"\x1b[5m";
    /// Code to unset blink effect.
    pub const BLINK_OFF: [u8; 5] = *b"\x1b[25m";
    // pub const BLINK_FAST: &'static [u8] = b"\x1b[6m";

    /// Code to set inverse effect.
    pub const INVERSE: [u8; 4] = *b"\x1b[7m";
    /// Code to unset inverse effect.
    pub const INVERSE_OFF: [u8; 5] = *b"\x1b[27m";

    /// Code to set crossed effect.
    pub const CROSSED: [u8; 4] = *b"\x1b[9m";
    /// Code to unset crossed effect.
    pub const CROSSED_OFF: [u8; 5] = *b"\x1b[29m";
}
