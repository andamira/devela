// devela/src/ui/event/pointer/pointer.rs
//
//! Defines [`EventMouse`], [`EventPointer`], [`EventPointerKind`].
//

use crate::{_impl_init, EventButton, EventButtonState, EventButtons, KeyMods, f32bits_niche};

/* definitions */

#[doc = crate::_tags!(event interaction)]
/// Represents a basic mouse event.
#[doc = crate::_doc_meta!{
    location("ui/event"),
    test_size_of(EventMouse = 16|128; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventMouse {
    /// The x-coordinate of the mouse cursor.
    pub x: i32,
    /// The y-coordinate of the mouse cursor.
    pub y: i32,
    /// The mouse button involved in the event.
    pub button: Option<EventButton>,
    /// The state of the mouse button (Pressed, Released, Moved).
    pub state: EventButtonState,
    /// A bitmask of currently pressed buttons.
    pub buttons: EventButtons,
    /// Keyboard modifiers held during the event, when reported by the backend.
    ///
    /// Terminal backends may not report every modifier combination
    /// because some are reserved by the terminal emulator.
    pub mods: KeyMods,
}
_impl_init! {
    Self::new(0, 0, None, EventButtonState::INIT, EventButtons::INIT, KeyMods::INIT) => EventMouse
}
#[rustfmt::skip]
impl EventMouse {
    /// Returns a normalized mouse event.
    pub const fn new(x: i32, y: i32, button: Option<EventButton>,
        state: EventButtonState, buttons: EventButtons, mods: KeyMods) -> Self {
        Self { x, y, button, state, buttons, mods }
    }
}

#[doc = crate::_tags!(event interaction)]
/// Represents a pointer event (mouse, touch, or pen).
#[doc = crate::_doc_meta!{
    location("ui/event"),
    test_size_of(EventPointer = 36|288; niche Option),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EventPointer {
    /// The type of pointer (mouse, touch, pen).
    pub kind: EventPointerKind,
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
    /// The state of the pointer (Pressed, Released, Moved).
    pub state: EventButtonState,
    /// A bitmask of currently pressed buttons.
    pub buttons: EventButtons,
    /// Keyboard modifiers held during the event, when reported by the backend.
    ///
    /// Terminal backends may not report every modifier combination
    /// because some are reserved by the terminal emulator.
    pub mods: KeyMods,
    // /// The phase of the pointer (useful for touch events).
    // pub phase: EventPointerPhase,
}
_impl_init! {
    Self::new(EventPointerKind::INIT, 0, 0, 0, 0, 0, f32bits_niche::INIT, 0, 0, 0, None,
    EventButtonState::INIT, EventButtons::INIT, KeyMods::INIT) => EventPointer
}
#[rustfmt::skip]
impl EventPointer {
    /// Returns a normalized pointer event.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(kind: EventPointerKind, id: u32, x: i32, y: i32, dx: i32, dy: i32,
        pressure: f32bits_niche, tilt_x: i8, tilt_y: i8, twist: u16, button: Option<EventButton>,
        state: EventButtonState, buttons: EventButtons, mods: KeyMods) -> Self {
        Self {
            kind, id, x, y, dx, dy, pressure, tilt_x, tilt_y, twist, button, state, buttons, mods
        }
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
#[doc = crate::_doc_meta!{location("ui/event")}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventPointerKind {
    /// A mouse pointer.
    Mouse,
    /// A touch pointer.
    Touch,
    /// A pen pointer.
    Pen,
}
_impl_init! { Self::Mouse => EventPointerKind }

// /// Represents the phase of a pointer (useful for touch events).
// #[doc = crate::_doc_meta!{location("ui/event")}]
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
// _impl_init! { Self::Start => EventPointerPhase }
