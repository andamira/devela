// devela::sys::os::term::event::input::tests

use super::TermInputParser;
use crate::{EventKey, EventKind, Key, KeyState, pos};
use crate::{TermParsed, TermReply};

/* helpers */

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

/* tests */

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
