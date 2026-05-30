// devela::sys::os::term::event::input::parser
//
//! Defines [`TermInputParser`].
//

use crate::{_impl_init, EventKind, Key, TermInputState, TermParsed};

#[doc = crate::_tags!(term event parser)]
/// Parses terminal input bytes into normalized events.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    test_size_of(TermInputParser = 19|152),
}]
///
/// `TermInputParser` is a byte-fed state machine. It accepts ordinary bytes,
/// UTF-8 text, and terminal escape sequences, returning an [`EventKind`] when
/// a complete input event has been recognized.
///
/// It is intentionally independent from any concrete terminal backend. Linux,
/// Windows, web-terminal, pseudo-terminal, and test backends can all feed bytes
/// into the same parser.
///
/// # Public output
/// The public [`feed`][Self::feed] method returns only normalized user-facing events.
///
/// Terminal replies used for probing capabilities are parsed internally
/// and are exposed only inside the crate.
///
/// # Escape handling
/// A lone `ESC` byte is ambiguous: it may be the Escape key, or it may begin a
/// longer escape sequence. Backends should call [`flush_escape`][Self::flush_escape]
/// after their escape timeout expires.
///
/// # Supported seed grammar
/// The first parser layer recognizes:
/// - printable ASCII
/// - UTF-8 scalar values
/// - Enter, Tab, Backspace, Escape
/// - basic Control-letter combinations
/// - common CSI navigation and editing keys
/// - cursor-position and device-attribute replies, internally
///
/// # Terminal limits
///
/// Terminal input is normalized from the byte stream reported by the active
/// terminal emulator. Some information may be unavailable because terminals
/// often reserve key or mouse combinations for selection, menus, shortcuts, or
/// scrollback behavior before the application receives them.
///
/// In SGR mouse mode, terminal reports can encode Shift, Alt/Meta, and Control
/// modifier bits. `TermInputParser` preserves those bits when present, but
/// applications should not assume every terminal will report every modifier
/// combination. In particular, Shift-click and Control-click are commonly
/// intercepted or repurposed by terminal emulators.
#[derive(Clone, Debug, Default)]
pub struct TermInputParser {
    pub(super) state: TermInputState,
    pub(super) paste: bool,
}
_impl_init! { Self::new() => TermInputParser }

impl TermInputParser {
    /// Returns a new parser in the ground state.
    #[must_use]
    pub const fn new() -> Self {
        Self { state: TermInputState::Ground, paste: false }
    }

    /// Feeds one byte into the parser.
    ///
    /// Returns `Some(EventKind)` when the byte completes a user-facing event.
    ///
    /// Returns `None` while a multi-byte sequence is still pending, or when the
    /// completed sequence is an internal terminal reply.
    ///
    /// Use [`flush_escape`][Self::flush_escape] to resolve a pending lone `ESC`
    /// after the backend's escape timeout expires.
    //
    // NOTE: not const because it discards `TermParsed`,
    // whose event path may carry non-const-drop event payloads.
    pub fn feed(&mut self, byte: u8) -> Option<EventKind> {
        match self.feed_parsed(byte) {
            TermParsed::Event(ev) => Some(ev),
            _ => None,
        }
    }

    #[cfg(not(feature = "alloc"))]
    #[cfg_attr(nightly_doc, doc(cfg(not(feature = "alloc"))))]
    /// Feeds one byte into the parser in const contexts.
    ///
    /// This is only available without `alloc`,
    /// because alloc-enabled events may contain owned values with destructors.
    pub const fn feed_const(&mut self, byte: u8) -> Option<EventKind> {
        match self.feed_parsed(byte) {
            TermParsed::Event(ev) => Some(ev),
            _ => None,
        }
    }

    /// Flushes a pending lone `ESC` as an Escape key press.
    ///
    /// Returns `None` unless the parser is currently waiting after an `ESC` byte.
    ///
    /// This is needed because terminal escape sequences
    /// and the Escape key share the same leading byte.
    pub const fn flush_escape(&mut self) -> Option<EventKind> {
        if matches!(self.state, TermInputState::Esc) {
            self.state = TermInputState::Ground;
            Some(Self::key(Key::Escape))
        } else {
            None
        }
    }
}
