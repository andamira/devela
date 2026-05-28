// devela::sys::os::term::event::input::csi_control

use crate::{Char, is, pos, slice, unwrap, whilst};
use crate::{EventKey, EventKind, Key, KeyMods};
use crate::{TermInputParser, TermParsed, TermParsedCsi, TermReply};

impl TermInputParser {
    /* control  */

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
    pub(super) const fn mods_control() -> KeyMods {
        let mut mods = KeyMods::empty();
        mods.set_control();
        mods
    }

    /* CSI */

    /// Maps a complete CSI sequence to an event, parser state change, or terminal reply.
    pub(super) const fn parse_csi(&mut self, args: &[u8], final_byte: u8) -> TermParsed {
        match self.parse_csi_protocol(args, final_byte) {
            TermParsedCsi::Continue => {}
            parsed => return parsed.to_term_parsed(),
        }
        match Self::parse_csi_event(args, final_byte) {
            TermParsedCsi::Continue => {}
            parsed => return parsed.to_term_parsed(),
        }
        match Self::parse_csi_key(args, final_byte) {
            TermParsedCsi::Continue => {}
            parsed => return parsed.to_term_parsed(),
        }
        self.parse_csi_reply(args, final_byte).to_term_parsed()
    }
    /// Handles CSI sequences that change parser/input protocol state.
    const fn parse_csi_protocol(&mut self, args: &[u8], final_byte: u8) -> TermParsedCsi {
        match (args, final_byte) {
            // Bracketed paste begin: ESC [ 200 ~
            (b"200", b'~') => {
                self.paste = true;
                TermParsedCsi::Pending
            }
            // Bracketed paste end: ESC [ 201 ~
            (b"201", b'~') => {
                self.paste = false;
                TermParsedCsi::Pending
            }
            _ => TermParsedCsi::Continue,
        }
    }
    /// Maps CSI sequences that directly represent normalized events.
    const fn parse_csi_event(args: &[u8], final_byte: u8) -> TermParsedCsi {
        match (args, final_byte) {
            // Focus reporting: ESC [ I / ESC [ O
            (b"", b'I') => TermParsedCsi::FocusGained,
            (b"", b'O') => TermParsedCsi::FocusLost,
            _ => TermParsedCsi::Continue,
        }
    }
    /// Maps CSI keyboard sequences to keys.
    const fn parse_csi_key(args: &[u8], final_byte: u8) -> TermParsedCsi {
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

            (b"11", b'~') => K::Fn(1),
            (b"12", b'~') => K::Fn(2),
            (b"13", b'~') => K::Fn(3),
            (b"14", b'~') => K::Fn(4),
            (b"15", b'~') => K::Fn(5),
            (b"17", b'~') => K::Fn(6),
            (b"18", b'~') => K::Fn(7),
            (b"19", b'~') => K::Fn(8),
            (b"20", b'~') => K::Fn(9),
            (b"21", b'~') => K::Fn(10),
            (b"23", b'~') => K::Fn(11),
            (b"24", b'~') => K::Fn(12),

            _ => return TermParsedCsi::Continue,
        };
        TermParsedCsi::Key(key)
    }
    /// Maps a complete CSI terminal response to an internal reply.
    const fn parse_csi_reply(&self, args: &[u8], final_byte: u8) -> TermParsedCsi {
        // DSR cursor-position reply: ESC [ row ; col R
        if final_byte == b'R' {
            if let Some((row, col)) = Self::parse_two_u16(args, b';') {
                return TermParsedCsi::Reply(TermReply::CursorPosition(pos![col, row]));
            }
        }
        // DA reply seed: ESC [ ? ... c or ESC [ ... c
        is! { final_byte == b'c', return TermParsedCsi::Reply(TermReply::DeviceAttributes) }
        TermParsedCsi::Unknown
    }

    /* misc. helpers */

    const fn parse_two_u16(bytes: &[u8], sep: u8) -> Option<(u16, u16)> {
        let mut split = None;
        whilst! { i in 0..bytes.len(); {
            is! { bytes[i] == sep, { split = Some(i); break; }}
        }}
        let split = unwrap![some? split];
        let a = unwrap![some? Self::parse_u16(slice!(bytes, ..split))];
        let b = unwrap![some? Self::parse_u16(slice!(bytes, split + 1, ..))];
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

    /// Returns whether bracketed paste input is currently active.
    #[must_use]
    #[allow(dead_code)]
    pub(super) const fn is_pasting(&self) -> bool {
        self.paste
    }
}
