// devela::ui::event::pointer
//
//! Defines [`EventMouse`], [`EventPointer`], [`EventPointerType`],
//! [`EventButton`], [`EventButtonState`], [`EventWheel`].
//
// TOC
// - definitions
// - impls
// - tests

use crate::{EventTimestamp, NonZeroU8};

/* definitions */

/// Represents a basic mouse event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventMouse {
    /// The x-coordinate of the mouse cursor.
    pub x: i32,
    /// The y-coordinate of the mouse cursor.
    pub y: i32,
    /// The mouse button involved in the event.
    pub button: Option<EventButton>,
    /// The state of the mouse button (pressed, released, moved).
    pub state: EventButtonState,
    /// A bitmask of currently pressed buttons (`1`: left, `2`: right, `4`: middle).
    pub buttons: u8,
    /// The time stamp of when the event occurred.
    pub time_stamp: Option<EventTimestamp>,
}

/// Represents a pointer event (mouse, touch, or pen).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EventPointer {
    /// The type of pointer (mouse, touch, pen).
    pub kind: EventPointerType,
    /// Unique ID for touch and pen inputs (e.g., multi-touch gestures).
    pub id: u32,
    /// The x-coordinate of the pointer.
    pub x: i32,
    /// The y-coordinate of the pointer.
    pub y: i32,
    /// The delta movement since the last event.
    pub dx: i32,
    ///
    pub dy: i32,
    /// The pressure applied (0.0 to 1.0), applicable for pen/touch.
    pub pressure: f32,
    /// The tilt angle of a stylus (if applicable).
    pub tilt_x: i8,
    /// The tilt of the stylus along the Y-axis (-90° to 90°).
    pub tilt_y: i8,
    /// The rotation of the stylus around its own axis (0° to 359°).
    pub twist: u16,
    /// The button involved.
    pub button: Option<EventButton>,
    /// The state of the pointer (pressed, released, moved).
    pub state: EventButtonState,
    // /// The phase of the pointer (useful for touch events).
    // pub phase: EventPointerPhase,
    /// The time stamp of the event.
    pub time_stamp: Option<EventTimestamp>,
}
/// Enum representing the type of pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventPointerType {
    /// A mouse pointer.
    Mouse,
    /// A touch pointer.
    Touch,
    /// A pen pointer.
    Pen,
}
// /// Represents the phase of a pointer (useful for touch events).
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum EventPointerPhase {
//     /// The pointer has started touching the surface.
//     Start,
//     /// The pointer is moving.
//     Move,
//     /// The pointer was lifted from the surface.
//     End,
//     /// The input was cancelled (e.g., interrupted by system gestures).
//     Cancel,
// }

/// Represents mouse, touch, or pen buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button (usually the scroll wheel button).
    Middle,
    /// Additional buttons (e.g., side buttons on advanced mice).
    Other(NonZeroU8),
}

/// Represents the state of a button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventButtonState {
    /// The button was pressed.
    Pressed,
    /// The button was released.
    Released,
    /// The pointer moved without a button press/release.
    Moved,
}

/// Represents a mouse wheel event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventWheel {
    /// The amount scrolled horizontally.
    pub delta_x: i32,
    /// The amount scrolled vertically.
    pub delta_y: i32,
    /// The x-coordinate of the mouse cursor during the event.
    pub x: i32,
    /// The y-coordinate of the mouse cursor during the event.
    pub y: i32,
    /// The timestamp of when the event occurred.
    pub timestamp: Option<EventTimestamp>,
}

/* impls */

#[cfg(feature = "js")] #[rustfmt::skip]
mod impl_js {
    use super::*;
    use crate::{is, js_number, JsEventKind, JsEventMouse, JsInstant};

    /* mouse */

    impl EventMouse {
        /// Converts `JsEventMouse` to `EventMouse`.
        pub const fn from_js(js: JsEventMouse) -> EventMouse {
            EventMouse {
                x: js.x as i32,
                y: js.y as i32,
                button: EventButton::from_js(js.button),
                state: EventButtonState::from_js(js.etype),
                buttons: js.buttons,
                time_stamp: EventTimestamp::try_from_js(js.time_stamp),
            }
        }
    }

    impl EventMouse {
        /// Converts `EventMouse` to `JsEventMouse`.
        pub const fn to_js(self) -> JsEventMouse {
            JsEventMouse {
                x: self.x as js_number,
                y: self.y as js_number,
                button: is![let Some(b) = self.button; b.to_js(); 255], // IMPROVE to_js
                buttons: self.buttons, // Already a bitmask, directly compatible
                etype: self.state.to_js_as_mouse(),
                time_stamp: is![let Some(t) = self.time_stamp; t.to_js(); JsInstant { ms: 0.0 }],
            }
        }
    }

    impl From<JsEventMouse> for EventMouse {
        fn from(from: JsEventMouse) -> Self { Self::from_js(from) }
    }
    impl From<EventMouse> for JsEventMouse {
        fn from(from: EventMouse) -> Self { from.to_js() }
    }

    /* button */

    impl EventButton {
        /// Converts a JavaScript button index in [`JsEventMouse`] into `EventButton`.
        pub const fn from_js(js_button: u8) -> Option<Self> {
            match js_button {
                0 => Some(EventButton::Left),
                1 => Some(EventButton::Middle),
                2 => Some(EventButton::Right),
                255 => None, // (== -1_i8) represents "no button"
                n => Some(EventButton::Other(NonZeroU8::new(n).unwrap())),
            }
        }
        /// Converts an EventButton to a JavaScript button index in [`JsEventMouse`].
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
        /// Converts a `JsEventKind` into `EventButtonState`.
        pub const fn from_js(js_event: JsEventKind) -> Self {
            use {JsEventKind as J, EventButtonState as E};
            match js_event {
                J::Click | J::MouseDown | J::PointerDown => E::Pressed,
                J::MouseUp | J::PointerUp => E::Released,
                _ => E::Moved,
            }
        }
        /// Converts a `EventButtonState` into a `JsEventKind`.
        pub const fn to_js_as_mouse(self) -> JsEventKind {
            use {JsEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::MouseDown,
                E::Released => J::MouseUp,
                E::Moved => J::MouseMove,
            }
        }
        /// Converts a `EventButtonState` into a `JsEventKind`.
        pub const fn to_js_as_pointer(self) -> JsEventKind {
            use {JsEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::PointerDown,
                E::Released => J::PointerUp,
                E::Moved => J::PointerMove,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[rustfmt::skip]
    fn sizes_of() {
        assert_eq![16, size_of::<EventMouse>()];        // 128
        assert_eq![02, size_of::<EventButton>()];       // 16
        assert_eq![01, size_of::<EventButtonState>()];  // 8
        assert_eq![36, size_of::<EventPointer>()];      // 288
        assert_eq![01, size_of::<EventPointerType>()];  // 8
        // assert_eq![01, size_of::<EventPointerPhase>()]; // 8
        assert_eq![20, size_of::<EventWheel>()];        // 160
    }
}
