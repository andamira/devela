// devela::sys::os::browser::web::event

use crate::{KeyFfi, KeyPad, KeyState};
use crate::{WebEventKind, WebEventMouse, WebEventPointer, WebKeyLocation};

#[test] #[rustfmt::skip]
fn sizes_of() {
    assert_eq![ 4, size_of::<WebEventKind>()];    // 32
    assert_eq![32, size_of::<WebEventMouse>()];   // 256
    assert_eq![48, size_of::<WebEventPointer>()]; // 384
    assert_eq![ 1, size_of::<WebKeyLocation>()];  // 8
}
#[test]
fn js_event_conversions() {
    assert_eq!(WebEventKind::from_repr(02), WebEventKind::KeyDown);
    assert_eq!(WebEventKind::from_repr(03), WebEventKind::KeyUp);
    assert_eq!(WebEventKind::from_repr(99), WebEventKind::Unknown);
}
#[test]
fn key_state_to_js_event() {
    assert_eq!(KeyState::Press.to_js(), WebEventKind::KeyDown);
    assert_eq!(KeyState::Release.to_js(), WebEventKind::KeyUp);
}
#[test]
fn js_event_to_key_state() {
    assert_eq!(KeyState::from_js(WebEventKind::KeyDown, false), Some(KeyState::Press));
    assert_eq!(KeyState::from_js(WebEventKind::KeyUp, false), Some(KeyState::Release));

    assert_eq!(KeyState::from_js(WebEventKind::KeyDown, true), Some(KeyState::Repeat));
    assert_eq!(KeyState::from_js(WebEventKind::KeyUp, true), Some(KeyState::Release));

    assert_eq!(KeyState::from_js(WebEventKind::Click, false), None);
    assert_eq!(KeyState::from_js(WebEventKind::Click, true), None);
}

#[test]
fn key_ffi_to_js_code() {
    assert_eq!(KeyFfi::Enter.to_js_code(), ("Enter", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Space.to_js_code(), ("Space", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Fn(5).to_js_code(), ("F5", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Pad(KeyPad::Num0).to_js_code(), ("Numpad0", WebKeyLocation::NumPad));
    assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("Unknown", WebKeyLocation::Standard));
}
#[test]
fn key_ffi_from_js_code() {
    assert_eq!(KeyFfi::from_js_code("Enter", WebKeyLocation::Standard), Some(KeyFfi::Enter));
    assert_eq!(KeyFfi::from_js_code("F5", WebKeyLocation::Standard), Some(KeyFfi::Fn(5)));
    assert_eq!(
        KeyFfi::from_js_code("Numpad0", WebKeyLocation::NumPad),
        Some(KeyFfi::Pad(KeyPad::Num0))
    );
    assert_eq!(KeyFfi::from_js_code("Unknown", WebKeyLocation::Standard), None);
}
/* semantic */
#[test]
fn key_ffi_to_js_key() {
    assert_eq!(KeyFfi::Enter.to_js_key(), ("Enter", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Space.to_js_key(), (" ", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Char('a' as u32).to_js_key(), ("a", WebKeyLocation::Standard));
    assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("Unknown", WebKeyLocation::Standard));
    // assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("é", WebKeyLocation::Standard)); // IMPROVE
}
#[test]
fn key_ffi_from_js_key() {
    assert_eq!(KeyFfi::from_js_key("Enter", WebKeyLocation::Standard), Some(KeyFfi::Enter));
    assert_eq!(KeyFfi::from_js_key(" ", WebKeyLocation::Standard), Some(KeyFfi::Space));
    assert_eq!(KeyFfi::from_js_key("a", WebKeyLocation::Standard), Some(KeyFfi::Char('a' as u32)));
    assert_eq!(KeyFfi::from_js_key("é", WebKeyLocation::Standard), Some(KeyFfi::Char('é' as u32)));
}
