// devela/src/sys/os/browser/web/event/tests.rs

use crate::{Key, KeyMods, KeyPad, KeyState};
use crate::{WebEventKind, WebKeyLocation};

#[test]
fn js_event_conversions() {
    assert_eq!(WebEventKind::from_repr(02), WebEventKind::KeyDown);
    assert_eq!(WebEventKind::from_repr(03), WebEventKind::KeyUp);
    assert_eq!(WebEventKind::from_repr(99), WebEventKind::Unknown);
}
#[test]
fn key_state_to_web_event() {
    assert_eq!(WebEventKind::from_key_state(KeyState::Press), WebEventKind::KeyDown);
    assert_eq!(WebEventKind::from_key_state(KeyState::Release), WebEventKind::KeyUp);
}
#[test]
fn js_event_to_key_state() {
    assert_eq!(WebEventKind::KeyDown.to_key_state(false), Some(KeyState::Press));
    assert_eq!(WebEventKind::KeyUp.to_key_state(false), Some(KeyState::Release));

    assert_eq!(WebEventKind::KeyDown.to_key_state(true), Some(KeyState::Repeat));
    assert_eq!(WebEventKind::KeyUp.to_key_state(true), Some(KeyState::Release));

    assert_eq!(WebEventKind::Click.to_key_state(false), None);
    assert_eq!(WebEventKind::Click.to_key_state(true), None);
}

#[test]
fn key_to_web_code() {
    assert_eq!(Key::Enter.to_web_code(), ("Enter", WebKeyLocation::Standard));
    assert_eq!(Key::Space.to_web_code(), ("Space", WebKeyLocation::Standard));
    assert_eq!(Key::Fn(5).to_web_code(), ("F5", WebKeyLocation::Standard));
    assert_eq!(Key::Pad(KeyPad::Num0).to_web_code(), ("Numpad0", WebKeyLocation::NumPad));
    assert_eq!(Key::Char('é').to_web_key(), ("Unknown", WebKeyLocation::Standard));
}
#[test]
fn key_from_web_code() {
    assert_eq!(Key::from_web_code("Enter", WebKeyLocation::Standard), Some(Key::Enter));
    assert_eq!(Key::from_web_code("F5", WebKeyLocation::Standard), Some(Key::Fn(5)));
    assert_eq!(Key::from_web_code("Numpad0", WebKeyLocation::NumPad), Some(Key::Pad(KeyPad::Num0)));
    assert_eq!(Key::from_web_code("Unknown", WebKeyLocation::Standard), None);
}
/* semantic */
#[test]
fn key_to_web_key() {
    assert_eq!(Key::Enter.to_web_key(), ("Enter", WebKeyLocation::Standard));
    assert_eq!(Key::Space.to_web_key(), (" ", WebKeyLocation::Standard));
    assert_eq!(Key::Char('a').to_web_key(), ("a", WebKeyLocation::Standard));
    assert_eq!(Key::Char('é').to_web_key(), ("Unknown", WebKeyLocation::Standard));
    // assert_eq!(Key::Char('é' as u32).to_web_key(), ("é", WebKeyLocation::Standard)); // IMPROVE
}
#[test]
fn key_from_web_key() {
    assert_eq!(Key::from_web_key("Enter", WebKeyLocation::Standard), Some(Key::Enter));
    assert_eq!(Key::from_web_key(" ", WebKeyLocation::Standard), Some(Key::Space));
    assert_eq!(Key::from_web_key("a", WebKeyLocation::Standard), Some(Key::Char('a')));
    assert_eq!(Key::from_web_key("é", WebKeyLocation::Standard), Some(Key::Char('é')));
}

#[test]
fn keymods_web_roundtrip_low_byte() {
    let mods = KeyMods::from_web(0b1111_1111);
    assert_eq!(mods.to_web(), 0b1111_1111);
    assert!(mods.has_control());
    assert!(mods.has_shift());
    assert!(mods.has_alt());
    assert!(mods.has_super());
    assert!(mods.has_alt_gr());
    assert!(mods.has_caps_lock());
    assert!(mods.has_num_lock());
    assert!(mods.has_scroll_lock());
}
#[test]
fn keymods_web_drops_non_web_flags() {
    let mut mods = KeyMods::new();
    mods.set_repeating();
    mods.set_composing();
    assert_eq!(mods.to_web(), 0);
}
