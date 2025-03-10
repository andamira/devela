// devela::lang:js::types::event
//
//! Defines [`JsEvent`], [`JsEventMouse`], [`JsEventPointer`].
//

use crate::{js_int32, js_number};

/// # Web API Events
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/Event>
/// - <https://developer.mozilla.org/en-US/docs/Web/API/EventTarget>
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum JsEvent {
    /// Fires when an element is clicked.
    Click,
    /// Fires when a key is pressed down.
    KeyDown,
    /// Fires when a key is released.
    KeyUp,
    /// Fires when the mouse button is pressed down.
    MouseDown,
    /// Fires when the mouse button is released.
    MouseUp,
    /// Fires when the mouse moves over an element.
    MouseMove,
    /// Fires when the pointer is pressed down.
    PointerDown,
    /// Fires when the pointer is released.
    PointerUp,
    /// Fires when the pointer is moved.
    PointerMove,
    ///
    GamepadPoll,
    /// Fires when the window is resized.
    Resize,
}

impl JsEvent {
    /// Returns the event name as a string.
    pub fn as_str(self) -> &'static str {
        use JsEvent as E;
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
        }
    }
}

/// # Web API Mouse Events
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
    /// If the event was a movement without a button click, this is `-1`.
    pub button: u8,
    /// A bitmask of buttons currently being held down (`1`: left, `2`: right, `4`: middle).
    pub buttons: u8,
}
impl JsEventMouse {
    pub(crate) fn new(x: js_number, y: js_number, button: u8, buttons: u8) -> Self {
        Self { x, y, button, buttons }
    }
}

/// # Web API Pointer Events
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
}
impl JsEventPointer {
    pub(crate) fn new(
        x: js_number,
        y: js_number,
        pressure: js_number,
        id: js_int32,
        tilt_x: i8,
        tilt_y: i8,
        twist: u16,
    ) -> Self {
        Self { x, y, pressure, id, tilt_x, tilt_y, twist }
    }
}
