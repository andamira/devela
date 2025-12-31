// devela_base_core::ui::front::term::ansi::namespace
//
//! Defines the [`Ansi`] namespace for emitting ANSI codes.
//

use crate::{_ansi_consts, Digits, slice};

#[doc = crate::_TAG_TERM!()]
/// ANSI escape codes.
///
/// # Return type
/// Constants ending with `_B` return a byte array. Those without it return a string slice.
///
/// Functions ending with `_B` return either an array or a byte slice.
/// Those without it return a [`StringU8`][crate::StringU8] or a string slice, respectively.
///
/// # Example
/// ```
/// use devela_base_core::Ansi;
/// assert_eq![Ansi::ITALIC_B, *b"\x1b[3m"];
/// assert_eq![Ansi::ITALIC, "\x1b[3m"];
/// ```
///
/// # Methods
/// ## Escape codes
/// - [Screen][Self#screen-escape-codes]
/// - [Erase][Self#erase-escape-codes]
/// - [Cursor][Self#cursor-escape-codes]
/// - [Font effects][Self#font-effects-escape-codes]
/// - [Color (3-bit)][Self#3-bit-color-escape-codes]
/// - [Color (8-bit)][Self#8-bit-color-escape-codes]
/// - [Grey (8-bit) Palette][Self#8-bit-grey-escape-codes]
/// - [Palette (8-bit)][Self#8-bit-palette-escape-codes]
/// - [Color (rgb)][Self#rgb-color-escape-codes]
///
/// ## Other
/// - [`strip_codes`](Self#strip_codes)
///
/// # Coordinate Order
/// All cursor positioning functions use `(columns, rows)` order, equivalent to `(x, y)`.
/// This follows graphics API conventions but is the **reverse** of the underlying
/// ANSI sequence which uses `[row;column]` order in escape sequences.
///
/// # Related Items
/// See also the [`ansi!`] macro.
#[doc = crate::doclink!(custom devela "[`ansi!`]" "ui/front/term/macro.ansi.html")]
#[derive(Debug)]
pub struct Ansi;

impl Ansi {
    _ansi_consts! {
        /// Control Sequence Introducer (CSI).
        ///
        /// <https://en.wikipedia.org/wiki/ANSI_escape_code#Control_Sequence_Introducer_commands>
        pub const CSI: [u8; 2] = "\x1b[", *b"\x1b[";
    }

    /* helper functions */

    // Writes an ansi code with a dynamic number of digits as an argument.
    #[must_use]
    const fn write_ansi_code_n(buffer: &mut [u8], n: u16, final_byte: u8) -> &[u8] {
        buffer[0] = b'\x1b';
        buffer[1] = b'[';
        let mut index = 2;
        let digit_count = Digits(n).count_digits10();
        let mut i = 0;
        while i < digit_count {
            buffer[index] = Digits(n).digit_at_index10(digit_count - 1 - i);
            index += 1;
            i += 1;
        }
        buffer[index] = final_byte;
        slice![buffer, ..=index]
    }
}

/// # Screen escape codes
impl Ansi {
    _ansi_consts! {
        /// Code to enable the alternative screen.
        pub const ENABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049h", *b"\x1b[?1049h";
        /// Code to disable the alternative screen.
        pub const DISABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049l", *b"\x1b[?1049l";
    }
}

/// # Erase escape codes
impl Ansi {
    _ansi_consts! {
        /// Code to erase from the cursor to the end of the line.
        pub const ERASE_LINE_END: [u8; 3] = "\x1b[K", *b"\x1b[K"; // also "\x1b[0K"
        /// Code to erase from the cursor to the start of the line.
        pub const ERASE_LINE_START: [u8; 4] = "\x1b[1K", *b"\x1b[1K";
        /// Code to erase the entire line.
        pub const ERASE_LINE: [u8; 4] = "\x1b[2K", *b"\x1b[2K";
        /// Code to erase from the cursor to the end of the screen.
        pub const ERASE_SCREEN_END: [u8; 3] = "\x1b[J", *b"\x1b[J"; // also "\x1b[0J"
        /// Code to erase from the cursor to the start of the screen.
        pub const ERASE_SCREEN_START: [u8; 4] = "\x1b[1J", *b"\x1b[1J";
        /// Code to erase the entire screen.
        pub const ERASE_SCREEN: [u8; 4] = "\x1b[2J", *b"\x1b[2J";
    }
}

/// # Cursor escape codes
impl Ansi {
    _ansi_consts! {
        /// Code to make the cursor invisible.
        pub const CURSOR_INVISIBLE: [u8; 6] = "\x1b[?25l", *b"\x1b[?25l";
        /// Code to make the cursor visible.
        pub const CURSOR_VISIBLE: [u8; 6] = "\x1b[?25h", *b"\x1b[?25h";

        /// Code to save the cursor position.
        pub const CURSOR_SAVE: [u8; 3] = "\x1b[s", *b"\x1b[s";
        /// Code to restore the cursor position.
        pub const CURSOR_RESTORE: [u8; 3] = "\x1b[u", *b"\x1b[u";

        /// Code to move the cursor to the home position (1, 1).
        pub const CURSOR_HOME: [u8; 3] = "\x1b[H", *b"\x1b[H";
    }
    _ansi_consts! {
        /// Code to move the cursor to the specified 1-digit position (col, row). (CUP)
        /// # Panics
        /// Panics in debug if either `row` or `col` > 9.
        pub const fn CURSOR_MOVE1(col: u8, row: u8) -> [u8; 6] {
            [ b'\x1b', b'[', Digits(row).digits10_1(), b';', Digits(col).digits10_1(), b'H' ]
        }
        /// Code to move the cursor to the specified 2-digit position (col, row).
        /// # Panics
        /// Panics in debug if either `row` or `col` > 99.
        pub const fn CURSOR_MOVE2(col: u8, row: u8) -> [u8; 8] {
            let r: [u8; 2] = Digits(row).digits10_2();
            let c: [u8; 2] = Digits(col).digits10_2();
            [b'\x1b', b'[', r[0], r[1], b';', c[0], c[1], b'H']
        }
        /// Code to move the cursor to the specified 3-digit position (col,row).
        /// # Panics
        /// Panics in debug if either `row` or `col` > 999.
        pub const fn CURSOR_MOVE3(col: u16, row: u16) -> [u8; 10] {
            let r: [u8; 3] = Digits(row).digits10_3();
            let c: [u8; 3] = Digits(col).digits10_3();
            [b'\x1b', b'[', r[0], r[1], r[2], b';', c[0], c[1], c[2], b'H']
        }
        /// Code to move the cursor to the specified 4-digit position (col, row).
        /// # Panics
        /// Panics in debug if either `row` or `col` > 9999.
        pub const fn CURSOR_MOVE4(col: u16, row: u16) -> [u8; 12] {
            let r: [u8; 4] = Digits(row).digits10_4();
            let c: [u8; 4] = Digits(col).digits10_4();
            [b'\x1b', b'[', r[0], r[1], r[2], r[3], b';', c[0], c[1], c[2], c[3], b'H']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor to the specified position (col, row).
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_MOVE_N(buffer: &mut [u8], col: u16, row: u16) -> &[u8] {
            buffer[0] = b'\x1b';
            buffer[1] = b'[';
            let mut index = 2;
            let mut i = 0;
            let row_digits = Digits(row).count_digits10();
            while i < row_digits {
                buffer[index] = Digits(row).digit_at_index10(row_digits - 1 - i);
                index += 1;
                i += 1;
            }
            buffer[index] = b';';
            index += 1;
            let col_digits = Digits(col).count_digits10();
            let mut i = 0;
            while i < col_digits {
                buffer[index] = Digits(col).digit_at_index10(col_digits - 1 - i);
                index += 1;
                i += 1;
            }
            buffer[index] = b'H';
            slice![buffer, ..=index]
        }
    }
    _ansi_consts! {
        /// Code to move the cursor up by one line. (CUU)
        pub const CURSOR_UP: [u8; 3] = "\x1b[A", *b"\x1b[A";
    }
    _ansi_consts! {
        /// Code to move the cursor up by 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_UP1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'A']
        }
        /// Code to move the cursor up by 2-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_UP2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'A']
        }
        /// Code to move the cursor up by 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_UP3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'A']
        }
        /// Code to move the cursor up by 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9999.
        pub const fn CURSOR_UP4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'A']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor up by `n` lines.
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_UP_N(buffer: &mut [u8], n: u16) -> &[u8] {
            Self::write_ansi_code_n(buffer, n, b'A')
        }
    }
    _ansi_consts! {
        /// Code to move the cursor down by one line. (CUD)
        pub const CURSOR_DOWN: [u8; 3] = "\x1b[B", *b"\x1b[B";
    }
    _ansi_consts! {
        /// Code to move the cursor down by 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_DOWN1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'B']
        }
        /// Code to move the cursor down by 2-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_DOWN2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'B']
        }
        /// Code to move the cursor down by 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_DOWN3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'B']
        }
        /// Code to move the cursor down by 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_DOWN4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'B']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor down by `n` lines.
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_DOWN_N(buffer: &mut [u8], n: u16) -> &[u8] {
            Self::write_ansi_code_n(buffer, n, b'B')
        }
    }
    _ansi_consts! {
        /// Code to move the cursor right by one column. (CUF)
        pub const CURSOR_RIGHT: [u8; 3] = "\x1b[C", *b"\x1b[C";
    }
    _ansi_consts! {
        /// Code to move the cursor right by 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_RIGHT1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'C']
        }
        /// Code to move the cursor right by 2-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_RIGHT2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'C']
        }
        /// Code to move the cursor right by 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_RIGHT3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'C']
        }
        /// Code to move the cursor right by 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9999.
        pub const fn CURSOR_RIGHT4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'C']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor right by `n` lines.
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_RIGHT_N(buffer: &mut [u8], n: u16) -> &[u8] {
            Self::write_ansi_code_n(buffer, n, b'C')
        }
    }
    _ansi_consts! {
        /// Code to move the cursor left by one column. (CUB)
        pub const CURSOR_LEFT: [u8; 3] = "\x1b[D", *b"\x1b[D";
    }
    _ansi_consts! {
        /// Code to move the cursor left by 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_LEFT1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'D']
        }
        /// Code to move the cursor left by 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_LEFT2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'D']
        }
        /// Code to move the cursor left by 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_LEFT3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'D']
        }
        /// Code to move the cursor left by 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9999.
        pub const fn CURSOR_LEFT4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'D']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor left by `n` lines.
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_LEFT_N(buffer: &mut [u8], n: u16) -> &[u8] {
            Self::write_ansi_code_n(buffer, n, b'D')
        }
    }
    _ansi_consts! {
        /// Code to move the cursor to the beginning of the next line. (CNL)
        pub const CURSOR_NEXT_LINE: [u8; 3] = "\x1b[E", *b"\x1b[E";
    }
    _ansi_consts! {
        /// Code to move the cursor to the beginning of the next 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_NEXT_LINE1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'E']
        }
        /// Code to move the cursor to the beginning of the next 2-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_NEXT_LINE2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'E']
        }
        /// Code to move the cursor to the beginning of the next 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_NEXT_LINE3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'E']
        }
        /// Code to move the cursor to the beginning of the next 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_NEXT_LINE4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'E']
        }
    }
    _ansi_consts! {
        /// Returns a slice with the code to move the cursor to the beginning of the next `n` lines.
        ///
        /// It needs a `buffer` where to store the bytes.
        ///
        /// # Panics
        /// Panics if the buffer is not big enough.
        pub const fn CURSOR_NEXT_LINE_N(buffer: &mut [u8], n: u16) -> &[u8] {
            Self::write_ansi_code_n(buffer, n, b'F')
        }
    }

    _ansi_consts! {
        /// Code to move the cursor to the beginning of the previous line. (CPL)
        pub const CURSOR_PREV_LINE: [u8; 3] = "\x1b[E", *b"\x1b[E";
    }
    _ansi_consts! {
        /// Code to move the cursor to the beginning of the previous 1-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 9.
        pub const fn CURSOR_PREV_LINE1(n: u8) -> [u8; 4] {
            [b'\x1b', b'[', Digits(n).digits10_1(), b'E']
        }
        /// Code to move the cursor to the beginning of the previous 2-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 99.
        pub const fn CURSOR_PREV_LINE2(n: u8) -> [u8; 5] {
            let n: [u8; 2] = Digits(n).digits10_2();
            [b'\x1b', b'[', n[0], n[1], b'E']
        }
        /// Code to move the cursor to the beginning of the previous 3-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_PREV_LINE3(n: u16) -> [u8; 6] {
            let n: [u8; 3] = Digits(n).digits10_3();
            [b'\x1b', b'[', n[0], n[1], n[2], b'E']
        }
        /// Code to move the cursor to the beginning of the previous 4-digit `n` lines.
        /// # Panics
        /// Panics in debug if `n` > 999.
        pub const fn CURSOR_PREV_LINE4(n: u16) -> [u8; 7] {
            let n: [u8; 4] = Digits(n).digits10_4();
            [b'\x1b', b'[', n[0], n[1], n[2], n[3], b'E']
        }
    }
    _ansi_consts! {
    /// Returns a slice with the code to move the cursor to the beginning of the previous `n` lines.
    ///
    /// It needs a `buffer` where to store the bytes.
    ///
    /// # Panics
    /// Panics if the buffer is not big enough.
    pub const fn CURSOR_PREV_LINE_N(buffer: &mut [u8], n: u16) -> &[u8] {
        Self::write_ansi_code_n(buffer, n, b'E')
    }}
}

/// # Font effects escape codes
impl Ansi {
    _ansi_consts! {
        /// Code to turn off all effects and colors.
        pub const RESET: [u8; 4] = "\x1b[0m", *b"\x1b[0m";

        /// Code to set bold effect.
        pub const BOLD: [u8; 4] = "\x1b[1m", *b"\x1b[1m";
        /// Code to unset bold and dim effects.
        pub const BOLD_OFF: [u8; 5] = "\x1b[22m", *b"\x1b[22m";

        /// Code to set italic effect.
        pub const ITALIC: [u8; 4] = "\x1b[3m", *b"\x1b[3m";
        /// Code to unset italic and fraktur effects.
        pub const ITALIC_OFF: [u8; 5] = "\x1b[23m", *b"\x1b[23m";

        /// Code to set dim effect.
        pub const DIM: [u8; 4] = "\x1b[2m", *b"\x1b[2m";
        /// Code to unset bold and dim effects.
        pub const DIM_OFF: [u8; 5] = "\x1b[22m", *b"\x1b[22m";

        /// Code to set underline effect.
        pub const UNDERLINE: [u8; 4] = "\x1b[4m", *b"\x1b[4m";
        /// Code to unset underline effect.
        pub const UNDERLINE_OFF: [u8; 5] = "\x1b[24m", *b"\x1b[24m";

        /// Code to set blink effect.
        pub const BLINK: [u8; 4] = "\x1b[5m", *b"\x1b[5m";
        /// Code to unset blink effect.
        pub const BLINK_OFF: [u8; 5] = "\x1b[25m", *b"\x1b[25m";

        /// Code to set inverse effect.
        pub const INVERSE: [u8; 4] = "\x1b[7m", *b"\x1b[7m";
        /// Code to unset inverse effect.
        pub const INVERSE_OFF: [u8; 5] = "\x1b[27m", *b"\x1b[27m";

        /// Code to set crossed effect.
        pub const CROSSED: [u8; 4] = "\x1b[9m", *b"\x1b[9m";
        /// Code to unset crossed effect.
        pub const CROSSED_OFF: [u8; 5] = "\x1b[29m", *b"\x1b[29m";
    }
}

// /// # Not very well supported font effects escape codes
// impl Ansi {
//     _ansi_consts! {
//         pub const UNDERLINE_DOUBLE: [u8; 5] = "\x1b[21m", *b"\x1b[21m"; // or bold_off
//         pub const BLINK_FAST: &[u8] = "\x1b[6m", *b"\x1b[6m";
//     }
// }

#[cfg(test)]
mod tests {
    use super::Ansi;

    #[test]
    fn write_ansi_code_n() {
        let mut buffer = [0u8; 16];
        let result = Ansi::write_ansi_code_n(&mut buffer, 0, b'J');
        assert_eq!(result, b"\x1b[0J");
        let result = Ansi::write_ansi_code_n(&mut buffer, 5, b'm');
        assert_eq!(result, b"\x1b[5m");
        let result = Ansi::write_ansi_code_n(&mut buffer, 255, b'B');
        assert_eq!(result, b"\x1b[255B");
        let result = Ansi::write_ansi_code_n(&mut buffer, 15000, b'C');
        assert_eq!(result, b"\x1b[15000C");
    }

    #[test]
    fn cursor_move_n() {
        let mut buffer = [0u8; 32];
        let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 0, 0);
        assert_eq!(result, b"\x1b[0;0H");
        let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 1, 2);
        assert_eq!(result, b"\x1b[2;1H");
        let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 5, 10);
        assert_eq!(result, b"\x1b[10;5H");
        let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 123, 456);
        assert_eq!(result, b"\x1b[456;123H");
        let result = Ansi::CURSOR_MOVE_N_B(&mut buffer, 1999, 10999);
        assert_eq!(result, b"\x1b[10999;1999H");
    }
}
