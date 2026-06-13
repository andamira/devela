// devela/src/sys/os/term/ansi/namespace/mouse.rs

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
