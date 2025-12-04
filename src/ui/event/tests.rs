// devela::ui::event::tests

use super::*;
#[cfg(all(feature = "js", not(windows)))]
use crate::WebEventKind;

#[test] #[rustfmt::skip]
fn sizes_of() {
    /* event, kind, window */

    #[cfg(not(feature = "alloc"))] {
    assert_eq![56, size_of::<Event>()];             // 448 bits (+proc+count NonZeroU64)
    assert_eq![36, size_of::<EventKind>()];         // 288 bits
    assert_eq![16, size_of::<EventWindow>()];       // 128 bits
    assert![size_of::<Event>() == size_of::<Option<Event>>()];
    assert![size_of::<EventKind>() == size_of::<Option<EventKind>>()];
    assert![size_of::<EventWindow>() == size_of::<Option<EventWindow>>()];
    }
    #[cfg(feature = "alloc")] {
    assert_eq![56, size_of::<Event>()];             // 448 bits (+proc+count NonZeroU64)
    assert_eq![40, size_of::<EventKind>()];         // 320 bits
    assert_eq![24, size_of::<EventWindow>()];       // 192 bits
    assert![size_of::<Event>() == size_of::<Option<Event>>()];
    assert![size_of::<EventKind>() == size_of::<Option<EventKind>>()];
    assert![size_of::<EventWindow>() == size_of::<Option<EventWindow>>()];
    }


    /* key */

    assert_eq![24, size_of::<EventKey>()];          // 192 bits
    assert![size_of::<EventKey>() == size_of::<Option<EventKey>>()];
    #[cfg(ffi··)] {
    assert_eq![24, size_of::<EventKeyFfi>()];       // 192 bits
    assert![size_of::<EventKeyFfi>() == size_of::<Option<EventKeyFfi>>()];
    }

    assert_eq![8, size_of::<Key>()];                //  64 bits
    assert![size_of::<Key>() == size_of::<Option<Key>>()];
    #[cfg(ffi··)] {
    assert_eq![8, size_of::<KeyFfi>()];             //  64 bits
    assert![size_of::<KeyFfi>() == size_of::<Option<KeyFfi>>()];
    }

    assert_eq![1, size_of::<KeyDead>()];            //   8 bits
    assert_eq![1, size_of::<KeyMedia>()];           //   8 bits
    assert_eq![1, size_of::<KeyMod>()];             //   8 bits
    assert_eq![2, size_of::<KeyMods>()];            //  16 bits
    assert_eq![1, size_of::<KeyPad>()];             //   8 bits
    assert_eq![1, size_of::<KeyState>()];           //   8 bits
    assert![size_of::<KeyDead>() == size_of::<Option<KeyDead>>()];
    assert![size_of::<KeyMedia>() == size_of::<Option<KeyMedia>>()];
    assert![size_of::<KeyMod>() == size_of::<Option<KeyMod>>()];
    assert![size_of::<KeyMods>() != size_of::<Option<KeyMods>>()]; // NOTE !=
    assert![size_of::<KeyPad>() == size_of::<Option<KeyPad>>()];
    assert![size_of::<KeyState>() == size_of::<Option<KeyState>>()];

    /* pointer */

    assert_eq![16, size_of::<EventMouse>()];        // 128
    assert_eq![02, size_of::<EventButton>()];       // 16
    assert_eq![01, size_of::<EventButtonState>()];  // 8
    assert_eq![36, size_of::<EventPointer>()];      // 288
    // assert_eq![40, size_of::<EventPointer>()];      // 320 FUTURE: with phase
    assert_eq![01, size_of::<EventPointerType>()];  // 8
    // assert_eq![01, size_of::<EventPointerPhase>()]; // 8 // FUTURE
    assert_eq![20, size_of::<EventWheel>()];        // 160
    assert![size_of::<EventMouse>() == size_of::<Option<EventMouse>>()];
    assert![size_of::<EventButton>() == size_of::<Option<EventButton>>()];
    assert![size_of::<EventButtonState>() == size_of::<Option<EventButtonState>>()];
    assert![size_of::<EventPointer>() == size_of::<Option<EventPointer>>()];
    assert![size_of::<EventPointerType>() == size_of::<Option<EventPointerType>>()];
    assert![size_of::<EventWheel>() != size_of::<Option<EventWheel>>()]; // NOTE !=

    /* timestamp */

    assert_eq![4, size_of::<EventTimestamp>()]; // 32
    assert![size_of::<EventTimestamp>() == size_of::<Option<EventTimestamp>>()];
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
