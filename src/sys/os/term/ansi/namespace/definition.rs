// devela::sys::os::term::ansi::namespace::definition
//
//! Defines the [`Ansi`] namespace for emitting ANSI codes.
//

use crate::__ansi_consts;

#[doc = crate::_tags!(term namespace)]
/// ANSI escape codes.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
///
/// # Return types
/// Constants ending with `_B` contain a byte array.
/// Those without `_B` contain a string slice.
///
/// Functions ending with `_B` return either an array or a byte slice.
/// Functions without `_B` return either a [`StringNonul`] or [`&str`].
///
/// [`StringNonul`]: crate::StringNonul
///
/// # Constants and Methods
/// ## Escape codes
/// - [Control prefixes](#control-prefixes)
/// - CSI sequences
///   - [Erase](#erase-escape-codes)
///   - [Terminal](#terminal-escape-codes)
///     - [Mode reports](#terminal-mode-report-escape-codes)
///   - [Cursor](#cursor-escape-codes)
///   - [Mouse](#mouse-escape-codes)
///   - [Font effects](#font-effects-escape-codes)
///     - [Not very well supported](#not-very-well-supported-font-effects-escape-codes)
///     - [Extended underline escape codes](#extended-underline-escape-codes)
///     - [Underline color escape codes](#underline-color-escape-codes)
///   - Color
///     - [Color (3-bit)](#3-bit-color-escape-codes)
///     - [Color (8-bit)](#8-bit-color-escape-codes)
///     - [Grey (8-bit) Palette](#8-bit-grey-escape-codes)
///     - [Palette (8-bit)](#8-bit-palette-colors)
///     - [Color (rgb)](#rgb-color-escape-codes)
///     - [Default rendition colors](#default-rendition-colors)
/// - [OSC sequences](#operating-system-commands)
///   - [title](#method.title_all) (OSC 0)
///   - [title_icon](#method.icon_title) (OSC 1)
///   - [title_window](#method.window_title) (OSC 2)
///   - [link](#method.link) (OSC 8)
///   - [link_with_id](#method.link_with_id)
///   - [clipboard_base64](#method.clipboard_base64) (OSC 52)
///   - [clipboard_query](#method.clipboard_query)
///   - [clipboard_query_clipboard](#method.clipboard_query_clipboard)
///
/// ## Misc. functions
/// - [strip_codes](#method.strip_codes)
///
/// # Coordinate Order
/// All cursor positioning functions use `(columns, rows)` order, equivalent to `(x, y)`.
/// This follows graphics API conventions but is the **reverse** of the underlying
/// ANSI sequence which uses `[row;column]` order in escape sequences.
///
/// # Examples
/// ```
/// # use devela::Ansi;
/// assert_eq![Ansi::ITALIC_B, *b"\x1b[3m"];
/// assert_eq![Ansi::ITALIC, "\x1b[3m"];
/// ```
///
/// See also the [`ansi!`] macro.
///
/// [`ansi!`]: crate::ansi
#[derive(Debug)]
pub struct Ansi;

/// # Control prefixes
///
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#C0_control_codes>
impl Ansi {
    __ansi_consts! {
        /// Escape (ESC).
        ///
        /// Starts all the escape sequences
        pub const ESC: [u8; 1] = "\x1b", *b"\x1b";

        /// Bell (BEL).
        ///
        /// - <https://en.wikipedia.org/wiki/ANSI_escape_code#Operating_System_Command_sequences>
        pub const BEL: [u8; 1] = "\x07", *b"\x07";
    }
}
/// <https://en.wikipedia.org/wiki/ANSI_escape_code#Fe_Escape_sequences>
impl Ansi {
    __ansi_consts! {
        /// Control Sequence Introducer (CSI).
        ///
        /// Starts most of the useful sequences,
        /// terminated by a byte in the range 0x40 through 0x7E.
        ///
        /// - <https://en.wikipedia.org/wiki/ANSI_escape_code#CSIsection>
        pub const CSI: [u8; 2] = "\x1b[", *b"\x1b[";

        /// Operating System Command (OSC).
        ///
        /// Starts a control string for the operating system to use, terminated by ST.
        ///
        /// <https://en.wikipedia.org/wiki/ANSI_escape_code#OSC>
        pub const OSC: [u8; 2] = "\x1b]", *b"\x1b]";

        /// Single Shift Three (SS3).
        ///
        /// Select a single character from the G3 alternative character set.
        pub const SS3: [u8; 2] = "\x1bO", *b"\x1bO";

        /// String Terminator (ST).
        ///
        /// Terminates strings in other controls.
        pub const ST: [u8; 2] = "\x1b\\", *b"\x1b\\";
    }
}

/// # Erase escape codes
impl Ansi {
    __ansi_consts! {
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
