// devela::ui::event::pointer
//
//! Defines [`EventMouse`], [`EventPointer`], [`EventPointerType`],
//! [`EventButton`], [`EventButtons`], [`EventButtonState`].
//

use crate::{_impl_init, KeyMods, f32bits_niche, set};

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
    Self::new(EventPointerType::INIT, 0, 0, 0, 0, 0, f32bits_niche::INIT, 0, 0, 0, None,
    EventButtonState::INIT, EventButtons::INIT, KeyMods::INIT) => EventPointer
}
#[rustfmt::skip]
impl EventPointer {
    /// Returns a normalized pointer event.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(kind: EventPointerType, id: u32, x: i32, y: i32, dx: i32, dy: i32,
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
pub enum EventPointerType {
    /// A mouse pointer.
    Mouse,
    /// A touch pointer.
    Touch,
    /// A pen pointer.
    Pen,
}
_impl_init! { Self::Mouse => EventPointerType }

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

#[doc = crate::_tags!(event interaction)]
/// Represents mouse, touch, or pen buttons.
#[doc = crate::_doc_meta!{
    location("ui/event"),
    test_size_of(EventButton = 2|16; niche Option),
}]
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
    /// The first auxiliary mouse button, commonly "back".
    X1,
    /// The second auxiliary mouse button, commonly "forward".
    X2,
    /// The third auxiliary mouse button.
    X3,
    /// The fourth auxiliary mouse button.
    X4,
    /// The fifth auxiliary mouse button.
    X5,
    /// Additional buttons (e.g., side buttons on advanced mice).
    Other(u8),
}
_impl_init! { Self::Left => EventButton }
impl EventButton {
    /// Returns some button as long as it's not 0.
    pub const fn new(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Left),
            2 => Some(Self::Middle),
            3 => Some(Self::Right),
            _ => Some(Self::Other(number)),
        }
    }

    /// Returns the button represented by the mask, if exactly one button is set.
    #[inline(always)]
    pub const fn from_one_bit_mask(mask: EventButtons) -> Option<EventButton> {
        match mask.bits() {
            1 => Some(EventButton::Left),
            2 => Some(EventButton::Right),
            4 => Some(EventButton::Middle),
            8 => Some(EventButton::X1),
            16 => Some(EventButton::X2),
            32 => Some(EventButton::X3),
            64 => Some(EventButton::X4),
            128 => Some(EventButton::X5),
            _ => None,
        }
    }
    /// Returns this button as a held-button mask, if it has a normalized role.
    #[inline(always)]
    pub const fn to_mask(self) -> EventButtons {
        match self {
            Self::Left => EventButtons::new().with(EventButtons::LEFT),
            Self::Right => EventButtons::new().with(EventButtons::RIGHT),
            Self::Middle => EventButtons::new().with(EventButtons::MIDDLE),
            Self::X1 => EventButtons::new().with(EventButtons::X1),
            Self::X2 => EventButtons::new().with(EventButtons::X2),
            Self::X3 => EventButtons::new().with(EventButtons::X3),
            Self::X4 => EventButtons::new().with(EventButtons::X4),
            Self::X5 => EventButtons::new().with(EventButtons::X5),
            Self::Other(_) => EventButtons::new(),
        }
    }
}

set! {
    #[doc = crate::_tags!(event interaction set)]
    /// A semantic bitmask of currently held pressable buttons.
    #[doc = crate::_doc_meta!{
        location("ui/event"),
        test_size_of(EventButtons = 1|8), // option = 2|16
    }]
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
    /// - **Web** can naturally carry `LEFT`, `RIGHT`, `MIDDLE`, `X1`, and `X2`.
    /// - **X11 core** currently sets only `LEFT`, `RIGHT`, and `MIDDLE`;
    ///   buttons 4 and 5 are treated as wheel input in the current backend.
    /// - **Terminal SGR** reports primary buttons, wheel motion, and some
    ///   extended button encodings. This library normalizes the first supported
    ///   non-primary button encodings into `X1..X5` when unambiguous.
    /// - Other backends may populate only the subset they can observe.
    ///
    /// This type is semantic and cross-platform.
    /// Backend-specific numbering should be translated at the backend edge.
    //
    // Firefox may consume browser Back/Forward mouse buttons before dispatching
    // DOM mouse events, so button=3/4 and buttons=8/16 may never reach web code.
    // WAIT: [firefox-back-forward-buttons](https://bugzilla.mozilla.org/show_bug.cgi?id=1933746)
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
#[doc = crate::_doc_meta!{location("ui/event")}]
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
_impl_init! { Self::Pressed => EventButtonState }
