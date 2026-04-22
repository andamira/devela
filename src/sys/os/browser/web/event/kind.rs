// devela::sys::os::browser::web::event::kind
//
//! Defines [`WebEventKind`].
//

use crate::{KeyState, is};

#[doc = crate::_tags!(event web uid)]
/// A typed selector for web event names used by the web API browser bridge.
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// # Role
/// - Selects the exact web event name passed to `addEventListener`.
/// - Bridges raw browser events into web API callbacks and normalized events.
/// - Keeps the web adapter typed without leaking stringly-typed event names into user code.
///
/// ---
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
// NOTE: In sync with web_api.js::get_event_kind()
#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum WebEventKind {
    /// Unknown event. Default case.
    #[default]
    Unknown = 0,

    /// Fires when an element is clicked.
    Click = 1,

    /* keyboard */
    /// Fires when a key is pressed down.
    KeyDown = 2,
    /// Fires when a key is released.
    KeyUp = 3,
    // no KeyPress (deprecated)

    /* mouse */
    /// Fires when the mouse button is pressed down.
    MouseDown = 4,
    /// Fires when the mouse button is released.
    MouseUp = 5,
    /// Fires when the mouse moves over an element.
    MouseMove = 6,

    /* pointer */
    /// Fires when the pointer is pressed down.
    PointerDown = 7,
    /// Fires when the pointer is released.
    PointerUp = 8,
    /// Fires when the pointer is moved.
    PointerMove = 9,

    /// Fires when the wheel is moved.
    Wheel = 10,

    ///
    GamepadPoll = 11,

    /// Fires when the window is resized.
    Resize = 12,
}
impl WebEventKind {
    /// Constructs a `WebEventKind` from its representation.
    pub const fn from_repr(from: u8) -> Self {
        use WebEventKind as E;
        match from {
            1 => E::Click,
            2 => E::KeyDown,
            3 => E::KeyUp,
            4 => E::MouseDown,
            5 => E::MouseUp,
            6 => E::MouseMove,
            7 => E::PointerDown,
            8 => E::PointerUp,
            9 => E::PointerMove,
            10 => E::Wheel,
            11 => E::GamepadPoll,
            12 => E::Resize,
            _ => E::Unknown,
        }
    }
    /// Returns the event name as a string.
    pub const fn as_str(self) -> &'static str {
        use WebEventKind as E;
        match self {
            E::Click => "click",
            E::KeyDown => "keydown",
            E::KeyUp => "keyup",
            E::MouseDown => "mousedown",
            E::MouseUp => "mouseup",
            E::MouseMove => "mousemove",
            E::PointerDown => "pointerdown",
            E::PointerUp => "pointerup",
            E::PointerMove => "pointermove",
            E::Wheel => "wheel",
            E::GamepadPoll => "gamepadpoll",
            E::Resize => "resize",
            E::Unknown => "none",
        }
    }
}

#[rustfmt::skip]
impl WebEventKind {
    /// Converts a `WebEventKind` to `KeyState`, if applicable.
    // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat
    #[must_use]
    pub const fn to_key_state(self, repeat: bool) -> Option<KeyState> {
        match self {
            WebEventKind::KeyDown => Some(is![repeat, KeyState::Repeat, KeyState::Press]),
            WebEventKind::KeyUp => Some(KeyState::Release),
            _ => None,
        }
    }
    /// Converts a `KeyState` to `WebEventKind`.
    #[must_use]
    pub const fn from_key_state(from: KeyState) -> WebEventKind {
        match from {
            KeyState::Press => WebEventKind::KeyDown,
            KeyState::Release => WebEventKind::KeyUp,
            KeyState::Repeat => WebEventKind::KeyDown,
        }
    }
}
