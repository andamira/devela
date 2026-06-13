// devela/src/sys/os/term/event/input/feed.rs

use crate::{Char, CharIter, Key, TermInputParser, TermInputState, TermParsed, slice, unwrap};

// internal feed_* methods
impl TermInputParser {
    /// Advances the parser by one byte and returns the internal parse result.
    pub(crate) const fn feed_parsed(&mut self, byte: u8) -> TermParsed {
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
        if Self::is_csi_final(byte) {
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
    const fn is_csi_final(byte: u8) -> bool {
        byte >= 0x40 && byte <= 0x7E
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
}
