// devela::ui::event::key::tests
//

use super::*;
#[cfg(all(feature = "js", not(windows)))]
use crate::WebEventKind;

#[test] #[rustfmt::skip]
fn sizes_of() {
    assert_eq![24, size_of::<EventKey>()];        // 192 bits
    assert_eq![ 8, size_of::<Key>()];             //  64 bits
    #[cfg(ffi··)] crate::items! {
        assert_eq![24, size_of::<EventKeyFfi>()]; // 192 bits
        assert_eq![ 8, size_of::<KeyFfi>()];      //  64 bits
    }
    assert_eq![ 1, size_of::<KeyAlpha>()];        //   8 bits
    assert_eq![ 1, size_of::<KeyMedia>()];        //   8 bits
    assert_eq![ 1, size_of::<KeyMod>()];          //   8 bits
    assert_eq![ 2, size_of::<KeyMods>()];         //  16 bits
    assert_eq![ 1, size_of::<KeyPad>()];          //   8 bits
    assert_eq![ 1, size_of::<KeyState>()];        //   8 bits
}

#[test]
#[cfg(all(feature = "js", not(windows)))]
fn key_state_to_js_event() {
    assert_eq!(KeyState::Press.to_js(), WebEventKind::KeyDown);
    assert_eq!(KeyState::Release.to_js(), WebEventKind::KeyUp);
}
#[test]
#[cfg(all(feature = "js", not(windows)))]
fn js_event_to_key_state() {
    assert_eq!(KeyState::from_js(WebEventKind::KeyDown), Some(KeyState::Press));
    assert_eq!(KeyState::from_js(WebEventKind::KeyUp), Some(KeyState::Release));
    assert_eq!(KeyState::from_js(WebEventKind::Click), None);
}
