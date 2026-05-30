// devela::sys::os::term::event::input::tests
//
// TOC
// - mod control
// - mod csi_keys_replies
// - mod ascii_utf8
// - mod paste
// - mod focus WIP
// - mod mouse
// - mod wheel
// - mod _helper

use super::TermInputParser;
use crate::{EventButton, EventButtonState, EventButtons, EventMouse, EventWheel, EventWheelUnit};
use crate::{EventKey, EventKind, Key, KeyMods, KeyState, pos};
use crate::{TermParsed, TermReply};

mod control {
    use super::*;

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
mod csi_keys_replies {
    use super::*;

    /* keys */
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
    /* replies */
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
    fn csi_with_unexpected_final_byte_is_unknown_at_final_byte() {
        let mut p = TermInputParser::new();
        assert_eq!(p.feed_parsed(0x1b), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'['), TermParsed::Pending);
        assert_eq!(p.feed_parsed(b'<'), TermParsed::Pending);
        // `x` is a valid CSI final-byte position, so the CSI sequence ends here.
        assert_eq!(p.feed_parsed(b'x'), TermParsed::Unknown);
        // Remaining bytes are ground input, not part of the malformed CSI anymore.
        assert_eq!(p.feed_parsed(b';'), TermParsed::Event(TermInputParser::key(Key::Char(';'))));
    }
}
mod ascii_utf8 {
    use super::*;

    /* ascii */
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
    /* utf8 */
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
}
mod paste {
    use super::*;

    #[test]
    fn bracketed_paste_delimiters_toggle_paste_state() {
        let mut p = TermInputParser::new();
        for &b in b"\x1b[200~" {
            assert_eq!(p.feed(b), None);
        }
        assert!(p.is_pasting());
        for &b in b"\x1b[201~" {
            assert_eq!(p.feed(b), None);
        }
        assert!(!p.is_pasting());
    }
    #[test]
    fn bracketed_paste_payload_emits_chars_for_now() {
        let mut p = TermInputParser::new();
        for &b in b"\x1b[200~" {
            let _ = p.feed(b);
        }
        assert!(matches!(
            p.feed(b'a'),
            Some(EventKind::Key(k)) if k.semantic == Key::Char('a')
        ));
        for &b in b"\x1b[201~" {
            let _ = p.feed(b);
        }
        assert!(!p.is_pasting());
    }
}
mod focus {} // TODO
mod mouse {
    use super::*;
    #[test]
    fn sgr_left_press() {
        assert_sgr_mouse(
            b"\x1b[<0;10;20M",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_middle_press() {
        assert_sgr_mouse(
            b"\x1b[<1;10;20M",
            9,
            19,
            Some(EventButton::Middle),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::MIDDLE),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_right_press() {
        assert_sgr_mouse(
            b"\x1b[<2;10;20M",
            9,
            19,
            Some(EventButton::Right),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::RIGHT),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_left_release() {
        assert_sgr_mouse(
            b"\x1b[<0;10;20m",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Released,
            EventButtons::new(),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_release_without_specific_button() {
        assert_sgr_mouse(
            b"\x1b[<3;10;20m",
            9,
            19,
            None,
            EventButtonState::Released,
            EventButtons::new(),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_left_drag_motion() {
        assert_sgr_mouse(
            b"\x1b[<32;10;20M",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Moved,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_middle_drag_motion() {
        assert_sgr_mouse(
            b"\x1b[<33;10;20M",
            9,
            19,
            Some(EventButton::Middle),
            EventButtonState::Moved,
            EventButtons::new().with(EventButtons::MIDDLE),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_right_drag_motion() {
        assert_sgr_mouse(
            b"\x1b[<34;10;20M",
            9,
            19,
            Some(EventButton::Right),
            EventButtonState::Moved,
            EventButtons::new().with(EventButtons::RIGHT),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_plain_motion_without_button() {
        assert_sgr_mouse(
            b"\x1b[<35;10;20M",
            9,
            19,
            None,
            EventButtonState::Moved,
            EventButtons::new(),
            KeyMods::empty(),
        );
    }
    #[test]
    fn sgr_coordinates_are_normalized_to_zero_based() {
        assert_sgr_mouse(
            b"\x1b[<0;1;1M",
            0,
            0,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty(),
        );
    }
    #[test]
    fn malformed_sgr_mouse_is_unknown() {
        let mut p = TermInputParser::new();
        assert_eq!(feed_parsed_seq(&mut p, b"\x1b[<0;10M"), TermParsed::Unknown,);
        let mut p = TermInputParser::new();
        assert_eq!(feed_parsed_seq(&mut p, b"\x1b[<0;10;20;30M"), TermParsed::Unknown,);
    }
    #[test]
    fn sgr_mouse_shift_modifier() {
        assert_sgr_mouse(
            b"\x1b[<4;10;20M",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty().with_shift(),
        );
    }
    #[test]
    fn sgr_mouse_alt_modifier() {
        assert_sgr_mouse(
            b"\x1b[<8;10;20M",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty().with_alt(),
        );
    }
    #[test]
    fn sgr_mouse_control_modifier() {
        assert_sgr_mouse(
            b"\x1b[<16;10;20M",
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty().with_control(),
        );
    }
    #[test]
    fn sgr_mouse_combined_modifiers() {
        assert_sgr_mouse(
            b"\x1b[<28;10;20M", // 4 + 8 + 16
            9,
            19,
            Some(EventButton::Left),
            EventButtonState::Pressed,
            EventButtons::new().with(EventButtons::LEFT),
            KeyMods::empty().with_shift().with_alt().with_control(),
        );
    }
}
mod wheel {
    use super::*;
    #[test]
    fn sgr_wheel_up() {
        assert_sgr_wheel(b"\x1b[<64;10;20M", 0, -1, 9, 19, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_down() {
        assert_sgr_wheel(b"\x1b[<65;10;20M", 0, 1, 9, 19, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_left() {
        assert_sgr_wheel(b"\x1b[<66;10;20M", -1, 0, 9, 19, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_right() {
        assert_sgr_wheel(b"\x1b[<67;10;20M", 1, 0, 9, 19, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_coordinates_are_normalized_to_zero_based() {
        assert_sgr_wheel(b"\x1b[<64;1;1M", 0, -1, 0, 0, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_release_final_is_still_parsed() {
        // Odd but harmless: the wheel family is identified by Cb, not by M/m.
        assert_sgr_wheel(b"\x1b[<65;10;20m", 0, 1, 9, 19, KeyMods::empty());
    }
    #[test]
    fn sgr_wheel_with_shift_modifier() {
        // 64 wheel + 4 shift + 0 up
        assert_sgr_wheel(b"\x1b[<68;10;20M", 0, -1, 9, 19, KeyMods::empty().with_shift());
    }

    #[test]
    fn sgr_wheel_with_control_modifier() {
        // 64 wheel + 16 control + 1 down
        assert_sgr_wheel(b"\x1b[<81;10;20M", 0, 1, 9, 19, KeyMods::empty().with_control());
    }
}

use _helper::*;
mod _helper {
    use super::*;
    pub(super) fn feed_seq(parser: &mut TermInputParser, bytes: &[u8]) -> Option<EventKind> {
        let mut out = None;
        for &b in bytes {
            out = parser.feed(b);
        }
        out
    }
    pub(super) fn key_from(kind: EventKind) -> EventKey {
        match kind {
            EventKind::Key(k) => k,
            other => panic!("expected key event, got {other:?}"),
        }
    }
    pub(super) fn assert_key(bytes: &[u8], expected: Key) {
        let mut p = TermInputParser::new();
        let k = key_from(feed_seq(&mut p, bytes).expect("expected event"));
        assert_eq!(k.semantic, expected);
        assert_eq!(k.physical, expected);
        assert_eq!(k.state, KeyState::Press);
        assert_eq!(k.mods, crate::KeyMods::empty());
    }
    pub(super) fn feed_parsed_seq(parser: &mut TermInputParser, bytes: &[u8]) -> TermParsed {
        let mut out = TermParsed::Pending;
        for &b in bytes {
            out = parser.feed_parsed(b);
        }
        out
    }
    pub(super) fn mouse_from(kind: EventKind) -> EventMouse {
        match kind {
            EventKind::Mouse(m) => m,
            other => panic!("expected mouse event, got {other:?}"),
        }
    }
    pub(super) fn wheel_from(kind: EventKind) -> EventWheel {
        match kind {
            EventKind::Wheel(w) => w,
            other => panic!("expected wheel event, got {other:?}"),
        }
    }
    pub(super) fn assert_sgr_mouse(
        bytes: &[u8],
        x: i32,
        y: i32,
        button: Option<EventButton>,
        state: EventButtonState,
        buttons: EventButtons,
        mods: KeyMods,
    ) {
        let mut p = TermInputParser::new();
        let m = mouse_from(feed_seq(&mut p, bytes).expect("expected mouse event"));
        assert_eq!(m.x, x);
        assert_eq!(m.y, y);
        assert_eq!(m.button, button);
        assert_eq!(m.state, state);
        assert_eq!(m.buttons, buttons);
        assert_eq!(m.mods, mods);
    }
    pub(super) fn assert_sgr_wheel(
        bytes: &[u8],
        delta_x: i32,
        delta_y: i32,
        x: i32,
        y: i32,
        mods: KeyMods,
    ) {
        let mut p = TermInputParser::new();
        let w = wheel_from(feed_seq(&mut p, bytes).expect("expected wheel event"));
        assert_eq!(w.delta_x, delta_x);
        assert_eq!(w.delta_y, delta_y);
        assert_eq!(w.unit, EventWheelUnit::Step);
        assert_eq!(w.x, x);
        assert_eq!(w.y, y);
        assert_eq!(w.buttons, EventButtons::new());
        assert_eq!(w.mods, mods);
    }
}
