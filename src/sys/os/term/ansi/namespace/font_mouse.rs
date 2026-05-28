// devela::sys::os::term::ansi::namespace::font_mouse

use crate::{__ansi_consts, Ansi};

/// # Mouse escape codes
impl Ansi {
    __ansi_consts! {
        /// Code to enable normal mouse button reporting.
        ///
        /// Reports button press and release events.
        /// Prefer combining this with `ENABLE_MOUSE_SGR` for modern parsing.
        pub const ENABLE_MOUSE: [u8; 8] = "\x1b[?1000h", *b"\x1b[?1000h";
        /// Code to disable normal mouse button reporting.
        pub const DISABLE_MOUSE: [u8; 8] = "\x1b[?1000l", *b"\x1b[?1000l";

        /// Code to enable mouse drag reporting.
        ///
        /// Reports movement while a button is held.
        pub const ENABLE_MOUSE_DRAG: [u8; 8] = "\x1b[?1002h", *b"\x1b[?1002h";
        /// Code to disable mouse drag reporting.
        pub const DISABLE_MOUSE_DRAG: [u8; 8] = "\x1b[?1002l", *b"\x1b[?1002l";

        /// Code to enable mouse motion reporting.
        ///
        /// Reports mouse movement even when no button is held. This can be noisy.
        pub const ENABLE_MOUSE_MOTION: [u8; 8] = "\x1b[?1003h", *b"\x1b[?1003h";
        /// Code to disable mouse motion reporting.
        pub const DISABLE_MOUSE_MOTION: [u8; 8] = "\x1b[?1003l", *b"\x1b[?1003l";

        /// Code to enable SGR extended mouse encoding.
        ///
        /// Prefer this with `ENABLE_MOUSE`, `ENABLE_MOUSE_DRAG`, or
        /// `ENABLE_MOUSE_MOTION` for modern cell-coordinate mouse parsing.
        pub const ENABLE_MOUSE_SGR: [u8; 8] = "\x1b[?1006h", *b"\x1b[?1006h";
        /// Code to disable SGR extended mouse encoding.
        pub const DISABLE_MOUSE_SGR: [u8; 8] = "\x1b[?1006l", *b"\x1b[?1006l";

        /// Code to enable SGR extended mouse encoding with pixel coordinates.
        pub const ENABLE_MOUSE_SGR_PIXELS: [u8; 8] = "\x1b[?1016h", *b"\x1b[?1016h";
        /// Code to disable SGR extended mouse encoding with pixel coordinates.
        pub const DISABLE_MOUSE_SGR_PIXELS: [u8; 8] = "\x1b[?1016l", *b"\x1b[?1016l";

        /* legacy */

        /// Code to enable legacy X10 mouse reporting.
        ///
        /// Response: `CSI M Cb Cx Cy` on button press only.
        pub const ENABLE_MOUSE_X10: [u8; 5] = "\x1b[?9h", *b"\x1b[?9h";
        /// Code to disable legacy X10 mouse reporting.
        pub const DISABLE_MOUSE_X10: [u8; 5] = "\x1b[?9l", *b"\x1b[?9l";
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
