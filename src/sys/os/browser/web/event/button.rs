// devela/src/sys/os/browser/web/event/button.rs
//
//! Implement conversions from/into web event types.
//

use crate::{EventButton, EventButtonState, WebEventKind};

impl EventButton {
    /// Converts a DOM-compatible `button` value into a normalized button slot.
    ///
    /// This preserves the reported button slot. It does not assign contextual
    /// roles such as "back", "forward", "barrel", or "eraser".
    pub const fn from_web(js_button: u8) -> Option<Self> {
        match js_button {
            0 => Some(EventButton::Left),
            1 => Some(EventButton::Middle),
            2 => Some(EventButton::Right),
            3 => Some(EventButton::X1),
            4 => Some(EventButton::X2),
            //
            5 => Some(EventButton::X3),
            6 => Some(EventButton::X4),
            7 => Some(EventButton::X5),
            255 => None, // (== -1_i8) represents "no button"
            n => Some(EventButton::Other(n)),
        }
    }
    /// Converts a normalized button slot into a DOM-compatible button value.
    pub const fn to_web(self) -> u8 {
        match self {
            EventButton::Left => 0,
            EventButton::Middle => 1,
            EventButton::Right => 2,
            EventButton::X1 => 3,
            EventButton::X2 => 4,
            //
            EventButton::X3 => 5,
            EventButton::X4 => 6,
            EventButton::X5 => 7,
            EventButton::Other(n) => n,
        }
    }
}

// IMPROVE: MAYBE impl try_ methods
impl EventButtonState {
    /// Converts a `WebEventKind` into `EventButtonState`.
    pub const fn from_web(js_event: WebEventKind) -> Self {
        use {EventButtonState as E, WebEventKind as J};
        match js_event {
            J::Click | J::MouseDown | J::PointerDown => E::Pressed,
            J::MouseUp | J::PointerUp => E::Released,
            _ => E::Moved,
        }
    }
    /// Converts a `EventButtonState` into a `WebEventKind`.
    pub const fn to_web_as_mouse(self) -> WebEventKind {
        use {EventButtonState as E, WebEventKind as J};
        match self {
            E::Pressed => J::MouseDown,
            E::Released => J::MouseUp,
            E::Moved => J::MouseMove,
        }
    }
    /// Converts a `EventButtonState` into a `WebEventKind`.
    pub const fn to_web_as_pointer(self) -> WebEventKind {
        use {EventButtonState as E, WebEventKind as J};
        match self {
            E::Pressed => J::PointerDown,
            E::Released => J::PointerUp,
            E::Moved => J::PointerMove,
        }
    }
}
