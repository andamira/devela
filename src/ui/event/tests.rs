// devela::ui::event::tests

use super::*;
#[cfg(all(feature = "js", not(windows)))]
use crate::WebEventKind;

#[test] #[rustfmt::skip]
fn sizes_of() {
    #[cfg(not(feature = "alloc"))] {
    // assert_eq![40, size_of::<Event>()];             // 320 bits (old)
    // assert_eq![44, size_of::<Event>()];             // 352 bits (+processed)
    // assert_eq![64, size_of::<Event>()];             // 512 bits (+proc+count u64)
    assert_eq![56, size_of::<Event>()];             // 448 bits (+proc+count NonZeroU64)
    assert_eq![36, size_of::<EventKind>()];         // 288 bits
    }
    #[cfg(feature = "alloc")] {
    // assert_eq![48, size_of::<Event>()];             // 384 bits (old)
    assert_eq![56, size_of::<Event>()];             // 448 bits (+proc+count NonZeroU64)
    assert_eq![40, size_of::<EventKind>()];         // 320 bits
    }

    /* key */

    assert_eq![24, size_of::<EventKey>()];          // 192 bits
    #[cfg(ffi··)]
    assert_eq![24, size_of::<EventKeyFfi>()];       // 192 bits

    assert_eq![ 8, size_of::<Key>()];               //  64 bits
    #[cfg(ffi··)]
    assert_eq![ 8, size_of::<KeyFfi>()];            //  64 bits

    assert_eq![ 1, size_of::<KeyMedia>()];          //   8 bits
    assert_eq![ 1, size_of::<KeyMod>()];            //   8 bits
    assert_eq![ 2, size_of::<KeyMods>()];           //  16 bits
    assert_eq![ 1, size_of::<KeyPad>()];            //   8 bits
    assert_eq![ 1, size_of::<KeyState>()];          //   8 bits

    /* pointer */

    assert_eq![16, size_of::<EventMouse>()];        // 128
    assert_eq![02, size_of::<EventButton>()];       // 16
    assert_eq![01, size_of::<EventButtonState>()];  // 8
    assert_eq![36, size_of::<EventPointer>()];      // 288
    // assert_eq![40, size_of::<EventPointer>()];      // 320 (with phase)
    assert_eq![01, size_of::<EventPointerType>()];  // 8
    // assert_eq![01, size_of::<EventPointerPhase>()]; // 8
    assert_eq![20, size_of::<EventWheel>()];        // 160

    // niche
    assert![size_of::<EventMouse>() == size_of::<Option<EventMouse>>()];
    assert![size_of::<EventButton>() == size_of::<Option<EventButton>>()];
    assert![size_of::<EventButtonState>() == size_of::<Option<EventButtonState>>()];
    assert![size_of::<EventPointer>() == size_of::<Option<EventPointer>>()];
    assert![size_of::<EventPointerType>() == size_of::<Option<EventPointerType>>()];
    // assert![size_of::<EventWheel>() == size_of::<Option<EventWheel>>()]; // not equal
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
    assert_eq!(KeyState::from_js(WebEventKind::KeyDown, false), Some(KeyState::Press));
    assert_eq!(KeyState::from_js(WebEventKind::KeyUp, false), Some(KeyState::Release));

    assert_eq!(KeyState::from_js(WebEventKind::KeyDown, true), Some(KeyState::Repeat));
    assert_eq!(KeyState::from_js(WebEventKind::KeyUp, true), Some(KeyState::Release));

    assert_eq!(KeyState::from_js(WebEventKind::Click, false), None);
    assert_eq!(KeyState::from_js(WebEventKind::Click, true), None);
}
