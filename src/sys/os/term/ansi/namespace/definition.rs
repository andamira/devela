// devela::sys::os::term::ansi::namespace::definition
//
//! Defines the [`Ansi`] namespace for emitting ANSI codes.
//

use crate::{__ansi_consts, Digits, slice};

#[doc = crate::_tags!(term namespace)]
/// ANSI escape codes.
#[doc = crate::_doc_location!("sys/os/term")]
///
/// # Return type
/// Constants ending with `_B` return a byte array. Those without it return a string slice.
///
/// Functions ending with `_B` return either an array or a byte slice.
/// Those without it return a [`StringNonul`][crateStringNonul] or a string slice, respectively.
///
/// # Examples
/// ```
/// use devela::Ansi;
/// assert_eq![Ansi::ITALIC_B, *b"\x1b[3m"];
/// assert_eq![Ansi::ITALIC, "\x1b[3m"];
/// ```
///
/// # Methods
/// ## Escape codes
/// - [Control prefixes][Self#screen-escape-codes]
/// - [Screen][Self#screen-escape-codes]
/// - [Erase][Self#erase-escape-codes]
/// - [Cursor][Self#cursor-escape-codes]
/// - [Mouse][Self#mouse-escape-codes]
/// - [Font effects][Self#font-effects-escape-codes]
/// - [Color (3-bit)][Self#3-bit-color-escape-codes]
/// - [Color (8-bit)][Self#8-bit-color-escape-codes]
/// - [Grey (8-bit) Palette][Self#8-bit-grey-escape-codes]
/// - [Palette (8-bit)][Self#8-bit-palette-escape-codes]
/// - [Default][Self#default-color-escape-codes]
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
#[doc = crate::doclink!(custom devela "[`ansi!`]" "sys/os/term/macro.ansi.html")]
#[derive(Debug)]
pub struct Ansi;

/// # Control prefixes
impl Ansi {
    __ansi_consts! {
        /// Control Sequence Introducer (CSI).
        ///
        /// <https://en.wikipedia.org/wiki/ANSI_escape_code#Control_Sequence_Introducer_commands>
        pub const CSI: [u8; 2] = "\x1b[", *b"\x1b[";
    }

    /* helper functions */

    // Writes an ANSI CSI code with one dynamic `u16` argument.
    //
    // Required capacity: `ESC [` + 5 digits + final byte = 8 bytes.
    #[must_use]
    pub(super) const fn write_ansi_code_n(buffer: &mut [u8], n: u16, final_byte: u8) -> &[u8] {
        assert![buffer.len() >= 8];
        buffer[0] = b'\x1b';
        buffer[1] = b'[';
        let mut index = 2;
        index += Digits(n).write_digits10_fast(buffer, index);
        buffer[index] = final_byte;
        slice![buffer, ..=index]
    }
}

/// # Screen escape codes
impl Ansi {
    __ansi_consts! {
        /// Code to enable the alternative screen.
        pub const ENABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049h", *b"\x1b[?1049h";
        /// Code to disable the alternative screen.
        pub const DISABLE_ALT_SCREEN: [u8; 8] = "\x1b[?1049l", *b"\x1b[?1049l";
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
