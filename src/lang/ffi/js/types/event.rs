// devela::lang::ffi::js::types::event
//
//! Defines [`JsEventKind`], [`JsEventMouse`], [`JsEventPointer`], [`JsKeyLocation`].
//

#![allow(dead_code, reason = "feature-gated")]

use crate::{js_int32, js_number, JsInstant};

/// # A web API Event kind.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum JsEventKind {
    /// Unknown event. Default case.
    #[default]
    Unknown = 0,

    /// Fires when an element is clicked.
    Click = 1,

    /// Fires when a key is pressed down.
    KeyDown = 2,
    /// Fires when a key is released.
    KeyUp = 3,
    // no KeyPress (deprecated)
    /// Fires when the mouse button is pressed down.
    MouseDown = 4,
    /// Fires when the mouse button is released.
    MouseUp = 5,
    /// Fires when the mouse moves over an element.
    MouseMove = 6,

    /// Fires when the pointer is pressed down.
    PointerDown = 7,
    /// Fires when the pointer is released.
    PointerUp = 8,
    /// Fires when the pointer is moved.
    PointerMove = 9,

    ///
    GamepadPoll = 10,

    /// Fires when the window is resized.
    Resize = 11,
}
impl JsEventKind {
    /// Constructs a `JsEventKind` from its representation.
    pub const fn from_repr(from: u8) -> Self {
        use JsEventKind as E;
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
            10 => E::GamepadPoll,
            11 => E::Resize,
            _ => E::Unknown,
        }
    }
    /// Returns the event name as a string.
    pub const fn as_str(self) -> &'static str {
        use JsEventKind as E;
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
            E::GamepadPoll => "gamepadpoll",
            E::Resize => "resize",
            E::Unknown => "none",
        }
    }
}

/// # A web API Mouse Event.
///
/// Represents a JavaScript mouse event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent>
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsEventMouse {
    /// The X-coordinate of the mouse event relative to the viewport.
    pub x: js_number,
    /// The Y-coordinate of the mouse event relative to the viewport.
    pub y: js_number,
    /// The mouse button that triggered the event (`0`: left, `1`: middle, `2`: right).
    /// If the event was a movement without a button click, this is `-1` (255)
    pub button: u8,
    /// A bitmask of buttons currently being held down (`1`: left, `2`: right, `4`: middle).
    pub buttons: u8,
    /// The type of mouse event (Click, MouseDown, MouseMove, etc.).
    pub etype: JsEventKind,
    /// The JavaScript event timestamp.
    pub time_stamp: JsInstant,
}
impl JsEventMouse {
    pub(crate) fn new(
        x: js_number,
        y: js_number,
        button: u8,
        buttons: u8,
        etype: JsEventKind,
        time_stamp: JsInstant,
    ) -> Self {
        Self { x, y, button, buttons, etype, time_stamp }
    }
}

/// # A web API Pointer Event.
///
/// Represents a JavaScript pointer event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent>
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsEventPointer {
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
    pub etype: JsEventKind,
    /// The JavaScript event timestamp.
    pub time_stamp: JsInstant,
}
impl JsEventPointer {
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub(crate) fn new(
        x: js_number,
        y: js_number,
        pressure: js_number,
        id: js_int32,
        tilt_x: i8,
        tilt_y: i8,
        twist: u16,
        etype: JsEventKind,
        time_stamp: JsInstant,
    ) -> Self {
        Self { x, y, pressure, id, tilt_x, tilt_y, twist, etype, time_stamp }
    }
}

/// Which part of the keyboard the key event originates from
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent#keyboard_locations>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JsKeyLocation {
    /// The key is not identified as being located in a particular area of the keyboard. (Default)
    #[default]
    Standard = 0,
    /// On the left side of the keyboard.
    Left = 1,
    /// Ont he right side of the keyboard.
    Right = 2,
    /// On the numeric keypad.
    NumPad = 3,
}
impl JsKeyLocation {
    /// Constructs a keyboard location from the numeric value of its representation.
    pub const fn from_repr(from: u8) -> Self {
        use JsKeyLocation as L;
        match from {
            0 => L::Standard,
            1 => L::Left,
            2 => L::Right,
            3 => L::NumPad,
            _ => L::Standard,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[rustfmt::skip]
    fn sizes_of() {
        assert_eq![ 4, size_of::<JsEventKind>()];    // 32
        assert_eq![32, size_of::<JsEventMouse>()];   // 256
        assert_eq![48, size_of::<JsEventPointer>()]; // 384
        assert_eq![ 1, size_of::<JsKeyLocation>()];  // 8
    }
    #[test]
    fn js_event_conversions() {
        assert_eq!(JsEventKind::from_repr(2), JsEventKind::KeyDown);
        assert_eq!(JsEventKind::from_repr(3), JsEventKind::KeyUp);
        assert_eq!(JsEventKind::from_repr(99), JsEventKind::Unknown);
    }
}
