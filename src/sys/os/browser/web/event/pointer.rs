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
    /// Returns a new [`WebEventMouse`].
    pub const fn new(
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
    /// Returns a new [`WebEventPointer`].
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub const fn new(
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
    use crate::{
        EventButton, EventButtons, EventButtonState, EventKind, EventKindTimed, EventMouse,
        EventTimestamp, JsInstant, NonZeroU8, Timed, WebEventKind, WebEventMouse, is, js_number,
    };

    /* mouse */

    impl WebEventMouse {
        /// Converts `WebEventMouse` to `EventKindTimed`.
        pub const fn to_kind_timed(self) -> EventKindTimed {
            let kind = EventKind::Mouse(EventMouse {
                x: self.x as i32,
                y: self.y as i32,
                button: EventButton::from_web(self.button),
                state: EventButtonState::from_web(self.etype),
                buttons: EventButtons::from_bits(self.buttons),
            });
            let timestamp = Some(EventTimestamp::from_js(self.timestamp));
            EventKindTimed::new(kind, timestamp)
        }
        /// Converts a timed normalized `EventMouse` back to `WebEventMouse`.
        pub const fn from_event_mouse_timed(from: Timed<EventMouse, Option<EventTimestamp>>)
            -> WebEventMouse {
            let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
            WebEventMouse {
                x: from.value.x as js_number,
                y: from.value.y as js_number,
                button: is![let Some(b) = from.value.button, b.to_web(), 255], // IMPROVE to_web
                buttons: from.value.buttons.bits(), // already a bitmask, directly compatible
                etype: from.value.state.to_web_as_mouse(),
                timestamp,
            }
        }
    }
    impl From<WebEventMouse> for EventKindTimed {
        fn from(from: WebEventMouse) -> Self { from.to_kind_timed() }
    }
    impl From<Timed<EventMouse, Option<EventTimestamp>>> for WebEventMouse {
        fn from(from: Timed<EventMouse, Option<EventTimestamp>>) -> Self {
            Self::from_event_mouse_timed(from)
        }
    }

    /* button */

    impl EventButton {
        /// Converts a web API button index in [`WebEventMouse`] into `EventButton`.
        pub const fn from_web(js_button: u8) -> Option<Self> {
            match js_button {
                0 => Some(EventButton::Left),
                1 => Some(EventButton::Middle),
                2 => Some(EventButton::Right),
                255 => None, // (== -1_i8) represents "no button"
                n => Some(EventButton::Other(NonZeroU8::new(n).unwrap())),
            }
        }
        /// Converts an EventButton to a JavaScript button index in [`WebEventMouse`].
        pub const fn to_web(self) -> u8 {
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
        pub const fn from_web(js_event: WebEventKind) -> Self {
            use {WebEventKind as J, EventButtonState as E};
            match js_event {
                J::Click | J::MouseDown | J::PointerDown => E::Pressed,
                J::MouseUp | J::PointerUp => E::Released,
                _ => E::Moved,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_web_as_mouse(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::MouseDown,
                E::Released => J::MouseUp,
                E::Moved => J::MouseMove,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_web_as_pointer(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::PointerDown,
                E::Released => J::PointerUp,
                E::Moved => J::PointerMove,
            }
        }
    }
}
