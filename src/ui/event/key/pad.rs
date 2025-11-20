// devela::ui::event::key::pad
//
//! Defines [`KeyPad`].
//

/* definitions */

/// Keypad keys.
#[repr(u8)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyPad {
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Multiply, Add, Subtract, Divide, Decimal,
    Enter, Equal, Comma,
}

/* impls */

#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyPad {
    /// Attempts to construct a `KeyPad` from a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    pub const fn from_js_code(code: &str) -> Option<Self> {
        use KeyPad as K;
        match code.as_bytes() {
            b"Numpad0" => Some(K::Num0), b"Numpad1" => Some(K::Num1), b"Numpad2" => Some(K::Num2),
            b"Numpad3" => Some(K::Num3), b"Numpad4" => Some(K::Num4), b"Numpad5" => Some(K::Num5),
            b"Numpad6" => Some(K::Num6), b"Numpad7" => Some(K::Num7), b"Numpad8" => Some(K::Num8),
            b"Numpad9" => Some(K::Num9), b"NumpadMultiply" => Some(K::Multiply),
            b"NumpadAdd" => Some(K::Add), b"NumpadSubtract" => Some(K::Subtract),
            b"NumpadDivide" => Some(K::Divide), b"NumpadDecimal" => Some(K::Decimal),
            b"NumpadEnter" => Some(K::Enter), b"NumpadEqual" => Some(K::Equal),
            b"NumpadComma" => Some(K::Comma),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_code_values
    pub const fn to_js_code(self) -> &'static str {
        use KeyPad as K;
        match self {
            K::Num0 => "Numpad0", K::Num1 => "Numpad1", K::Num2 => "Numpad2",
            K::Num3 => "Numpad3", K::Num4 => "Numpad4", K::Num5 => "Numpad5",
            K::Num6 => "Numpad6", K::Num7 => "Numpad7", K::Num8 => "Numpad8",
            K::Num9 => "Numpad9", K::Multiply => "NumpadMultiply", K::Add => "NumpadAdd",
            K::Subtract => "NumpadSubtract", K::Divide => "NumpadDivide",
            K::Decimal => "NumpadDecimal", K::Enter => "NumpadEnter", K::Equal => "NumpadEqual",
            K::Comma => "NumpadComma",
        }
    }

    /// Attempts to construct a `KeyPad` from a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
    pub const fn from_js_key(key: &str) -> Option<Self> {
        use KeyPad as K;
        match key.as_bytes() {
            b"0" => Some(K::Num0), b"1" => Some(K::Num1), b"2" => Some(K::Num2),
            b"3" => Some(K::Num3), b"4" => Some(K::Num4), b"5" => Some(K::Num5),
            b"6" => Some(K::Num6), b"7" => Some(K::Num7), b"8" => Some(K::Num8),
            b"9" => Some(K::Num9), b"*" => Some(K::Multiply), b"+" => Some(K::Add),
            b"-" => Some(K::Subtract), b"/" => Some(K::Divide), b"." => Some(K::Decimal),
            b"Enter" => Some(K::Enter), b"=" => Some(K::Equal), b"," => Some(K::Comma),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    // https://developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
    pub const fn to_js_key(self) -> &'static str {
        use KeyPad as K;
        match self {
            K::Num0 => "0", K::Num1 => "1", K::Num2 => "2", K::Num3 => "3", K::Num4 => "4",
            K::Num5 => "5", K::Num6 => "6", K::Num7 => "7", K::Num8 => "8", K::Num9 => "9",
            K::Multiply => "*", K::Add => "+", K::Subtract => "-", K::Divide => "/",
            K::Decimal => ".", K::Enter => "Enter", K::Equal => "=", K::Comma => ",",
        }
    }
}
