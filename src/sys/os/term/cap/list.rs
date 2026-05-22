// devela::sys::os::term::cap::list
//
//! Defines [`TermCap`].
//

crate::test_size_of![TermCap = 1]; // 8 bits
#[doc = crate::_tags!(term runtime)]
/// Terminal capability flag.
#[doc = crate::_doc_location!("sys/os/term")]
///
/// These flags describe independent terminal features.
/// Ordered or exclusive properties, such as color depth,
/// are stored separately in [`TermCaps`][crate::TermCaps].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TermCap {
    /* input */
    /// Keyboard input.
    Keyboard,

    /// Mouse input reporting.
    Mouse,

    /// Focus-in/focus-out reporting.
    Focus,

    /// Bracketed paste reporting.
    BracketedPaste,

    /// Terminal resize reporting.
    Resize,

    /* output */
    /// ANSI/VT escape sequences.
    Ansi,

    /// Cursor movement and visibility control.
    Cursor,

    /// SGR styling, such as bold, reset, and colors.
    Style,

    /// Alternate screen buffer.
    AltScreen,

    /// Synchronized output updates.
    SyncUpdate,

    /* image */
    /// Sixel image output.
    Sixel,

    /// Kitty graphics protocol image output.
    KittyImage,

    /// iTerm2 inline image output.
    ItermImage,

    /* query replies */
    /// Device-attributes query replies.
    QueryDeviceAttrs,

    /// Cursor-position query replies.
    QueryCursorPos,

    /// Color query replies.
    QueryColor,
}
impl TermCap {
    /// Returns the bit representing this capability.
    #[must_use]
    pub const fn bit(self) -> u32 {
        1 << self as u8
    }
}
