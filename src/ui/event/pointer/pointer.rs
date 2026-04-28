// devela::ui::event::pointer
//
//! Defines [`EventMouse`], [`EventPointer`], [`EventPointerType`],
//! [`EventButton`], [`EventButtons`], [`EventButtonState`].
//

use crate::{_impl_init, NonZeroU8, f32bits_niche, set, unwrap};

/* definitions */

#[doc = crate::_tags!(event interaction)]
/// Represents a basic mouse event.
#[doc = crate::_doc_location!("ui/event")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub buttons: EventButtons,
}
_impl_init! {
    ConstInit: Self::new(0, 0, None, EventButtonState::INIT, EventButtons::INIT) => EventMouse
}
#[rustfmt::skip]
impl EventMouse {
    /// Returns a normalized wheel-scroll event.
    pub const fn new(x: i32, y: i32, button: Option<EventButton>,
        state: EventButtonState, buttons: EventButtons,) -> Self {
        Self { x, y, button, state, buttons }
    }
}

#[doc = crate::_tags!(event interaction)]
/// Represents a pointer event (mouse, touch, or pen).
#[doc = crate::_doc_location!("ui/event")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
_impl_init! { ConstInit: Self::new(EventPointerType::INIT, 0, 0, 0, 0, 0,
f32bits_niche::INIT, 0, 0, 0, None, EventButtonState::INIT) => EventPointer }
#[rustfmt::skip]
impl EventPointer {
    /// Returns a normalized wheel-scroll event.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(kind: EventPointerType, id: u32, x: i32, y: i32, dx: i32, dy: i32,
        pressure: f32bits_niche, tilt_x: i8, tilt_y: i8, twist: u16,
        button: Option<EventButton>, state: EventButtonState) -> Self {
        Self { kind, id, x, y, dx, dy, pressure, tilt_x, tilt_y, twist, button, state }
    }
    /// Gets the pressure.
    pub const fn get_pressure(&self) -> f32 {
        self.pressure.as_float()
    }
    /// Sets the pressure.
    pub const fn set_pressure(&mut self, pressure: f32) {
        self.pressure = f32bits_niche::new(pressure);
    }
}

#[doc = crate::_tags!(event interaction)]
/// Enum representing the type of pointer.
#[doc = crate::_doc_location!("ui/event")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventPointerType {
    /// A mouse pointer.
    Mouse,
    /// A touch pointer.
    Touch,
    /// A pen pointer.
    Pen,
}
_impl_init! { ConstInit: Self::Mouse => EventPointerType }

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
// _impl_init! { ConstInit: Self::Start => EventPointerPhase }

#[doc = crate::_tags!(event interaction)]
/// Represents mouse, touch, or pen buttons.
#[doc = crate::_doc_location!("ui/event")]
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
_impl_init! { ConstInit: Self::Left => EventButton }
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
    pub const fn primary_from_mask(mask: EventButtons) -> Option<EventButton> {
        match mask.bits() {
            1 => Some(EventButton::Left),
            2 => Some(EventButton::Right),
            4 => Some(EventButton::Middle),
            _ => None,
        }
    }
}

set! {
    #[doc = crate::_tags!(event interaction)]
    /// A semantic bitmask of currently held pressable buttons.
    #[doc = crate::_doc_location!("ui/event")]
    ///
    /// The bits represent normalized button roles, not raw backend button numbers.
    ///
    /// This means:
    /// - `LEFT`, `RIGHT`, and `MIDDLE` are the three primary buttons.
    /// - `X1..X5` are additional auxiliary buttons when a backend can report them.
    /// - wheel motion is **not** represented here; it belongs in
    ///   [`EventWheel`][crate::EventWheel].
    ///
    /// Unsupported buttons are left cleared.
    ///
    /// # Backend notes
    /// - **X11** currently sets only `LEFT`, `RIGHT`, and `MIDDLE`,
    ///   because the current X11 state-mask mapping only exposes those three.
    /// - **Web** can naturally carry `LEFT`, `RIGHT`, `MIDDLE`, `X1`, and `X2`
    ///   from the DOM `buttons` bitmask.
    /// - Other backends may populate only the subset they can observe.
    ///
    /// This type is semantic and cross-platform.
    /// Backend-specific numbering should be translated at the backend edge.
    pub struct EventButtons(u8) {
        /// The primary left button.
        LEFT = 0;
        /// The primary right button.
        RIGHT = 1;
        /// The primary middle button.
        MIDDLE = 2;
        /// The first auxiliary button, often “back”.
        X1 = 3;
        /// The second auxiliary button, often “forward”.
        X2 = 4;
        /// The third auxiliary button, when available.
        X3 = 5;
        /// The fourth auxiliary button, when available.
        X4 = 6;
        /// The fifth auxiliary button, when available.
        X5 = 7;
    }
}

#[doc = crate::_tags!(event interaction)]
/// Represents the state of a button.
#[doc = crate::_doc_location!("ui/event")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventButtonState {
    /// The button was pressed.
    ///
    /// This is the default.
    #[default]
    Pressed,
    /// The button was released.
    Released,
    /// The pointer moved without a button press/release.
    Moved,
}
_impl_init! { ConstInit: Self::Pressed => EventButtonState }
