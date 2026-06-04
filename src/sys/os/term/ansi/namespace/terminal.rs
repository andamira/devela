// devela::sys::os::term::ansi::namespace::terminal

use crate::{__ansi_consts, Ansi, Digits, slice, write_at};

/// # Terminal escape codes
impl Ansi {
    __ansi_consts! {
        /// Code to enable the alternate screen buffer, with cursor save/restore.
        pub const ENABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049h", *b"\x1b[?1049h";
        /// Code to disable the alternate screen buffer, with cursor save/restore.
        pub const DISABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049l", *b"\x1b[?1049l";

        /// Code to enable DEC alternate screen mode 1047, without cursor save/restore.
        pub const ENABLE_ALT_SCREEN_ONLY: [u8; 8] = "\x1b[?1047h", *b"\x1b[?1047h";
        /// Code to disable DEC alternate screen mode 1047, without cursor save/restore.
        pub const DISABLE_ALT_SCREEN_ONLY: [u8; 8] = "\x1b[?1047l", *b"\x1b[?1047l";

        /// Code to enable bracketed paste mode.
        pub const ENABLE_BRACKETED_PASTE: [u8; 8] = "\x1b[?2004h", *b"\x1b[?2004h";
        /// Code to disable bracketed paste mode.
        pub const DISABLE_BRACKETED_PASTE: [u8; 8] = "\x1b[?2004l", *b"\x1b[?2004l";

        /// Code to enable focus in/out events.
        pub const ENABLE_FOCUS_EVENTS: [u8; 8] = "\x1b[?1004h", *b"\x1b[?1004h";
        /// Code to disable focus in/out events.
        pub const DISABLE_FOCUS_EVENTS: [u8; 8] = "\x1b[?1004l", *b"\x1b[?1004l";

        /// Code to enable synchronized update mode.
        pub const ENABLE_SYNC_UPDATE: [u8; 8] = "\x1b[?2026h", *b"\x1b[?2026h";
        /// Code to disable synchronized update mode.
        pub const DISABLE_SYNC_UPDATE: [u8; 8] = "\x1b[?2026l", *b"\x1b[?2026l";
    }
}

/// # Terminal mode report escape codes
impl Ansi {
    __ansi_consts! {
        /// Queries the state/support of a DEC private mode. (DECRQM)
        ///
        /// Emits `CSI ? mode $ p`.
        ///
        /// The terminal may reply with `CSI ? mode ; state $ y`, where:
        /// - `0`: not recognized
        /// - `1`: set
        /// - `2`: reset
        /// - `3`: permanently set
        /// - `4`: permanently reset
        ///
        /// The `buffer` must have room for the longest `u16` sequence:
        /// `ESC [` + `?` + 5 digits + `$p`, i.e. 10 bytes.
        ///
        /// # Panics
        /// Panics if `buffer.len() < 10`.
        pub const fn QUERY_DEC_PRIVATE_MODE_N(buffer: &mut [u8], mode: u16) -> &[u8] {
            assert![buffer.len() >= 10];
            let mut index = 0;
            write_at![buffer, +=index, @3 b"\x1b[?"];
            index += Digits(mode).write_digits10_fast(buffer, index);
            write_at![buffer, +=index, @2 b"$p"];
            slice![buffer, ..index]
        }
    }
    __ansi_consts! {
        /// Query cursor visibility mode support/state. (DECTCEM)
        pub const QUERY_CURSOR_VISIBLE: [u8; 7] = "\x1b[?25$p", *b"\x1b[?25$p";

        /// Query focus in/out event mode support/state.
        pub const QUERY_FOCUS_EVENTS: [u8; 9] = "\x1b[?1004$p", *b"\x1b[?1004$p";

        /// Query mouse press/release reporting.
        pub const QUERY_MOUSE: [u8; 9] = "\x1b[?1000$p", *b"\x1b[?1000$p";
        /// Query mouse drag reporting.
        pub const QUERY_MOUSE_DRAG: [u8; 9] = "\x1b[?1002$p", *b"\x1b[?1002$p";
        /// Query mouse motion reporting.
        pub const QUERY_MOUSE_MOTION: [u8; 9] = "\x1b[?1003$p", *b"\x1b[?1003$p";
        /// Query SGR mouse mode support/state.
        pub const QUERY_MOUSE_SGR: [u8; 9] = "\x1b[?1006$p", *b"\x1b[?1006$p";
        /// Query SGR mouse pixel-coordinate mode support/state.
        pub const QUERY_MOUSE_SGR_PIXELS: [u8; 9] = "\x1b[?1016$p", *b"\x1b[?1016$p";

        /// Query alternate scroll mode, affecting wheel behavior in alt screen.
        pub const QUERY_ALT_SCROLL_MODE: [u8; 9] = "\x1b[?1007$p", *b"\x1b[?1007$p";
        /// Query alternate screen buffer mode 1047 support/state.
        pub const QUERY_ALT_SCREEN_ONLY: [u8; 9] = "\x1b[?1047$p", *b"\x1b[?1047$p";
        /// Query alternate screen buffer with cursor save/restore support/state.
        pub const QUERY_ALT_SCREEN: [u8; 9] = "\x1b[?1049$p", *b"\x1b[?1049$p";

        /// Query bracketed paste mode support/state.
        pub const QUERY_BRACKETED_PASTE: [u8; 9] = "\x1b[?2004$p", *b"\x1b[?2004$p";

        /// Query synchronized output/update mode support/state.
        pub const QUERY_SYNC_UPDATE: [u8; 9] = "\x1b[?2026$p", *b"\x1b[?2026$p";

        /// Query sixel display mode.
        pub const QUERY_SIXEL_DISPLAY: [u8; 7] = "\x1b[?80$p", *b"\x1b[?80$p";
    }
}
