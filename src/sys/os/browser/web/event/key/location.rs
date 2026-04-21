// devela::sys::os::browser::web::event::key::location
//
//! Defines [`WebKeyLocation`].
//

use crate::KeyMod;

#[doc = crate::_tags!(interaction web)]
/// Which part of the keyboard the key event originates from
#[doc = crate::_doc_location!("sys/os/browser/web")]
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent#keyboard_locations>
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WebKeyLocation {
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
impl WebKeyLocation {
    /// Constructs a keyboard location from the numeric value of its representation.
    pub const fn from_repr(from: u8) -> Self {
        use WebKeyLocation as L;
        match from {
            0 => L::Standard,
            1 => L::Left,
            2 => L::Right,
            3 => L::NumPad,
            _ => L::Standard,
        }
    }
}

#[rustfmt::skip]
impl KeyMod {
    /// Atempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// physical [code] and [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    pub const fn from_js_code(code: &str, location: WebKeyLocation) -> Option<Self> {
        use {KeyMod as K, WebKeyLocation as L};
        match (code.as_bytes(), location) {
            (b"ShiftLeft", L::Left) => Some(K::LeftShift),
            (b"ControlLeft", L::Left) => Some(K::LeftControl),
            (b"AltLeft", L::Left) => Some(K::LeftAlt),
            (b"MetaLeft", L::Left) => Some(K::LeftSuper),
            (b"ShiftRight", L::Right) => Some(K::RightShift),
            (b"ControlRight", L::Right) => Some(K::RightControl),
            (b"AltRight", L::Right) => Some(K::RightAlt),
            (b"MetaRight", L::Right) => Some(K::RightSuper),
            (b"AltGraph", L::Standard) => Some(K::AltGr),
            (b"Level5Shift", L::Standard) => Some(K::IsoLevel5Shift),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code] and [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    pub const fn to_js_code(self) -> (&'static str, WebKeyLocation) {
        use {KeyMod as K, WebKeyLocation as L};
        match self {
            K::LeftShift => ("ShiftLeft", L::Left),
            K::LeftControl => ("ControlLeft", L::Left),
            K::LeftAlt => ("AltLeft", L::Left),
            K::LeftSuper => ("MetaLeft", L::Left),
            K::RightShift => ("ShiftRight", L::Right),
            K::RightControl => ("ControlRight", L::Right),
            K::RightAlt => ("AltRight", L::Right),
            K::RightSuper => ("MetaRight", L::Right),
            K::AltGr => ("AltGraph", L::Standard),
            K::IsoLevel5Shift => ("Level5Shift", L::Standard),
        }
    }
    /// Atempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// semantic [key] and [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn from_js_key(key: &str, location: WebKeyLocation) -> Option<Self> {
        use {KeyMod as K, WebKeyLocation as L};
        match (key.as_bytes(), location) {
            (b"Shift", L::Left) => Some(K::LeftShift),
            (b"Control", L::Left) => Some(K::LeftControl),
            (b"Alt", L::Left) => Some(K::LeftAlt),
            (b"Meta", L::Left) => Some(K::LeftSuper),
            (b"Shift", L::Right) => Some(K::RightShift),
            (b"Control", L::Right) => Some(K::RightControl),
            (b"Alt", L::Right) => Some(K::RightAlt),
            (b"Meta", L::Right) => Some(K::RightSuper),
            (b"AltGraph", L::Standard) => Some(K::AltGr),
            (b"Level5Shift", L::Standard) => Some(K::IsoLevel5Shift),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key] and [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn to_js_key(self) -> (&'static str, WebKeyLocation) {
        use {KeyMod as K, WebKeyLocation as L};
        match self {
            K::LeftShift => ("Shift", L::Left),
            K::LeftControl => ("Control", L::Left),
            K::LeftAlt => ("Alt", L::Left),
            K::LeftSuper => ("Meta", L::Left),
            K::RightShift => ("Shift", L::Right),
            K::RightControl => ("Control", L::Right),
            K::RightAlt => ("Alt", L::Right),
            K::RightSuper => ("Meta", L::Right),
            K::AltGr => ("AltGraph", L::Standard),
            K::IsoLevel5Shift => ("Level5Shift", L::Standard),
        }
    }
}
