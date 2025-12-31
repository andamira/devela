// devela::ui::event::pointer
//
//! Defines [`EventMouse`], [`EventPointer`], [`EventPointerType`],
//! [`EventButton`], [`EventButtonState`], [`EventWheel`].
//
// TOC
// - definitions
// - impls
// - tests

use crate::{ConstInit, EventTimestamp, NonZeroU8, f32bits_niche, unwrap};

/* definitions */

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Represents a basic mouse event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventMouse {
    /// The time stamp of when the event occurred.
    pub timestamp: Option<EventTimestamp>,
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
}

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Represents a pointer event (mouse, touch, or pen).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventPointer {
    /// The time stamp of the event.
    pub timestamp: Option<EventTimestamp>,
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
    pressure: f32bits_niche,
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
}

#[rustfmt::skip]
impl EventPointer {
    /// Gets the pressure.
    pub const fn get_pressure(&self) -> f32 { self.pressure.as_float() }
    /// Sets the pressure.
    pub const fn set_pressure(&mut self, pressure: f32) {
        self.pressure = f32bits_niche::new(pressure);
    }
}

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Enum representing the type of pointer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventPointerType {
    /// A mouse pointer.
    Mouse,
    /// A touch pointer.
    Touch,
    /// A pen pointer.
    Pen,
}

// /// Represents the phase of a pointer (useful for touch events).
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Represents mouse, touch, or pen buttons.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventButton {
    /// Left mouse button.
    #[default]
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button (usually the scroll wheel button).
    Middle,
    /// Additional buttons (e.g., side buttons on advanced mice).
    Other(NonZeroU8),
}
impl EventButton {
    /// Returns some button as long as it's not 0.
    pub const fn new(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Left),
            2 => Some(Self::Middle),
            3 => Some(Self::Right),
            _ => Some(Self::Other(unwrap![some? NonZeroU8::new(number)])),
        }
    }

    /// Returns some primary button (left, right, middle) from the mask, if only one is set.
    #[inline(always)]
    pub const fn primary_from_mask(mask: u8) -> Option<EventButton> {
        match mask {
            1 => Some(EventButton::Left),
            2 => Some(EventButton::Right),
            4 => Some(EventButton::Middle),
            _ => None,
        }
    }
}

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Represents the state of a button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventButtonState {
    /// The button was pressed.
    Pressed,
    /// The button was released.
    Released,
    /// The pointer moved without a button press/release.
    Moved,
}

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Represents a mouse wheel event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[rustfmt::skip]
mod init {
    use super::*;

    impl ConstInit for EventMouse {
        const INIT: Self = Self {
            x: 0, y: 0, button: None, state: EventButtonState::INIT, buttons: 0, timestamp: None,
        };
    }
    impl ConstInit for EventPointer {
        const INIT: Self = Self {
            kind: EventPointerType::INIT, id: 0, x: 0, y: 0, dx: 0, dy: 0,
            pressure: f32bits_niche::INIT, tilt_x: 0, tilt_y: 0, twist: 0, button: None,
            state: EventButtonState::INIT, timestamp: None,
        };
    }
    impl ConstInit for EventPointerType { const INIT: Self = Self::Mouse; }
    // impl ConstInit for EventPointerPhase { const INIT: Self = Self::Start; }

    impl ConstInit for EventButton { const INIT: Self = Self::Left; }
    impl ConstInit for EventButtonState { const INIT: Self = Self::Pressed; }
    impl ConstInit for EventWheel {
        const INIT: Self = Self { delta_x: 0, delta_y: 0, x: 0, y: 0, timestamp: None };
    }
}

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
mod impl_js {
    use super::*;
    use crate::{is, js_number, WebEventKind, WebEventMouse, JsInstant};

    /* mouse */

    impl EventMouse {
        /// Converts `WebEventMouse` to `EventMouse`.
        pub const fn from_js(js: WebEventMouse) -> EventMouse {
            EventMouse {
                x: js.x as i32,
                y: js.y as i32,
                button: EventButton::from_js(js.button),
                state: EventButtonState::from_js(js.etype),
                buttons: js.buttons,
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
                button: is![let Some(b) = self.button; b.to_js(); 255], // IMPROVE to_js
                buttons: self.buttons, // already a bitmask, directly compatible
                etype: self.state.to_js_as_mouse(),
                timestamp: is![let Some(t) = self.timestamp; t.to_js(); JsInstant { ms: 0.0 }],
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
