// devela::os::terminal::ansi
//
//! ANSI codes.
//

use super::AnsiColor as Color;
use crate::ascii::{ascii_d1, ascii_d2, ascii_d3, ascii_d4};
#[cfg(all(
    any(not(feature = "std"), doc),
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    ),
    feature = "unsafe_os",
    not(miri),
))]
use crate::os::linux::linux_print_bytes;
#[cfg(feature = "std")]
use std::io::{stdout, Write};

/// ANSI escape codes with print support.
pub struct Ansi;

#[cfg(any(
    feature = "std",
    all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    )
))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(
        feature = "std",
        all(
            target_os = "linux",
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            ),
            feature = "unsafe_os",
        )
    )))
)]
impl Ansi {
    /// Prints an ANSI escape `sequence` of bytes to `stdout`.
    ///
    /// It uses the [`Write`] trait on `std`, and [`linux_print_bytes`] on `no_std`.
    #[inline]
    pub fn print(sequence: &[u8]) {
        // MAYBE -> Result<()>
        #[cfg(feature = "std")]
        let _ = stdout().write_all(sequence);
        #[cfg(not(feature = "std"))]
        linux_print_bytes(sequence);
    }
}

// Generates methods for printing the escape codes.
//
// # Common arguments:
// $fn    : function name
// $ansi  : function name (smallcase) and ansi constant (uppercase)
// $doc   : doc string
#[allow(unused_macros)]
macro_rules! __print {
    // referring to an associated const
    ($ansi:ident, $doc:literal) => { crate::codegen::paste! {
        #[doc = $doc]
        #[inline]
        pub fn [<$ansi:lower>]() {
            let _ = Self::print(Self::[<$ansi:upper>]);
        }
    }};

    // referring to an associated (const) fn
    //
    // $arg: $t   : optional pairs of [argument: type],
    ($ansi:ident, $($arg:ident: $t:ty),* ; $doc:literal) => { crate::codegen::paste! {
        #[doc = $doc]
        #[inline]
        pub fn [<$ansi:lower>]($($arg: $t,)*) {
            let _ = Self::print(&Self::[<$ansi:upper>]($($arg, )*));
        }
    }};
}

/// # Methods to print the ANSI escape codes to `stdout`.
///
/// Uses the [`print`][Self::print] method, so they are only available on either:
/// - crate feature `std`, or
/// - Linux and (`x86`-64 or `x86` or `ARM` or `AArch64` or `RISC-V RV32`
/// or `RISC-V RV64`) and crate feature `unsafe_os`.
#[cfg(any(
    feature = "std",
    all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    )
))]
#[rustfmt::skip]
impl Ansi {
    /* screen */

    __print![clear_screen, "Clears the screen."];
    __print![enable_alternative_screen, "Enables the alternative screen."];
    __print![disable_alternative_screen, "Disables the alternative screen."];

    /* cursor */

    __print![cursor_invisible, "Makes the cursor invisible."];
    __print![cursor_visible, "Makes the cursor visible."];
    __print![cursor_save, "Saves the cursor position."];
    __print![cursor_restore, "Restores the cursor position."];
    __print![cursor_home, "Moves the cursor to the home position (1, 1)."];

    __print![cursor_move1, row: u8, col: u8;
    "Moves the cursor to the specified 1-digit position.
    \n# Panics\n\nPanics in debug if either `row` or `col` > 9."];
    __print![cursor_move2, row: u8, col: u8;
    "Moves the cursor to the specified 2-digit position.
    \n# Panics\n\nPanics in debug if either `row` or `col` > 99."];
    __print![cursor_move3, row: u16, col: u16;
    "Moves the cursor to the specified 3-digit position.
    \n# Panics\n\nPanics in debug if either `row` or `col` > 999."];
    __print![cursor_move4, row: u16, col: u16;
    "Moves the cursor to the specified 4-digit position.
    \n# Panics\n\nPanics in debug if either `row` or `col` > 9999."];

    __print![cursor_up, "Moves the cursor up by one line."];
    __print![cursor_up1, n: u8; "Moves the cursor up by 1-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9."];
    __print![cursor_up2, n: u8; "Moves the cursor up by 2-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 99."];
    __print![cursor_up3, n: u16; "Moves the cursor up by 3-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 999."];
    __print![cursor_up4, n: u16; "Moves the cursor up by 4-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9999."];

    __print![cursor_down, "Moves the cursor down by one line."];
    __print![cursor_down1, n: u8; "Moves the cursor down by 1-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9."];
    __print![cursor_down2, n: u8; "Moves the cursor down by 2-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 99."];
    __print![cursor_down3, n: u16; "Moves the cursor down by 3-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 999."];
    __print![cursor_down4, n: u16; "Moves the cursor down by 4-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9999."];

    __print![cursor_right, "Moves the cursor right by one line."];
    __print![cursor_right1, n: u8; "Moves the cursor right by 1-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9."];
    __print![cursor_right2, n: u8; "Moves the cursor right by 2-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 99."];
    __print![cursor_right3, n: u16; "Moves the cursor right by 3-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 999."];
    __print![cursor_right4, n: u16; "Moves the cursor right by 4-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9999."];

    __print![cursor_left, "Moves the cursor left by one line."];
    __print![cursor_left1, n: u8; "Moves the cursor left by 1-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9."];
    __print![cursor_left2, n: u8; "Moves the cursor left by 2-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 99."];
    __print![cursor_left3, n: u16; "Moves the cursor left by 3-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 999."];
    __print![cursor_left4, n: u16; "Moves the cursor left by 4-digit `n` lines.
    \n# Panics\n\nPanics in debug if `n` > 9999."];

    /* font effects */

    __print![reset, "Turns off all effects and colors."];
    __print![bold, "Turns bold on."];
    __print![bold_off, "Turns bold and dim off."];
    __print![dim, "Turns dim on."];
    __print![dim_off, "Turns bold and dim off."];
    __print![italic, "Turns italic on."];
    __print![italic_off, "Turns italic off."];
    __print![underline, "Turns underline on."];
    __print![underline_off, "Turns underline off."];
    __print![blink, "Turns blink on."];
    __print![blink_off, "Turns blink off."];
    __print![inverse, "Turns inverse on."];
    __print![inverse_off, "Turns inverse off."];
    __print![crossed, "Turns crossed on."];
    __print![crossed_off, "Turns crossed off."];

    /* 4-bit colors */

    __print![black, "Sets the foreground color to black."];
    __print![red, "Sets the foreground color to red."];
    __print![green, "Sets the foreground color to green."];
    __print![yellow, "Sets the foreground color to yellow."];
    __print![blue, "Sets the foreground color to blue."];
    __print![magenta, "Sets the foreground color to magenta."];
    __print![cyan, "Sets the foreground color to cyan."];
    __print![white, "Sets the foreground color to white."];

    __print![black_bg, "Sets the background color to black."];
    __print![red_bg, "Sets the background color to red."];
    __print![green_bg, "Sets the background color to green."];
    __print![yellow_bg, "Sets the background color to yellow."];
    __print![blue_bg, "Sets the background color to blue."];
    __print![magenta_bg, "Sets the background color to magenta."];
    __print![cyan_bg, "Sets the background color to cyan."];
    __print![white_bg, "Sets the background color to white."];

    __print![bright_black, "Sets the foreground color to bright black."];
    __print![bright_red, "Sets the foreground color to bright red."];
    __print![bright_green, "Sets the foreground color to bright green."];
    __print![bright_yellow, "Sets the foreground color to bright yellow."];
    __print![bright_blue, "Sets the foreground color to bright blue."];
    __print![bright_magenta, "Sets the foreground color to bright magenta."];
    __print![bright_cyan, "Sets the foreground color to bright cyan."];
    __print![bright_white, "Sets the foreground color to bright white."];

    __print![bright_black_bg, "Sets the background color to bright black."];
    __print![bright_red_bg, "Sets the background color to bright red."];
    __print![bright_green_bg, "Sets the background color to bright green."];
    __print![bright_yellow_bg, "Sets the background color to bright yellow."];
    __print![bright_blue_bg, "Sets the background color to bright blue."];
    __print![bright_magenta_bg, "Sets the background color to bright magenta."];
    __print![bright_cyan_bg, "Sets the background color to bright cyan."];
    __print![bright_white_bg, "Sets the background color to bright white."];
}

/// # Ansi escape codes
#[allow(non_snake_case)]
impl Ansi {
    /* screen */

    /// Code to clear the screen.
    pub const CLEAR_SCREEN: &'static [u8] = b"\x1b[2J";

    /// Code to enable the alternative screen.
    pub const ENABLE_ALTERNATIVE_SCREEN: &'static [u8] = b"\x1b[1049h";

    /// Code to disable the alternative screen.
    pub const DISABLE_ALTERNATIVE_SCREEN: &'static [u8] = b"\x1b[1049l";

    /* cursor */

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

    /* font effects */

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
