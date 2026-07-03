// devela/src/ui/text/_test.rs
//
// TOC
// - misc
// - alloc
// - event

use crate::TextInput;
use crate::{TextInputAction as A, TextInputConfig, TextInputOutcome as O, TextInputReject as R};

#[test]
fn inline_storage_edits() {
    let mut input = TextInput::<[u8; 8]>::new();
    assert_eq!(input.apply(A::Insert('a')), O::Changed);
    assert_eq!(input.apply(A::Insert('ñ')), O::Changed);
    assert_eq!(input.as_str(), "añ");
    assert_eq!(input.cursor(), 3);
    assert_eq!(input.apply(A::MoveLeft), O::Changed);
    assert_eq!(input.apply(A::Insert('b')), O::Changed);
    assert_eq!(input.as_str(), "abñ");
    assert_eq!(input.apply(A::Backspace), O::Changed);
    assert_eq!(input.as_str(), "añ");
}
#[test]
fn borrowed_storage_edits() {
    let mut buf = [0u8; 8];
    let mut input = TextInput::from_buf(&mut buf);
    assert_eq!(input.apply(A::Insert('x')), O::Changed);
    assert_eq!(input.apply(A::Insert('€')), O::Changed);
    assert_eq!(input.as_str(), "x€");
    assert_eq!(input.apply(A::MoveStart), O::Changed);
    assert_eq!(input.apply(A::Delete), O::Changed);
    assert_eq!(input.as_str(), "€");
}
#[test]
fn configured_max_bytes_rejects() {
    let config = TextInputConfig { max_bytes: Some(2), can_be_empty: true };
    let mut input = TextInput::from_storage_config([0u8; 8], config);
    assert_eq!(input.apply(A::Insert('ñ')), O::Changed);
    assert!(matches!(input.apply(A::Insert('a')), O::Rejected(_)));
    assert_eq!(input.as_str(), "ñ");
}
#[test]
fn utf8_cursor_movement_uses_char_boundaries() {
    let mut input = TextInput::<[u8; 8]>::new();
    assert_eq!(input.apply(A::Insert('a')), O::Changed);
    assert_eq!(input.apply(A::Insert('€')), O::Changed);
    assert_eq!(input.apply(A::Insert('ñ')), O::Changed);
    assert_eq!(input.as_str(), "a€ñ");
    assert_eq!(input.cursor(), 6);
    assert_eq!(input.apply(A::MoveLeft), O::Changed);
    assert_eq!(input.cursor(), 4);
    assert_eq!(input.apply(A::MoveLeft), O::Changed);
    assert_eq!(input.cursor(), 1);
    assert_eq!(input.apply(A::MoveLeft), O::Changed);
    assert_eq!(input.cursor(), 0);
    assert_eq!(input.apply(A::MoveLeft), O::Unchanged);
    assert_eq!(input.cursor(), 0);
    assert_eq!(input.apply(A::MoveRight), O::Changed);
    assert_eq!(input.cursor(), 1);
    assert_eq!(input.apply(A::MoveRight), O::Changed);
    assert_eq!(input.cursor(), 4);
    assert_eq!(input.apply(A::MoveRight), O::Changed);
    assert_eq!(input.cursor(), 6);
    assert_eq!(input.apply(A::MoveRight), O::Unchanged);
}
#[test]
fn delete_removes_multibyte_char_at_cursor() {
    let mut input = TextInput::<[u8; 8]>::new();
    input.apply(A::Insert('a'));
    input.apply(A::Insert('€'));
    input.apply(A::Insert('b'));
    assert_eq!(input.as_str(), "a€b");
    assert_eq!(input.apply(A::MoveStart), O::Changed);
    assert_eq!(input.apply(A::MoveRight), O::Changed);
    assert_eq!(input.cursor(), 1);
    assert_eq!(input.apply(A::Delete), O::Changed);
    assert_eq!(input.as_str(), "ab");
    assert_eq!(input.cursor(), 1);
}
#[test]
fn backspace_removes_multibyte_char_before_cursor() {
    let mut input = TextInput::<[u8; 8]>::new();
    input.apply(A::Insert('a'));
    input.apply(A::Insert('€'));
    input.apply(A::Insert('b'));
    assert_eq!(input.as_str(), "a€b");
    assert_eq!(input.apply(A::MoveLeft), O::Changed);
    assert_eq!(input.cursor(), 4);
    assert_eq!(input.apply(A::Backspace), O::Changed);
    assert_eq!(input.as_str(), "ab");
    assert_eq!(input.cursor(), 1);
}
#[test]
fn rejects_when_storage_is_full() {
    let mut input = TextInput::<[u8; 3]>::new();
    assert_eq!(input.apply(A::Insert('€')), O::Changed);
    assert_eq!(input.as_str(), "€");
    assert_eq!(input.cursor(), 3);
    assert_eq!(input.remaining_capacity(), 0);
    assert_eq!(input.apply(A::Insert('a')), O::Rejected(R::Full));
    assert_eq!(input.as_str(), "€");
    assert_eq!(input.cursor(), 3);
}
#[test]
fn accept_empty_respects_config() {
    let config = TextInputConfig { max_bytes: None, can_be_empty: false };
    let mut input = TextInput::from_storage_config([0u8; 8], config);
    assert_eq!(input.apply(A::Accept), O::Rejected(R::Empty));
    assert_eq!(input.apply(A::Insert('a')), O::Changed);
    assert_eq!(input.apply(A::Accept), O::Accepted);
}
#[test]
fn clear_resets_text_and_cursor() {
    let mut input = TextInput::<[u8; 8]>::new();
    input.apply(A::Insert('a'));
    input.apply(A::Insert('€'));
    assert_eq!(input.as_str(), "a€");
    assert_eq!(input.cursor(), 4);
    assert_eq!(input.apply(A::Clear), O::Changed);
    assert_eq!(input.as_str(), "");
    assert_eq!(input.cursor(), 0);
    assert!(input.is_empty());
}
#[test]
fn try_set_cursor_accepts_only_utf8_boundaries() {
    let mut input = TextInput::<[u8; 8]>::new();
    input.apply(A::Insert('a'));
    input.apply(A::Insert('€'));
    assert_eq!(input.as_str(), "a€");
    assert!(input.try_set_cursor(0));
    assert_eq!(input.cursor(), 0);
    assert!(input.try_set_cursor(1));
    assert_eq!(input.cursor(), 1);
    assert!(!input.try_set_cursor(2));
    assert_eq!(input.cursor(), 1);
    assert!(!input.try_set_cursor(3));
    assert_eq!(input.cursor(), 1);
    assert!(input.try_set_cursor(4));
    assert_eq!(input.cursor(), 4);
}
#[test]
fn invalid_cursor_rejects_cursor_sensitive_actions() {
    let mut input = TextInput::<[u8; 8]>::new();
    input.apply(A::Insert('€'));
    assert_eq!(input.as_str(), "€");
    input.cursor = 1; // inside the 3-byte scalar
    assert_eq!(input.apply(A::Insert('a')), O::Rejected(R::InvalidCursor));
    assert_eq!(input.apply(A::Backspace), O::Rejected(R::InvalidCursor));
    assert_eq!(input.apply(A::Delete), O::Rejected(R::InvalidCursor));
    assert_eq!(input.apply(A::MoveLeft), O::Rejected(R::InvalidCursor));
    assert_eq!(input.apply(A::MoveRight), O::Rejected(R::InvalidCursor));
    assert_eq!(input.apply(A::Accept), O::Rejected(R::InvalidCursor));
    assert_eq!(input.as_str(), "€");
    assert_eq!(input.cursor(), 1);
}
#[test]
fn borrowed_storage_keeps_written_bytes() {
    let mut buf = [0u8; 8];
    {
        let mut input = TextInput::from_buf(&mut buf);
        input.apply(A::Insert('x'));
        input.apply(A::Insert('€'));
        assert_eq!(input.as_str(), "x€");
        assert_eq!(input.as_bytes(), "x€".as_bytes());
        input.apply(A::MoveStart);
        input.apply(A::Delete);
        assert_eq!(input.as_str(), "€");
    }
    assert_eq!(&buf[..4], &[0xE2, 0x82, 0xAC, 0]);
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;

    #[test]
    fn alloc_storage_edits() {
        use crate::String;
        let mut input = TextInput::from_string(String::from("ab"));
        assert_eq!(input.apply(A::MoveLeft), O::Changed);
        assert_eq!(input.apply(A::Insert('€')), O::Changed);
        assert_eq!(input.as_str(), "a€b");
        assert_eq!(input.apply(A::Delete), O::Changed);
        assert_eq!(input.as_str(), "a€");
    }
    #[test]
    fn alloc_try_set_cursor_accepts_only_utf8_boundaries() {
        use crate::String;
        let mut input = TextInput::from_string(String::from("a€"));
        assert!(input.try_set_cursor(1));
        assert_eq!(input.cursor(), 1);
        assert!(!input.try_set_cursor(2));
        assert_eq!(input.cursor(), 1);
        assert!(input.try_set_cursor(4));
        assert_eq!(input.cursor(), 4);
    }
}

#[cfg(feature = "event")]
mod event {
    use super::*;
    use crate::{EventKey as E, Key, KeyMods, KeyState, TextInputKeymap};

    #[test]
    fn default_keymap_maps_plain_text() {
        assert_eq!(A::from_key_event(&E::text('x')), Some(A::Insert('x')));
        assert_eq!(A::from_key_event(&E::press(Key::Space)), Some(A::Insert(' ')));
    }
    #[test]
    fn default_keymap_maps_basic_editing_keys() {
        assert_eq!(A::from_key_event(&E::press(Key::Backspace)), Some(A::Backspace));
        assert_eq!(A::from_key_event(&E::press(Key::Delete)), Some(A::Delete));
        //
        assert_eq!(A::from_key_event(&E::press(Key::Left)), Some(A::MoveLeft));
        assert_eq!(A::from_key_event(&E::press(Key::Right)), Some(A::MoveRight));
        assert_eq!(A::from_key_event(&E::press(Key::Home)), Some(A::MoveStart));
        assert_eq!(A::from_key_event(&E::press(Key::End)), Some(A::MoveEnd));
        //
        assert_eq!(A::from_key_event(&E::press(Key::Enter)), Some(A::Accept));
        assert_eq!(A::from_key_event(&E::press(Key::Escape)), Some(A::Cancel));
    }
    #[test]
    fn default_keymap_ignores_key_release() {
        let ev = E::press(Key::Left).with_state(KeyState::Release);
        assert_eq!(A::from_key_event(&ev), None);
    }
    #[test]
    fn default_keymap_does_not_insert_control_text() {
        let mut ctrl = KeyMods::new();
        ctrl.set_control();
        assert_eq!(A::from_key_event(&E::modified_text('a', ctrl)), None);
        assert_eq!(A::from_key_event(&E::modified_text('c', ctrl)), Some(A::Cancel));
    }
    #[test]
    fn emacs_keymap_maps_basic_shortcuts() {
        let mut ctrl = KeyMods::new();
        ctrl.set_control();
        let map = TextInputKeymap::EMACS;
        assert_eq!(A::from_key_event_with(&E::modified_text('a', ctrl), map), Some(A::MoveStart));
        assert_eq!(A::from_key_event_with(&E::modified_text('e', ctrl), map), Some(A::MoveEnd));
        assert_eq!(A::from_key_event_with(&E::modified_text('b', ctrl), map), Some(A::MoveLeft));
        assert_eq!(A::from_key_event_with(&E::modified_text('f', ctrl), map), Some(A::MoveRight));
        assert_eq!(A::from_key_event_with(&E::modified_text('d', ctrl), map), Some(A::Delete));
        assert_eq!(A::from_key_event_with(&E::modified_text('h', ctrl), map), Some(A::Backspace));
        assert_eq!(A::from_key_event_with(&E::modified_text('g', ctrl), map), Some(A::Cancel));
    }
    #[test]
    fn mapped_key_event_edits_text_input() {
        let mut input = TextInput::<[u8; 8]>::new();
        let action = A::from_key_event(&E::text('a')).unwrap();
        assert_eq!(input.apply(action), O::Changed);
        assert_eq!(input.as_str(), "a");
        let action = A::from_key_event(&E::press(Key::Backspace)).unwrap();
        assert_eq!(input.apply(action), O::Changed);
        assert_eq!(input.as_str(), "");
    }
}
