// devela::sys::os::term::event::input::csi_control

use crate::{Char, is, pos, slice, unwrap, whilst};
use crate::{
    EventButton, EventButtonState, EventButtons, EventKey, EventKind, EventMouse, EventWheel,
    EventWheelUnit, Key, KeyMods,
};
use crate::{TermDecModeStatus, TermInputParser, TermParsed, TermParsedCsi, TermReply};

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
        let mut mods = KeyMods::new();
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
        match Self::parse_csi_mouse(args, final_byte) {
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
    ///
    const fn parse_csi_mouse(args: &[u8], final_byte: u8) -> TermParsedCsi {
        let released = match final_byte {
            b'M' => false,
            b'm' => true,
            _ => return TermParsedCsi::Continue,
        };
        let Some((cb, cx, cy)) = Self::parse_sgr_mouse_args(args) else {
            return TermParsedCsi::Continue;
        };
        Self::sgr_pointer_event(cb, cx, cy, released)
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
        // DECRPM: ESC [ ? mode ; status $ y
        if final_byte == b'y' {
            if let Some((mode, status)) = Self::parse_dec_private_mode_reply(args) {
                return TermParsedCsi::Reply(TermReply::DecPrivateMode { mode, status });
            }
        }
        // DA reply seed: ESC [ ? ... c or ESC [ ... c
        is! { final_byte == b'c', return TermParsedCsi::Reply(TermReply::DeviceAttributes) }
        TermParsedCsi::Unknown
    }
    const fn parse_dec_private_mode_reply(args: &[u8]) -> Option<(u16, TermDecModeStatus)> {
        // Expects args from: ESC [ ? 1016 ; 2 $ y
        // args = b"?1016;2$"
        is! { args.len() < 5, return None }
        is! { args[0] != b'?', return None }
        is! { args[args.len() - 1] != b'$', return None }
        let body = slice![args, 1, ..args.len() - 1];
        let (mode, raw_status) = unwrap![some? Self::parse_two_u16(body, b';')];
        let status = unwrap![some? TermDecModeStatus::from_u16(raw_status)];
        Some((mode, status))
    }

    /* mouse/pointer/wheel helpers */

    const fn parse_sgr_mouse_args(bytes: &[u8]) -> Option<(u16, u16, u16)> {
        is! { bytes.len() < 6 || bytes[0] != b'<', return None } // expects: b"<Cb;Cx;Cy"
        let body = slice!(bytes, 1, ..);
        let (mut first, mut second) = (None, None);
        whilst! { i in 0..body.len(); {
            if body[i] == b';' {
                if first.is_none() {
                    first = Some(i);
                } else {
                    second = Some(i);
                    break;
                }
            }
        }}
        let first = unwrap![some? first];
        let second = unwrap![some? second];
        let cb = unwrap![some? Self::parse_u16(slice!(body, ..first))];
        let cx = unwrap![some? Self::parse_u16(slice!(body, first + 1, ..second))];
        let cy = unwrap![some? Self::parse_u16(slice!(body, second + 1, ..))];
        Some((cb, cx, cy))
    }
    const fn sgr_pointer_event(cb: u16, cx: u16, cy: u16, released: bool) -> TermParsedCsi {
        if Self::sgr_is_wheel(cb) {
            let Some(wheel) = Self::wheel_sgr_event(cb, cx, cy) else {
                return TermParsedCsi::Unknown;
            };
            return TermParsedCsi::Wheel(wheel);
        }
        let Some(mouse) = Self::mouse_sgr_event(cb, cx, cy, released) else {
            return TermParsedCsi::Unknown;
        };
        TermParsedCsi::Mouse(mouse)
    }

    const fn mouse_sgr_event(cb: u16, cx: u16, cy: u16, released: bool) -> Option<EventMouse> {
        // wheel events are normalized separately as EventWheel
        is! { Self::sgr_is_wheel(cb), return None } // cold

        // normalize terminal coordinates to 0-based units.
        // In SGR mode these are cells; in SGR-pixels mode these are pixels.
        let (x, y) = (cx.saturating_sub(1) as i32, cy.saturating_sub(1) as i32);
        let motion = cb & 32 != 0;
        let state = if motion {
            EventButtonState::Moved
        } else if released {
            EventButtonState::Released
        } else {
            EventButtonState::Pressed
        };
        let button = Self::sgr_mouse_button(cb);
        let buttons = Self::sgr_mouse_buttons(state, button);
        let mods = Self::sgr_mouse_mods(cb);
        Some(EventMouse::new(x, y, button, state, buttons, mods))
    }
    const fn sgr_mouse_mods(cb: u16) -> KeyMods {
        let mut mods = KeyMods::new();
        is! { cb & 4 != 0, mods.set_shift() }
        is! { cb & 8 != 0, mods.set_alt() }
        is! { cb & 16 != 0, mods.set_control() }
        mods
    }
    const fn sgr_mouse_button(cb: u16) -> Option<EventButton> {
        let code = cb & 0b11;
        is! { Self::sgr_is_wheel(cb), return None } // cold
        if cb & 128 != 0 {
            return match code {
                0 => Some(EventButton::X1),
                1 => Some(EventButton::X2),
                2 => Some(EventButton::X3),
                3 => Some(EventButton::X4),
                _ => None,
            };
        }
        match code {
            0 => Some(EventButton::Left),
            1 => Some(EventButton::Middle),
            2 => Some(EventButton::Right),
            3 => None,
            _ => None,
        }
    }
    const fn sgr_mouse_buttons(
        state: EventButtonState,
        button: Option<EventButton>,
    ) -> EventButtons {
        match (state, button) {
            (EventButtonState::Pressed | EventButtonState::Moved, Some(button)) => button.to_mask(),
            _ => EventButtons::new(),
        }
    }
    #[rustfmt::skip]
    // SGR mouse uses the 64 family for wheel-like reports in the subset we
    // normalize as up/down/left/right wheel motion. The 128 family is kept for
    // extended auxiliary buttons and must not be swallowed as wheel input.
    const fn wheel_sgr_event(cb: u16, cx: u16, cy: u16) -> Option<EventWheel> {
        is! { !Self::sgr_is_wheel(cb), return None } // cold
        // normalize coordinates to 0-based cells
        let (x, y) = (cx.saturating_sub(1) as i32, cy.saturating_sub(1) as i32);
        let (delta_x, delta_y) = match cb & 0b11 {
            0 => (0, -1), // wheel up
            1 => (0, 1),  // wheel down
            2 => (-1, 0), // wheel left
            3 => (1, 0),  // wheel right
            _ => return None,
        };
        Some(EventWheel::new(delta_x, delta_y, EventWheelUnit::Step, x, y,
            EventButtons::new(), Self::sgr_mouse_mods(cb)))
    }
    #[rustfmt::skip]
    // accepts horizontal wheel while being conservative
    const fn sgr_is_wheel(cb: u16) -> bool { cb & 64 != 0 && cb & 128 == 0 }

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
