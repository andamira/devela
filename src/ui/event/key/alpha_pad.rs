// devela::ui::event::key::alpha_pad
//
//! Defines [`KeyAlpha`], [`KeyPad`].
//

/* definitions */

/// Alphanumeric (A-Z, 0-9) keys.
#[repr(u8)]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyAlpha {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
}

/// Keypad keys.
#[repr(u8)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyPad {
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Multiply, Add, Subtract, Divide, Decimal,
    Enter, Equal,
}

/* impls */

#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyAlpha {
    /// Attempts to construct a `KeyAlpha` from a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub const fn from_js_code(code: &str) -> Option<Self> {
        use KeyAlpha as K;
        match code.as_bytes() {
            b"KeyA" => Some(K::A), b"KeyB" => Some(K::B), b"KeyC" => Some(K::C),
            b"KeyD" => Some(K::D), b"KeyE" => Some(K::E), b"KeyF" => Some(K::F),
            b"KeyG" => Some(K::G), b"KeyH" => Some(K::H), b"KeyI" => Some(K::I),
            b"KeyJ" => Some(K::J), b"KeyK" => Some(K::K), b"KeyL" => Some(K::L),
            b"KeyM" => Some(K::M), b"KeyN" => Some(K::N), b"KeyO" => Some(K::O),
            b"KeyP" => Some(K::P), b"KeyQ" => Some(K::Q), b"KeyR" => Some(K::R),
            b"KeyS" => Some(K::S), b"KeyT" => Some(K::T), b"KeyU" => Some(K::U),
            b"KeyV" => Some(K::V), b"KeyW" => Some(K::W), b"KeyX" => Some(K::X),
            b"KeyY" => Some(K::Y), b"KeyZ" => Some(K::Z),
            b"Digit0" => Some(K::Num0), b"Digit1" => Some(K::Num1), b"Digit2" => Some(K::Num2),
            b"Digit3" => Some(K::Num3), b"Digit4" => Some(K::Num4), b"Digit5" => Some(K::Num5),
            b"Digit6" => Some(K::Num6), b"Digit7" => Some(K::Num7), b"Digit8" => Some(K::Num8),
            b"Digit9" => Some(K::Num9),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub const fn to_js_code(self) -> &'static str {
        use KeyAlpha as K;
        match self {
            K::A => "KeyA", K::B => "KeyB", K::C => "KeyC", K::D => "KeyD", K::E => "KeyE",
            K::F => "KeyF", K::G => "KeyG", K::H => "KeyH", K::I => "KeyI", K::J => "KeyJ",
            K::K => "KeyK", K::L => "KeyL", K::M => "KeyM", K::N => "KeyN", K::O => "KeyO",
            K::P => "KeyP", K::Q => "KeyQ", K::R => "KeyR", K::S => "KeyS", K::T => "KeyT",
            K::U => "KeyU", K::V => "KeyV", K::W => "KeyW", K::X => "KeyX", K::Y => "KeyY",
            K::Z => "KeyZ",
            K::Num0 => "Digit0", K::Num1 => "Digit1", K::Num2 => "Digit2", K::Num3 => "Digit3",
            K::Num4 => "Digit4", K::Num5 => "Digit5", K::Num6 => "Digit6", K::Num7 => "Digit7",
            K::Num8 => "Digit8", K::Num9 => "Digit9",
        }
    }

    /// Attempts to construct a `KeyAlpha` from a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn from_js_key(key: &str) -> Option<Self> {
        use KeyAlpha as K;
        match key.as_bytes() {
            b"A" => Some(K::A), b"B" => Some(K::B), b"C" => Some(K::C),
            b"D" => Some(K::D), b"E" => Some(K::E), b"F" => Some(K::F),
            b"G" => Some(K::G), b"H" => Some(K::H), b"I" => Some(K::I),
            b"J" => Some(K::J), b"K" => Some(K::K), b"L" => Some(K::L),
            b"M" => Some(K::M), b"N" => Some(K::N), b"O" => Some(K::O),
            b"P" => Some(K::P), b"Q" => Some(K::Q), b"R" => Some(K::R),
            b"S" => Some(K::S), b"T" => Some(K::T), b"U" => Some(K::U),
            b"V" => Some(K::V), b"W" => Some(K::W), b"X" => Some(K::X),
            b"Y" => Some(K::Y), b"Z" => Some(K::Z),
            b"0" => Some(K::Num0), b"1" => Some(K::Num1), b"2" => Some(K::Num2),
            b"3" => Some(K::Num3), b"4" => Some(K::Num4), b"5" => Some(K::Num5),
            b"6" => Some(K::Num6), b"7" => Some(K::Num7), b"8" => Some(K::Num8),
            b"9" => Some(K::Num9),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn to_js_key(self) -> &'static str {
        use KeyAlpha as K;
        match self {
            K::A => "A", K::B => "B", K::C => "C", K::D => "D", K::E => "E", K::F => "F",
            K::G => "G", K::H => "H", K::I => "I", K::J => "J", K::K => "K", K::L => "L",
            K::M => "M", K::N => "N", K::O => "O", K::P => "P", K::Q => "Q", K::R => "R",
            K::S => "S", K::T => "T", K::U => "U", K::V => "V", K::W => "W", K::X => "X",
            K::Y => "Y", K::Z => "Z",
            K::Num0 => "0", K::Num1 => "1", K::Num2 => "2", K::Num3 => "3", K::Num4 => "4",
            K::Num5 => "5", K::Num6 => "6", K::Num7 => "7", K::Num8 => "8", K::Num9 => "9",
        }
    }
}
#[rustfmt::skip]
#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyPad {
    /// Attempts to construct a `KeyPad` from a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
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
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub const fn to_js_code(self) -> &'static str {
        use KeyPad as K;
        match self {
            K::Num0 => "Numpad0", K::Num1 => "Numpad1", K::Num2 => "Numpad2",
            K::Num3 => "Numpad3", K::Num4 => "Numpad4", K::Num5 => "Numpad5",
            K::Num6 => "Numpad6", K::Num7 => "Numpad7", K::Num8 => "Numpad8",
            K::Num9 => "Numpad9", K::Multiply => "NumpadMultiply", K::Add => "NumpadAdd",
            K::Subtract => "NumpadSubtract", K::Divide => "NumpadDivide",
            K::Decimal => "NumpadDecimal", K::Enter => "NumpadEnter", K::Equal => "NumpadEqual",
        }
    }

    /// Attempts to construct a `KeyPad` from a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn from_js_key(key: &str) -> Option<Self> {
        use KeyPad as K;
        match key.as_bytes() {
            b"0" => Some(K::Num0), b"1" => Some(K::Num1), b"2" => Some(K::Num2),
            b"3" => Some(K::Num3), b"4" => Some(K::Num4), b"5" => Some(K::Num5),
            b"6" => Some(K::Num6), b"7" => Some(K::Num7), b"8" => Some(K::Num8),
            b"9" => Some(K::Num9), b"*" => Some(K::Multiply), b"+" => Some(K::Add),
            b"-" => Some(K::Subtract), b"/" => Some(K::Divide), b"." => Some(K::Decimal),
            b"Enter" => Some(K::Enter), b"=" => Some(K::Equal),
            _ => None,
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub const fn to_js_key(self) -> &'static str {
        use KeyPad as K;
        match self {
            K::Num0 => "0", K::Num1 => "1", K::Num2 => "2", K::Num3 => "3", K::Num4 => "4",
            K::Num5 => "5", K::Num6 => "6", K::Num7 => "7", K::Num8 => "8", K::Num9 => "9",
            K::Multiply => "*", K::Add => "+", K::Subtract => "-", K::Divide => "/",
            K::Decimal => ".", K::Enter => "Enter", K::Equal => "=",
        }
    }
}
