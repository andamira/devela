// devela::sys::os::browser::web::event::pointer
//
//! Defines [`WebEventMouse`], [`WebEventPointer`].
//

use crate::lang::prog::ffi::js::{JsInstant, js_int32, js_number};
use crate::sys::os::browser::WebEventKind;

#[doc = crate::_tags!(event web)]
/// A web API Mouse Event.
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// Represents a JavaScript mouse event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent>
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventMouse {
    /// The X-coordinate of the mouse event relative to the viewport.
    pub x: js_number,
    /// The Y-coordinate of the mouse event relative to the viewport.
    pub y: js_number,
    /// The mouse button that triggered the event (`0`: left, `1`: middle, `2`: right).
    ///
    /// If the event was a movement without a button click, this is `-1` (255)
    pub button: u8,
    /// A bitmask of buttons currently being held down (`1`: left, `2`: right, `4`: middle).
    pub buttons: u8,
    /// The type of mouse event (Click, MouseDown, MouseMove, etc.).
    pub etype: WebEventKind,
    /// The JavaScript event timestamp.
    pub timestamp: JsInstant,
}
impl WebEventMouse {
    pub(crate) fn new(
        x: js_number,
        y: js_number,
        button: u8,
        buttons: u8,
        etype: WebEventKind,
        timestamp: JsInstant,
    ) -> Self {
        Self { x, y, button, buttons, etype, timestamp }
    }
}

#[doc = crate::_tags!(event web)]
/// A web API Pointer Event.
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// Represents a JavaScript pointer event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent>
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventPointer {
    /// The X-coordinate of the pointer event relative to the viewport.
    pub x: js_number,
    /// The Y-coordinate of the pointer event relative to the viewport.
    pub y: js_number,
    /// The pressure applied to the pointer (0.0 to 1.0 for most devices).
    pub pressure: js_number,
    /// Unique identifier for the pointer device.
    pub id: js_int32,
    /// The tilt of the stylus along the X-axis (-90° to 90°).
    pub tilt_x: i8,
    /// The tilt of the stylus along the Y-axis (-90° to 90°).
    pub tilt_y: i8,
    /// The rotation of the stylus around its own axis (0° to 359°).
    pub twist: u16,
    /// The type of pointer event (PointerDown, PointerMove, etc.).
    pub etype: WebEventKind,
    /// The JavaScript event timestamp.
    pub timestamp: JsInstant,
}
impl WebEventPointer {
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub(crate) fn new(
        x: js_number,
        y: js_number,
        pressure: js_number,
        id: js_int32,
        tilt_x: i8,
        tilt_y: i8,
        twist: u16,
        etype: WebEventKind,
        timestamp: JsInstant,
    ) -> Self {
        Self { x, y, pressure, id, tilt_x, tilt_y, twist, etype, timestamp }
    }
}

#[rustfmt::skip]
mod impls {
    use crate::{is, NonZeroU8};
    use crate::{js_number, WebEventKind, WebEventMouse, JsInstant};
    use crate::ui::{EventButton, EventButtons, EventButtonState, EventMouse, EventTimestamp};

    /* mouse */

    impl EventMouse {
        /// Converts `WebEventMouse` to `EventMouse`.
        pub const fn from_js(js: WebEventMouse) -> EventMouse {
            EventMouse {
                x: js.x as i32,
                y: js.y as i32,
                button: EventButton::from_js(js.button),
                state: EventButtonState::from_js(js.etype),
                buttons: EventButtons::with_bits(js.buttons),
                timestamp: Some(EventTimestamp::from_js(js.timestamp)),
            }
        }
    }

    impl EventMouse {
        /// Converts `EventMouse` to `WebEventMouse`.
        pub const fn to_js(self) -> WebEventMouse {
            WebEventMouse {
                x: self.x as js_number,
                y: self.y as js_number,
                button: is![let Some(b) = self.button, b.to_js(), 255], // IMPROVE to_js
                buttons: self.buttons.bits(), // already a bitmask, directly compatible
                etype: self.state.to_js_as_mouse(),
                timestamp: is![let Some(t) = self.timestamp, t.to_js(), JsInstant { ms: 0.0 }],
            }
        }
    }

    impl From<WebEventMouse> for EventMouse {
        fn from(from: WebEventMouse) -> Self { Self::from_js(from) }
    }
    impl From<EventMouse> for WebEventMouse {
        fn from(from: EventMouse) -> Self { from.to_js() }
    }

    /* button */

    impl EventButton {
        /// Converts a JavaScript button index in [`WebEventMouse`] into `EventButton`.
        pub const fn from_js(js_button: u8) -> Option<Self> {
            match js_button {
                0 => Some(EventButton::Left),
                1 => Some(EventButton::Middle),
                2 => Some(EventButton::Right),
                255 => None, // (== -1_i8) represents "no button"
                n => Some(EventButton::Other(NonZeroU8::new(n).unwrap())),
            }
        }
        /// Converts an EventButton to a JavaScript button index in [`WebEventMouse`].
        pub const fn to_js(self) -> u8 {
            match self {
                EventButton::Left => 0,
                EventButton::Right => 1,
                EventButton::Middle => 2,
                EventButton::Other(n) => n.get(),
            }
        }
    }

    // IMPROVE: MAYBE impl try_ methods
    impl EventButtonState {
        /// Converts a `WebEventKind` into `EventButtonState`.
        pub const fn from_js(js_event: WebEventKind) -> Self {
            use {WebEventKind as J, EventButtonState as E};
            match js_event {
                J::Click | J::MouseDown | J::PointerDown => E::Pressed,
                J::MouseUp | J::PointerUp => E::Released,
                _ => E::Moved,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_js_as_mouse(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::MouseDown,
                E::Released => J::MouseUp,
                E::Moved => J::MouseMove,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_js_as_pointer(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::PointerDown,
                E::Released => J::PointerUp,
                E::Moved => J::PointerMove,
            }
        }
    }
}
