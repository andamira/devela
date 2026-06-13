// devela/src/sys/os/term/ansi/namespace/font.rs

use crate::{__ansi_consts, Ansi, AnsiColor8, Digits};

/// # Font effects escape codes
//
// https://en.wikipedia.org/wiki/ANSI_escape_code#Select_Graphic_Rendition_parameters
impl Ansi {
    __ansi_consts! {
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
        /// Code to unset blink effects.
        pub const BLINK_OFF: [u8; 5] = "\x1b[25m", *b"\x1b[25m";

        /// Code to set inverse effect.
        pub const INVERSE: [u8; 4] = "\x1b[7m", *b"\x1b[7m";
        /// Code to unset inverse effect.
        pub const INVERSE_OFF: [u8; 5] = "\x1b[27m", *b"\x1b[27m";

        /// Code to set hidden effect.
        pub const HIDDEN: [u8; 4] = "\x1b[8m", *b"\x1b[8m";
        /// Code to unset hidden effect.
        pub const HIDDEN_OFF: [u8; 5] = "\x1b[28m", *b"\x1b[28m";

        /// Code to set crossed effect.
        pub const CROSSED: [u8; 4] = "\x1b[9m", *b"\x1b[9m";
        /// Code to unset crossed effect.
        pub const CROSSED_OFF: [u8; 5] = "\x1b[29m", *b"\x1b[29m";
    }
}

/// # Not very well supported font effects escape codes
impl Ansi {
    __ansi_consts! {
        /// Code to set rapid blink effect.
        ///
        /// Terminal support varies.
        pub const BLINK_FAST: [u8; 4] = "\x1b[6m", *b"\x1b[6m";

        /// Set fraktur (gothic/blackletter) font (Rarely supported).
        pub const FRAKTUR: [u8; 5] = "\x1b[20m", *b"\x1b[20m";

        /// Code to frame subsequent characters.
        ///
        /// Rarely supported.
        pub const FRAME: [u8; 5] = "\x1b[51m", *b"\x1b[51m";

        /// Code to encircle subsequent characters.
        ///
        /// Rarely supported.
        pub const ENCIRCLE: [u8; 5] = "\x1b[52m", *b"\x1b[52m";
        /// Code to unset framed and encircled effects.
        pub const FRAME_ENCIRCLE_OFF: [u8; 5] = "\x1b[54m", *b"\x1b[54m";

        /// Code to set overline effect.
        pub const OVERLINE: [u8; 5] = "\x1b[53m", *b"\x1b[53m";
        /// Code to unset overline effect.
        pub const OVERLINE_OFF: [u8; 5] = "\x1b[55m", *b"\x1b[55m";

        /// Code to render subsequent text as superscript.
        ///
        /// This is a terminal extension with limited support.
        pub const SUPERSCRIPT: [u8; 5] = "\x1b[73m", *b"\x1b[73m";

        /// Code to render subsequent text as subscript.
        ///
        /// This is a terminal extension with limited support.
        pub const SUBSCRIPT: [u8; 5] = "\x1b[74m", *b"\x1b[74m";

        /// Code to restore the normal text baseline.
        pub const BASELINE: [u8; 5] = "\x1b[75m", *b"\x1b[75m";
    }
}
/// # Extended underline escape codes
impl Ansi {
    __ansi_consts! {
        /// Code to set double underline effect.
        ///
        /// Uses the modern, unambiguous underline-style syntax.
        pub const UNDERLINE_DOUBLE: [u8; 6] = "\x1b[4:2m", *b"\x1b[4:2m";

        /// Code to set curly underline effect.
        pub const UNDERLINE_CURLY: [u8; 6] = "\x1b[4:3m", *b"\x1b[4:3m";

        /// Code to set dotted underline effect.
        pub const UNDERLINE_DOTTED: [u8; 6] = "\x1b[4:4m", *b"\x1b[4:4m";

        /// Code to set dashed underline effect.
        pub const UNDERLINE_DASHED: [u8; 6] = "\x1b[4:5m", *b"\x1b[4:5m";

        /// Code to set double underline using the ECMA-48 SGR 21 form.
        ///
        /// Some terminals instead interpret this as normal intensity.
        pub const UNDERLINE_DOUBLE_ECMA: [u8; 5] = "\x1b[21m", *b"\x1b[21m";
    }

    /// Selects the primary or an alternative terminal font.
    ///
    /// `0` selects the primary font; `1..=9` select alternative fonts.
    ///
    /// Most modern terminal emulators ignore these selectors.
    pub const fn FONT(index: u8) -> [u8; 5] {
        assert![index <= 9];
        [b'\x1b', b'[', b'1', b'0' + index, b'm']
    }
}

/// # Underline color escape codes
#[rustfmt::skip]
impl Ansi {
    __ansi_consts! {
        /// Code to restore the underline color to the terminal default.
        ///
        /// The default underline color follows the foreground color.
        pub const DEFAULT_UNDERLINE_COLOR: [u8; 5] = "\x1b[59m", *b"\x1b[59m";
    }
    /// Code to set the underline color to an 8-bit palette `color`.
    pub const fn UNDERLINE_COLOR8(color: AnsiColor8) -> [u8; 11] {
        let c = color.to_ascii();
        [
            b'\x1b', b'[', b'5', b'8', b';', b'5', b';',
            c[0], c[1], c[2],
            b'm',
        ]
    }
    /// Code to set the underline color to `[r, g, b]` values.
    pub const fn UNDERLINE_RGB(color: [u8; 3]) -> [u8; 19] {
        let [r, g, b] = color;
        let [r, g, b] = [
            Digits(r).digits10(),
            Digits(g).digits10(),
            Digits(b).digits10(),
        ];
        [
            b'\x1b', b'[', b'5', b'8', b';', b'2', b';',
            r[0], r[1], r[2], b';',
            g[0], g[1], g[2], b';',
            b[0], b[1], b[2],
            b'm',
        ]
    }
}

// NOTE: missing rare/historical escape codes:
// - 26     proportional spacing
// - 50     disable proportional spacing
// - 60     ideogram underline / right-side line
// - 61     double ideogram underline / right-side line
// - 62     ideogram overline / left-side line
// - 63     double ideogram overline / left-side line
// - 64     ideogram stress marking
// - 65     cancel ideogram attributes
