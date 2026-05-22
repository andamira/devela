// devela::sys::os::term::ansi::namespace::font_mouse

use crate::{__ansi_consts, Ansi};

/// # Mouse escape codes
///
/// Example SGR mouse event received:
/// `ESC [ < 0 ; 45 ; 10 M`, where:
/// - `0`: button state
/// - `45`: column
/// - `10`: row
/// -  `M`: press (`m` indicates release)
impl Ansi {
    __ansi_consts! {
        /// Code to enable X10 compatibility mode for mouse click reporting.
        ///
        /// Response: `CSI M Cb Cx Cy` (fixed-length, byte-encoded).
        /// - Cb is button (1, 2 or 3) plus 32.
        /// - Cx and Cy are column and row plus 32.
        pub const MOUSE_X10_ENABLE: [u8; 5] = "\x1b[?9h", *b"\x1b[?9h";
        /// Code to disable X10 compatibility mode.
        pub const MOUSE_X10_DISABLE: [u8; 5] = "\x1b[?9l", *b"\x1b[?9l";

        // /// Code to enable basic mouse click reporting.
        // pub const MOUSE_NORMAL: [u8; 8] = "\x1b[?1000h", *b"\x1b[?1000h";
        // /// Code to enable all motion tracking.
        // pub const MOUSE_TRACKING: [u8; 8] = "\x1b[?1003h", *b"\x1b[?1003h";
        /// Code to enable UTF-8 extended mouse coordinates (legacy) with report in cells.
        pub const MOUSE_UTF8: [u8; 8] = "\x1b[?1005h", *b"\x1b[?1005h";
        /// Code to enable SGR extended mouse coordinates with report in cells.
        pub const MOUSE_SGR: [u8; 8] = "\x1b[?1006h", *b"\x1b[?1006h";
        /// Code to enable SGR extended coordinates with report in pixels.
        pub const MOUSE_SGR_PIXELS: [u8; 8] = "\x1b[?1016h", *b"\x1b[?1016h"; // = SGR + pixel mode
    }
}

/// # Font effects escape codes
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
//     __ansi_consts! {
//         pub const UNDERLINE_DOUBLE: [u8; 5] = "\x1b[21m", *b"\x1b[21m"; // or bold_off
//         pub const BLINK_FAST: &[u8] = "\x1b[6m", *b"\x1b[6m";
//     }
// }
