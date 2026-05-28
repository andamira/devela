// devela::sys::os::term::ansi::namespace::terminal

use crate::{__ansi_consts, Ansi};

/// # Terminal modes
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

        /// Code to enable synchronized update mode.
        pub const ENABLE_SYNC_UPDATE: [u8; 8] = "\x1b[?2026h", *b"\x1b[?2026h";
        /// Code to disable synchronized update mode.
        pub const DISABLE_SYNC_UPDATE: [u8; 8] = "\x1b[?2026l", *b"\x1b[?2026l";
    }
}
