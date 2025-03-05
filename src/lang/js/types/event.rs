// devela::lang:js::types::event

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
    /// Fires when the mouse moves over an element.
    MouseMove,
    /// Fires when the mouse button is pressed down.
    MouseDown,
    /// Fires when the mouse button is released.
    MouseUp,
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
            E::MouseMove => "mousemove",
            E::MouseDown => "mousedown",
            E::MouseUp => "mouseup",
            E::Resize => "resize",
        }
    }
}
