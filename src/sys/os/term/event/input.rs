// devela::sys::os::term::event::input
//
//! Terminal input parsing.
//

use crate::{_impl_init, Char, CharIter, Position2, is, pos, slice, unwrap, whilst};
use crate::{EventKey, EventKind, Key, KeyMods, KeyState};

#[doc = crate::_tags!(term event)]
/// Parses terminal input bytes into normalized events.
#[doc = crate::_doc_location!("sys/os/term")]
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

    /* internal methods */

    const fn feed_parsed(&mut self, byte: u8) -> TermParsed {
        match self.state {
            TermInputState::Ground => self.feed_ground(byte),
            TermInputState::Esc => self.feed_esc(byte),
            TermInputState::Csi { .. } => self.feed_csi(byte),
            TermInputState::Utf8 { .. } => self.feed_utf8(byte),
        }
    }
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
    const fn feed_esc(&mut self, byte: u8) -> TermParsed {
        match byte {
            b'[' => {
                self.state = TermInputState::Csi { buf: [0; 16], len: 0 };
                TermParsed::Pending
            }
            b'O' => {
                // SS3 sequence prefix. Keep as unknown for the first seed.
                self.state = TermInputState::Ground;
                TermParsed::Unknown
            }
            _ => {
                self.state = TermInputState::Ground;
                TermParsed::Unknown
            }
        }
    }
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
    const fn parse_csi(&self, args: &[u8], final_byte: u8) -> TermParsed {
        use Key as K;
        let key = match (args, final_byte) {
            (b"", b'A') => K::Up,
            (b"", b'B') => K::Down,
            (b"", b'C') => K::Right,
            (b"", b'D') => K::Left,
            (b"", b'H') => K::Home,
            (b"", b'F') => K::End,
            (b"1", b'~') | (b"7", b'~') => K::Home,
            (b"4", b'~') | (b"8", b'~') => K::End,
            (b"2", b'~') => K::Insert,
            (b"3", b'~') => K::Delete,
            (b"5", b'~') => K::PageUp,
            (b"6", b'~') => K::PageDown,
            _ => return self.parse_csi_reply(args, final_byte),
        };
        TermParsed::Event(Self::key(key))
    }
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
    /// Parses C0 control bytes and DEL.
    const fn parse_control(&self, byte: u8) -> TermParsed {
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
    const fn key(key: Key) -> EventKind {
        Self::modified_key(key, KeyMods::empty())
    }
    const fn modified_key(key: Key, mods: KeyMods) -> EventKind {
        EventKind::Key(EventKey {
            semantic: key,
            physical: key,
            mods,
            state: KeyState::Press,
        })
    }
    const fn mods_control() -> KeyMods {
        let mut mods = KeyMods::empty();
        mods.set_control();
        mods
    }
}

const fn is_csi_final(byte: u8) -> bool {
    byte >= 0x40 && byte <= 0x7E
}
const fn parse_two_u16(bytes: &[u8], sep: u8) -> Option<(u16, u16)> {
    let mut split = None;
    whilst! { i in 0..bytes.len(); {
        is! { bytes[i] == sep, { split = Some(i); break; }}
    }}
    let split = unwrap![some? split];
    let a = unwrap![some? parse_u16(slice!(bytes, ..split))];
    let b = unwrap![some? parse_u16(slice!(bytes, split + 1, ..))];
    Some((a, b))
}
const fn parse_u16(bytes: &[u8]) -> Option<u16> {
    is! { bytes.is_empty(), return None }
    let mut n = 0u16;
    whilst! { b in 0..bytes.len(); {
        let byte = bytes[b];
        is!{ !byte.is_ascii_digit(), return None }
        n = unwrap![some?
            unwrap![some? n.checked_mul(10)]
                .checked_add((byte - b'0') as u16)];
    }}
    Some(n)
}

/// Internal parser state.
#[derive(Clone, Debug, Default)]
enum TermInputState {
    /// No partial sequence is active.
    #[default]
    Ground,
    /// A single `ESC` byte has been received.
    Esc,
    /// A Control Sequence Introducer sequence is being collected.
    Csi { buf: [u8; 16], len: u8 },
    /// A UTF-8 scalar is being collected.
    Utf8 { buf: [u8; 4], len: u8, need: u8 },
}
_impl_init! { Self::Ground => TermInputState }

/// Internal parser result.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TermParsed {
    /// A normalized user-facing event.
    Event(EventKind),
    /// A terminal reply, usually produced after a query.
    Reply(TermReply),
    /// The current sequence is incomplete.
    Pending,
    /// The sequence is complete but currently unsupported or invalid.
    Unknown,
}

/// Terminal reply parsed from the input stream.
#[derive(Clone, Debug, PartialEq, Eq)]
enum TermReply {
    /// Cursor-position report: `ESC [ row ; col R`.
    ///
    /// The position is in terminal cells, meaning x = column and y = row.
    CursorPosition(Position2<u16>),
    /// Device-attributes reply: `ESC [ ... c`.
    DeviceAttributes,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventKey, EventKind, Key, KeyState};

    fn feed_seq(parser: &mut TermInputParser, bytes: &[u8]) -> Option<EventKind> {
        let mut out = None;
        for &b in bytes {
            out = parser.feed(b);
        }
        out
    }

    fn key_from(kind: EventKind) -> EventKey {
        match kind {
            EventKind::Key(k) => k,
            other => panic!("expected key event, got {other:?}"),
        }
    }

    fn assert_key(bytes: &[u8], expected: Key) {
        let mut p = TermInputParser::new();
        let k = key_from(feed_seq(&mut p, bytes).expect("expected event"));
        assert_eq!(k.semantic, expected);
        assert_eq!(k.physical, expected);
        assert_eq!(k.state, KeyState::Press);
        assert_eq!(k.mods, crate::KeyMods::empty());
    }

    #[test]
    fn ascii_printable() {
        assert_key(b"a", Key::Char('a'));
        assert_key(b"Z", Key::Char('Z'));
        assert_key(b"7", Key::Char('7'));
    }

    #[test]
    fn ascii_space_is_space_key() {
        assert_key(b" ", Key::Space);
    }

    #[test]
    fn enter_variants() {
        assert_key(b"\r", Key::Enter);
        assert_key(b"\n", Key::Enter);
    }

    #[test]
    fn tab() {
        assert_key(b"\t", Key::Tab);
    }

    #[test]
    fn backspace_variants() {
        assert_key(&[0x08], Key::Backspace);
        assert_key(&[0x7f], Key::Backspace);
    }

    #[test]
    fn escape_is_pending_until_flushed() {
        let mut p = TermInputParser::new();

        assert_eq!(p.feed(0x1b), None);

        let ev = p.flush_escape().expect("expected escaped key after flush");
        let k = key_from(ev);
        assert_eq!(k.semantic, Key::Escape);
        assert_eq!(k.physical, Key::Escape);
        assert_eq!(k.state, KeyState::Press);
    }

    #[test]
    fn csi_arrows() {
        assert_key(b"\x1b[A", Key::Up);
        assert_key(b"\x1b[B", Key::Down);
        assert_key(b"\x1b[C", Key::Right);
        assert_key(b"\x1b[D", Key::Left);
    }

    #[test]
    fn csi_home_end_plain() {
        assert_key(b"\x1b[H", Key::Home);
        assert_key(b"\x1b[F", Key::End);
    }

    #[test]
    fn csi_navigation_tilde() {
        assert_key(b"\x1b[1~", Key::Home);
        assert_key(b"\x1b[7~", Key::Home);
        assert_key(b"\x1b[4~", Key::End);
        assert_key(b"\x1b[8~", Key::End);
        assert_key(b"\x1b[5~", Key::PageUp);
        assert_key(b"\x1b[6~", Key::PageDown);
    }

    #[test]
    fn csi_editing_tilde() {
        assert_key(b"\x1b[2~", Key::Insert);
        assert_key(b"\x1b[3~", Key::Delete);
    }

    #[test]
    fn utf8_two_bytes() {
        assert_key("ñ".as_bytes(), Key::Char('ñ'));
    }

    #[test]
    fn utf8_three_bytes() {
        assert_key("€".as_bytes(), Key::Char('€'));
    }

    #[test]
    fn utf8_four_bytes() {
        assert_key("🦀".as_bytes(), Key::Char('🦀'));
    }

    #[test]
    fn invalid_utf8_continuation_is_unknown() {
        let mut p = TermInputParser::new();

        assert_eq!(p.feed_parsed(0xC2), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'a'), TermParsed::Unknown);
    }

    #[test]
    fn csi_cursor_position_reply() {
        let mut p = TermInputParser::new();

        assert_eq!(p.feed_parsed(0x1b), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'['), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'1'), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'2'), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b';'), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'4'), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'0'), TermParsed::Pending);

        assert_eq!(p.feed_parsed(b'R'), TermParsed::Reply(TermReply::CursorPosition(pos![40, 12])),);
    }

    #[test]
    fn public_feed_swallows_internal_replies() {
        let mut p = TermInputParser::new();

        let mut out = None;
        for &b in b"\x1b[12;40R" {
            out = p.feed(b);
        }

        assert_eq!(out, None);
    }

    #[test]
    fn csi_device_attributes_reply() {
        let mut p = TermInputParser::new();

        let mut parsed = TermParsed::Pending;
        for &b in b"\x1b[?1;2c" {
            parsed = p.feed_parsed(b);
        }

        assert_eq!(parsed, TermParsed::Reply(TermReply::DeviceAttributes));
    }

    #[test]
    fn ctrl_a_maps_to_control_char() {
        let mut p = TermInputParser::new();

        let ev = p.feed(0x01).expect("expected event");
        let k = key_from(ev);

        assert_eq!(k.semantic, Key::Char('a'));
        assert_eq!(k.physical, Key::Char('a'));
        assert!(k.mods.has_control());
    }

    #[test]
    fn ctrl_z_maps_to_control_char() {
        let mut p = TermInputParser::new();

        let ev = p.feed(0x1a).expect("expected event");
        let k = key_from(ev);

        assert_eq!(k.semantic, Key::Char('z'));
        assert_eq!(k.physical, Key::Char('z'));
        assert!(k.mods.has_control());
    }
    #[test]
    fn parse_control_named_controls() {
        let p = TermInputParser::new();

        assert_eq!(p.parse_control(b'\t'), TermParsed::Event(TermInputParser::key(Key::Tab)),);
        assert_eq!(p.parse_control(b'\n'), TermParsed::Event(TermInputParser::key(Key::Enter)),);
        assert_eq!(p.parse_control(b'\r'), TermParsed::Event(TermInputParser::key(Key::Enter)),);
        assert_eq!(p.parse_control(0x08), TermParsed::Event(TermInputParser::key(Key::Backspace)),);
        assert_eq!(p.parse_control(0x7f), TermParsed::Event(TermInputParser::key(Key::Backspace)),);
    }

    #[test]
    fn parse_control_letters() {
        let p = TermInputParser::new();

        let TermParsed::Event(EventKind::Key(k)) = p.parse_control(0x01) else {
            panic!("expected Ctrl-A key event");
        };
        assert_eq!(k.semantic, Key::Char('a'));
        assert_eq!(k.physical, Key::Char('a'));
        assert!(k.mods.has_control());

        let TermParsed::Event(EventKind::Key(k)) = p.parse_control(0x1a) else {
            panic!("expected Ctrl-Z key event");
        };
        assert_eq!(k.semantic, Key::Char('z'));
        assert_eq!(k.physical, Key::Char('z'));
        assert!(k.mods.has_control());
    }

    #[test]
    fn parse_control_punctuation() {
        let p = TermInputParser::new();

        let cases = [
            (0x00, Key::Space),
            (0x1c, Key::Char('\\')),
            (0x1d, Key::Char(']')),
            (0x1e, Key::Char('^')),
            (0x1f, Key::Char('_')),
        ];

        for (byte, key) in cases {
            let TermParsed::Event(EventKind::Key(k)) = p.parse_control(byte) else {
                panic!("expected control punctuation key event for {byte:#04x}");
            };
            assert_eq!(k.semantic, key);
            assert_eq!(k.physical, key);
            assert!(k.mods.has_control());
        }
    }
}
