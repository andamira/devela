// devela::sys::os::term::event::input::parser
//
//! Terminal input parsing.
//

use super::_helper::*;
use crate::{_impl_init, Char, CharIter, pos, slice, unwrap};
use crate::{EventKey, EventKind, Key, KeyMods};

#[doc = crate::_tags!(term event parser)]
/// Parses terminal input bytes into normalized events.
#[doc = crate::_doc_meta!{
    location("sys/os/term"),
    test_size_of(TermInputParser = 18|144),
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
#[derive(Clone, Debug, Default)]
pub struct TermInputParser {
    state: TermInputState,
}
_impl_init! { Self::new() => TermInputParser }

impl TermInputParser {
    /// Returns a new parser in the ground state.
    #[must_use]
    pub const fn new() -> Self {
        Self { state: TermInputState::Ground }
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

// Internal methods
impl TermInputParser {
    /* feed_ */

    /// Advances the parser by one byte and returns the internal parse result.
    pub(super) const fn feed_parsed(&mut self, byte: u8) -> TermParsed {
        match self.state {
            TermInputState::Ground => self.feed_ground(byte),
            TermInputState::Esc => self.feed_esc(byte),
            TermInputState::Ss3 => self.feed_ss3(byte),
            TermInputState::Csi { .. } => self.feed_csi(byte),
            TermInputState::Utf8 { .. } => self.feed_utf8(byte),
        }
    }
    /// Parses one byte while no partial escape or UTF-8 sequence is active.
    #[allow(clippy::match_overlapping_arm, reason = "b' ' with 0x20..=0x7E")]
    const fn feed_ground(&mut self, byte: u8) -> TermParsed {
        match byte {
            // ESC is stateful: it may become Escape, CSI, SS3, mouse, paste, etc.
            0x1B => {
                self.state = TermInputState::Esc;
                TermParsed::Pending
            }
            // C0 controls and DEL.
            //
            // This includes Enter, Tab, Backspace, Ctrl-A..Ctrl-Z,
            // Ctrl-Space, Ctrl-\ , Ctrl-] , Ctrl-^ , Ctrl-_.
            0x00..=0x1F | 0x7F => self.parse_control(byte),
            // Printable ASCII.
            b' ' => TermParsed::Event(Self::key(Key::Space)),
            0x20..=0x7E => TermParsed::Event(Self::key(Key::Char(Char(byte).as_char()))),
            // UTF-8 leading bytes.
            0xC2..=0xDF => {
                self.state = TermInputState::Utf8 { buf: [byte, 0, 0, 0], len: 1, need: 2 };
                TermParsed::Pending
            }
            0xE0..=0xEF => {
                self.state = TermInputState::Utf8 { buf: [byte, 0, 0, 0], len: 1, need: 3 };
                TermParsed::Pending
            }
            0xF0..=0xF4 => {
                self.state = TermInputState::Utf8 { buf: [byte, 0, 0, 0], len: 1, need: 4 };
                TermParsed::Pending
            }
            // Continuation bytes, overlong leading bytes, and invalid UTF-8 heads.
            _ => TermParsed::Unknown,
        }
    }
    /// Parses the byte following `ESC`, selecting a control-sequence family.
    const fn feed_esc(&mut self, byte: u8) -> TermParsed {
        match byte {
            b'[' => {
                self.state = TermInputState::Csi { buf: [0; 16], len: 0 };
                TermParsed::Pending
            }
            b'O' => {
                self.state = TermInputState::Ss3;
                TermParsed::Pending
            }
            _ => {
                self.state = TermInputState::Ground;
                TermParsed::Unknown
            }
        }
    }
    /// Parses one byte of an SS3 (`ESC O`) key sequence.
    const fn feed_ss3(&mut self, byte: u8) -> TermParsed {
        self.state = TermInputState::Ground;
        let key = match byte {
            b'A' => Key::Up,
            b'B' => Key::Down,
            b'C' => Key::Right,
            b'D' => Key::Left,
            b'H' => Key::Home,
            b'F' => Key::End,
            b'P' => Key::Fn(1),
            b'Q' => Key::Fn(2),
            b'R' => Key::Fn(3),
            b'S' => Key::Fn(4),
            _ => return TermParsed::Unknown,
        };
        TermParsed::Event(Self::key(key))
    }
    /// Collects bytes of a CSI (`ESC [`) sequence until its final byte.
    const fn feed_csi(&mut self, byte: u8) -> TermParsed {
        let TermInputState::Csi { mut buf, mut len } = self.state else {
            return TermParsed::Unknown;
        };
        if is_csi_final(byte) {
            self.state = TermInputState::Ground;
            self.parse_csi(slice!(&buf, ..len as usize), byte)
        } else if (len as usize) < buf.len() {
            buf[len as usize] = byte;
            len += 1;
            self.state = TermInputState::Csi { buf, len };
            TermParsed::Pending
        } else {
            self.state = TermInputState::Ground;
            TermParsed::Unknown
        }
    }
    /// Collects and validates the continuation bytes of one UTF-8 scalar.
    const fn feed_utf8(&mut self, byte: u8) -> TermParsed {
        let TermInputState::Utf8 { mut buf, mut len, need } = self.state else {
            return TermParsed::Unknown;
        };
        if byte & 0b1100_0000 != 0b1000_0000 {
            self.state = TermInputState::Ground;
            return TermParsed::Unknown;
        }
        buf[len as usize] = byte;
        len += 1;
        if len == need {
            self.state = TermInputState::Ground;
            match core::str::from_utf8(slice!(&buf, ..need as usize)) {
                Ok(s) => {
                    let c = unwrap![some_or CharIter::<&str>::new(s).next_char(),
                        char::REPLACEMENT_CHARACTER];
                    TermParsed::Event(Self::key(Key::Char(c)))
                }
                Err(_) => TermParsed::Unknown,
            }
        } else {
            self.state = TermInputState::Utf8 { buf, len, need };
            TermParsed::Pending
        }
    }

    /* parse_ */

    /// Maps C0 control bytes and DEL to normalized key events.
    pub(super) const fn parse_control(&self, byte: u8) -> TermParsed {
        match byte {
            // Practical terminal/editor meanings.
            b'\t' => TermParsed::Event(Self::key(Key::Tab)),
            b'\n' | b'\r' => TermParsed::Event(Self::key(Key::Enter)),
            0x08 | 0x7F => TermParsed::Event(Self::key(Key::Backspace)),
            // Ctrl-A..Ctrl-Z, except the named controls above.
            0x01..=0x07 | 0x0B..=0x0C | 0x0E..=0x1A => {
                let c = Char(b'a' + byte - 1).as_char();
                TermParsed::Event(Self::modified_key(Key::Char(c), Self::mods_control()))
            }
            // Common punctuation controls.
            0x00 => TermParsed::Event(Self::modified_key(Key::Space, Self::mods_control())),
            0x1C => TermParsed::Event(Self::modified_key(Key::Char('\\'), Self::mods_control())),
            0x1D => TermParsed::Event(Self::modified_key(Key::Char(']'), Self::mods_control())),
            0x1E => TermParsed::Event(Self::modified_key(Key::Char('^'), Self::mods_control())),
            0x1F => TermParsed::Event(Self::modified_key(Key::Char('_'), Self::mods_control())),
            // ESC belongs to `feed_ground`, because it changes parser state.
            _ => TermParsed::Unknown,
        }
    }
    /// Maps a complete CSI sequence to a key event or terminal reply.
    const fn parse_csi(&self, args: &[u8], final_byte: u8) -> TermParsed {
        use Key as K;
        let key = match (args, final_byte) {
            (b"", b'A') => K::Up,
            (b"", b'B') => K::Down,
            (b"", b'C') => K::Right,
            (b"", b'D') => K::Left,
            (b"", b'H') => K::Home,
            (b"", b'F') => K::End,
            (b"1" | b"7", b'~') => K::Home,
            (b"4" | b"8", b'~') => K::End,
            (b"2", b'~') => K::Insert,
            (b"3", b'~') => K::Delete,
            (b"5", b'~') => K::PageUp,
            (b"6", b'~') => K::PageDown,
            (b"15", b'~') => K::Fn(5),
            (b"17", b'~') => K::Fn(6),
            (b"18", b'~') => K::Fn(7),
            (b"19", b'~') => K::Fn(8),
            (b"20", b'~') => K::Fn(9),
            (b"21", b'~') => K::Fn(10),
            (b"23", b'~') => K::Fn(11),
            (b"24", b'~') => K::Fn(12),
            (b"11", b'~') => K::Fn(1),
            (b"12", b'~') => K::Fn(2),
            (b"13", b'~') => K::Fn(3),
            (b"14", b'~') => K::Fn(4),
            _ => return self.parse_csi_reply(args, final_byte),
        };
        TermParsed::Event(Self::key(key))
    }
    /// Maps a complete CSI terminal response to an internal reply.
    const fn parse_csi_reply(&self, args: &[u8], final_byte: u8) -> TermParsed {
        // DSR cursor-position reply: ESC [ row ; col R
        if final_byte == b'R' {
            if let Some((row, col)) = parse_two_u16(args, b';') {
                return TermParsed::Reply(TermReply::CursorPosition(pos![col, row]));
            }
        }
        // DA reply seed: ESC [ ? ... c or ESC [ ... c
        if final_byte == b'c' {
            return TermParsed::Reply(TermReply::DeviceAttributes);
        }
        TermParsed::Unknown
    }

    /* key */

    /// Creates a named key press where semantic and physical keys coincide.
    pub(super) const fn key(key: Key) -> EventKind {
        EventKind::Key(EventKey::press(key))
    }
    /// Creates a modified text-producing key event with unknown physical origin.
    pub(super) const fn modified_key(key: Key, mods: KeyMods) -> EventKind {
        EventKind::Key(EventKey::modified_press(key, mods))
    }
    // /// Creates a text-producing key event from a Unicode scalar.
    // pub(super) const fn text_key(c: char) -> EventKind {
    //     EventKind::Key(EventKey::text(c))
    // }
    pub(super) const fn mods_control() -> KeyMods {
        let mut mods = KeyMods::empty();
        mods.set_control();
        mods
    }
}
