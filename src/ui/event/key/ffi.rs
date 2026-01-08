// devela::ui::event::key::key
//
//! Defines [`KeyFfi`].
//
// TOC
// - enum KeyFfi
// - static F_KEYS
// - impls `js`

use super::*;
use crate::{ConstInit, is, unwrap};
#[cfg(all(feature = "js", not(windows)))]
crate::items! {
    use crate::{Char, WebKeyLocation, Slice};
    crate::_use! {basic::from_utf8}
}

#[doc = crate::_tags!(interaction ffi)]
/// An FFI-safe version of [`Key`], used in [`EventKeyFfi`][crate::EventKeyFfi].
// (The main difference is in the Char variant.)
#[doc = crate::_doc_location!("ui/event")]
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyFfi {
    // common control keys
    Backspace, Enter, Tab, Escape, Space,
    // navigation keys
    Left, Right, Up, Down,
    Home, End, PageUp, PageDown,
    // editing keys
    Delete, Insert,
    // lock keys
    CapsLock, ScrollLock, NumLock,
    // special system kesy
    PrintScreen, Pause, Menu,

    /// A keypad key.
    Pad(KeyPad),
    /// A multimedia key.
    Media(KeyMedia),
    /// A modifier key.
    Mod(KeyMod),

    /// Function keys (F1-F48).
    Fn(u8),

    #[doc = crate::_tags!(ffi)]
    /// A unicode character (FFI-safe, stored as `u32`).
    Char(u32),

    /// A dead key (accent prefix, composition).
    Dead(KeyDead),

    // Letters
    A, B, C, D, E, F, G, H, I, J,
    K, L, M, N, O, P, Q, R, S, T,
    U, V, W, X, Y, Z,

    // Digits (E row)
    Digit1, Digit2, Digit3, Digit4, Digit5,
    Digit6, Digit7, Digit8, Digit9, Digit0,

    // ANSI punctuation / symbol positions
    // E row
    /// Physical key left of '1'
    Backquote,
    /// Physical `-` key.
    Minus,
    /// Physical `=` key.
    Equal,
    // D row
    /// Physical `[` key.
    BracketLeft,
    /// Physical `]` key.
    BracketRight,
    // C (home) row
    /// Physical `;` key.
    Semicolon,
    /// Physical `'` key.
    Quote,
    // B row
    /// Physical `\` key.
    Backslash,
    /// ISO-102 extra key.
    IntlBackslash,
    // A row
    /// Physical `,` key.
    Comma,
    /// Physical `.` key.
    Dot,
    /// Physical `/` key.
    Slash,

    /// A physical scancode key.
    Scancode(u16),

    #[default]
    Unknown,
}
impl ConstInit for KeyFfi {
    const INIT: Self = Self::Unknown;
}

impl Key {
    /// Converts `Key` to `KeyFfi`.
    pub const fn to_ffi(self) -> KeyFfi {
        use {Key as K, KeyFfi as F};
        match self {
            // common control
            K::Backspace => F::Backspace,
            K::Enter => F::Enter,
            K::Tab => F::Tab,
            K::Escape => F::Escape,
            K::Space => F::Space,
            // navigation
            K::Left => F::Left,
            K::Right => F::Right,
            K::Up => F::Up,
            K::Down => F::Down,
            K::Home => F::Home,
            K::End => F::End,
            K::PageUp => F::PageUp,
            K::PageDown => F::PageDown,
            // editing
            K::Delete => F::Delete,
            K::Insert => F::Insert,
            // lock
            K::CapsLock => F::CapsLock,
            K::ScrollLock => F::ScrollLock,
            K::NumLock => F::NumLock,
            // special system
            K::PrintScreen => F::PrintScreen,
            K::Pause => F::Pause,
            K::Menu => F::Menu,
            //
            K::Pad(p) => F::Pad(p),
            K::Mod(m) => F::Mod(m),
            K::Media(m) => F::Media(m),
            K::Fn(f) => F::Fn(f),
            K::Char(c) => F::Char(c as u32), // Convert char to u32
            K::Dead(d) => F::Dead(d),
            // Letters
            K::A => F::A,
            K::B => F::B,
            K::C => F::C,
            K::D => F::D,
            K::E => F::E,
            K::F => F::F,
            K::G => F::G,
            K::H => F::H,
            K::I => F::I,
            K::J => F::J,
            K::K => F::K,
            K::L => F::L,
            K::M => F::M,
            K::N => F::N,
            K::O => F::O,
            K::P => F::P,
            K::Q => F::Q,
            K::R => F::R,
            K::S => F::S,
            K::T => F::T,
            K::U => F::U,
            K::V => F::V,
            K::W => F::W,
            K::X => F::X,
            K::Y => F::Y,
            K::Z => F::Z,
            // Digits (E row)
            K::Digit1 => F::Digit1,
            K::Digit2 => F::Digit2,
            K::Digit3 => F::Digit3,
            K::Digit4 => F::Digit4,
            K::Digit5 => F::Digit5,
            K::Digit6 => F::Digit6,
            K::Digit7 => F::Digit7,
            K::Digit8 => F::Digit8,
            K::Digit9 => F::Digit9,
            K::Digit0 => F::Digit0,

            // ANSI punctuation / symbol positions
            // E row
            K::Backquote => F::Backquote,
            K::Minus => F::Minus,
            K::Equal => F::Equal,
            // D row
            K::BracketLeft => F::BracketLeft,
            K::BracketRight => F::BracketRight,
            // C (home) row
            K::Semicolon => F::Semicolon,
            K::Quote => F::Quote,
            // B row
            K::Backslash => F::Backslash,
            K::IntlBackslash => F::IntlBackslash,
            // A row
            K::Comma => F::Comma,
            K::Dot => F::Dot,
            K::Slash => F::Slash,
            //
            K::Scancode(sc) => F::Scancode(sc),
            K::Unknown => F::Unknown,
        }
    }

    /// Converts `KeyFfi` to `Key`.
    pub const fn from_ffi(from: KeyFfi) -> Key {
        use {Key as K, KeyFfi as F};
        match from {
            // common control
            F::Backspace => K::Backspace,
            F::Enter => K::Enter,
            F::Tab => K::Tab,
            F::Escape => K::Escape,
            F::Space => K::Space,
            // navigation
            F::Left => K::Left,
            F::Right => K::Right,
            F::Up => K::Up,
            F::Down => K::Down,
            F::Home => K::Home,
            F::End => K::End,
            F::PageUp => K::PageUp,
            // editing
            F::PageDown => K::PageDown,
            F::Delete => K::Delete,
            F::Insert => K::Insert,
            // lock
            F::CapsLock => K::CapsLock,
            F::ScrollLock => K::ScrollLock,
            F::NumLock => K::NumLock,
            // special system
            F::PrintScreen => K::PrintScreen,
            F::Pause => K::Pause,
            F::Menu => K::Menu,
            F::Pad(p) => K::Pad(p),
            F::Mod(m) => K::Mod(m),
            F::Media(m) => K::Media(m),
            F::Fn(f) => K::Fn(f),
            F::Char(c) => K::Char(unwrap![some_or char::from_u32(c), char::REPLACEMENT_CHARACTER]),
            F::Dead(d) => K::Dead(d),
            // Letters
            F::A => K::A,
            F::B => K::B,
            F::C => K::C,
            F::D => K::D,
            F::E => K::E,
            F::F => K::F,
            F::G => K::G,
            F::H => K::H,
            F::I => K::I,
            F::J => K::J,
            F::K => K::K,
            F::L => K::L,
            F::M => K::M,
            F::N => K::N,
            F::O => K::O,
            F::P => K::P,
            F::Q => K::Q,
            F::R => K::R,
            F::S => K::S,
            F::T => K::T,
            F::U => K::U,
            F::V => K::V,
            F::W => K::W,
            F::X => K::X,
            F::Y => K::Y,
            F::Z => K::Z,
            // Digits (E row)
            F::Digit1 => K::Digit1,
            F::Digit2 => K::Digit2,
            F::Digit3 => K::Digit3,
            F::Digit4 => K::Digit4,
            F::Digit5 => K::Digit5,
            F::Digit6 => K::Digit6,
            F::Digit7 => K::Digit7,
            F::Digit8 => K::Digit8,
            F::Digit9 => K::Digit9,
            F::Digit0 => K::Digit0,

            // ANSI punctuation / symbol positions
            // E row
            F::Backquote => K::Backquote,
            F::Minus => K::Minus,
            F::Equal => K::Equal,
            // D row
            F::BracketLeft => K::BracketLeft,
            F::BracketRight => K::BracketRight,
            // C (home) row
            F::Semicolon => K::Semicolon,
            F::Quote => K::Quote,
            // B row
            F::Backslash => K::Backslash,
            F::IntlBackslash => K::IntlBackslash,
            // A row
            F::Comma => K::Comma,
            F::Dot => K::Dot,
            F::Slash => K::Slash,
            //
            F::Scancode(sc) => K::Scancode(sc),
            F::Unknown => K::Unknown,
        }
    }
}

crate::items! {
    impl From<Key> for KeyFfi { fn from(k: Key) -> Self { Key::to_ffi(k) } }
    impl From<KeyFfi> for Key { fn from(k: KeyFfi) -> Self { Key::from_ffi(k) } }
}

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
pub(crate) const F_KEYS: [&str; 48] = [
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24",
    "F25", "F26", "F27", "F28", "F29", "F30", "F31", "F32", "F33", "F34", "F35", "F36",
    "F37", "F38", "F39", "F40", "F41", "F42", "F43", "F44", "F45", "F46", "F47", "F48",
];

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyFfi {
    // IMPROVE generalize and move to some namespace
    // const fn parse_u8_from_bytes(mut bytes: &[u8]) -> (u8, bool) {
    //     let (mut num, mut valid) = (0u8, false);
    //     while let Some((&digit, rest)) = bytes.split_first() {
    //         if digit < b'0' || digit > b'9' { return (0, false); }
    //         num = num * 10 + (digit - b'0');
    //         valid = true;
    //         bytes = rest;
    //     }
    //     (num, valid)
    // }
    const fn parse_u8_from_bytes(bytes: &[u8]) -> Option<u8> {
        let (mut num, mut index, mut found_digit) = (0u8, 0usize, false);
        while index < bytes.len() {
            let digit = bytes[index];
            if digit < b'0' || digit > b'9' { break; } // Stop at first invalid character
            let digit_value = digit - b'0';
            if num > (u8::MAX / 10) || (num == u8::MAX / 10 && digit_value > u8::MAX % 10) {
                return None;
            }
            num = num * 10 + digit_value;
            found_digit = true;
            index += 1;
        }
        if found_digit { Some(num) } else { None }
    }

    /// Attempts to construct a `KeyFfi` from a JavaScript `KeyboardEvent`
    /// physical [code] + a [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    //
    // Returns `None` for unmappable codes to distinguish invalid JS input
    // from the internal `Key::Unknown` fallback.
    pub const fn from_js_code(code: &str, location: WebKeyLocation) -> Option<Self> {
        use KeyFfi as K;
        match code.as_bytes() {
            b"Backspace" => Some(K::Backspace),
            b"Enter" => Some(K::Enter),
            b"Tab" => Some(K::Tab),
            b"Escape" => Some(K::Escape),
            b"Space" => Some(K::Space),
            //
            b"ArrowLeft" => Some(K::Left),
            b"ArrowRight" => Some(K::Right),
            b"ArrowUp" => Some(K::Up),
            b"ArrowDown" => Some(K::Down),
            //
            b"Home" => Some(K::Home),
            b"End" => Some(K::End),
            b"PageUp" => Some(K::PageUp),
            b"PageDown" => Some(K::PageDown),
            //
            b"Delete" => Some(K::Delete),
            b"Insert" => Some(K::Insert),
            //
            b"CapsLock" => Some(K::CapsLock),
            b"ScrollLock" => Some(K::ScrollLock),
            b"NumLock" => Some(K::NumLock),
            //
            b"PrintScreen" => Some(K::PrintScreen),
            b"Pause" => Some(K::Pause),
            b"ContextMenu" => Some(K::Menu),
            // letters
            b"KeyA" => Some(K::A), b"KeyB" => Some(K::B), b"KeyC" => Some(K::C),
            b"KeyD" => Some(K::D), b"KeyE" => Some(K::E), b"KeyF" => Some(K::F),
            b"KeyG" => Some(K::G), b"KeyH" => Some(K::H), b"KeyI" => Some(K::I),
            b"KeyJ" => Some(K::J), b"KeyK" => Some(K::K), b"KeyL" => Some(K::L),
            b"KeyM" => Some(K::M), b"KeyN" => Some(K::N), b"KeyO" => Some(K::O),
            b"KeyP" => Some(K::P), b"KeyQ" => Some(K::Q), b"KeyR" => Some(K::R),
            b"KeyS" => Some(K::S), b"KeyT" => Some(K::T), b"KeyU" => Some(K::U),
            b"KeyV" => Some(K::V), b"KeyW" => Some(K::W), b"KeyX" => Some(K::X),
            b"KeyY" => Some(K::Y), b"KeyZ" => Some(K::Z),
            // numbers
            b"Digit0" => Some(K::Digit0), b"Digit1" => Some(K::Digit1),
            b"Digit2" => Some(K::Digit2), b"Digit3" => Some(K::Digit3),
            b"Digit4" => Some(K::Digit4), b"Digit5" => Some(K::Digit5),
            b"Digit6" => Some(K::Digit6), b"Digit7" => Some(K::Digit7),
            b"Digit8" => Some(K::Digit8), b"Digit9" => Some(K::Digit9),
            _ => {
                // Try converting from inner enums
                if let Some(pad) = KeyPad::from_js_code(code) {
                    return Some(K::Pad(pad));
                }
                if let Some(media) = KeyMedia::from_js_code(code) {
                    return Some(K::Media(media));
                }
                if let Some(mod_key) = KeyMod::from_js_code(code, location) {
                    return Some(K::Mod(mod_key));
                }
                let cbytes = code.as_bytes();
                // Check if the code starts with "F" (Function keys)
                if let Some(first) = cbytes.first() {
                    if *first == b'F' {
                        if let Some(n) = Self::parse_u8_from_bytes(Slice::range_from(cbytes, 1)) {
                            if n >= 1 && n <= 48 { return Some(K::Fn(n)); }
                        }
                    }
                }
                None
            }
        }
    }
    /// Returns a JavaScript `KeyboardEvent` physical [code] + a [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn to_js_code(self) -> (&'static str, WebKeyLocation) {
        use WebKeyLocation::{self as L, Standard as Std};
        use KeyFfi as K;
        match self {
            K::Backspace => ("Backspace", Std),
            K::Enter => ("Enter", Std),
            K::Tab => ("Tab", Std),
            K::Escape => ("Escape", Std),
            K::Space => ("Space", Std),
            //
            K::Left => ("ArrowLeft", Std),
            K::Right => ("ArrowRight", Std),
            K::Up => ("ArrowUp", Std),
            K::Down => ("ArrowDown", Std),
            //
            K::Home => ("Home", Std),
            K::End => ("End", Std),
            K::PageUp => ("PageUp", Std),
            K::PageDown => ("PageDown", Std),
            //
            K::Delete => ("Delete", Std),
            K::Insert => ("Insert", Std),
            //
            K::CapsLock => ("CapsLock", Std),
            K::ScrollLock => ("ScrollLock", Std),
            K::NumLock => ("NumLock", Std),
            //
            K::PrintScreen => ("PrintScreen", Std),
            K::Pause => ("Pause", Std),
            K::Menu => ("ContextMenu", Std),
            // letters
            K::A => ("KeyA", Std), K::B => ("KeyB", Std), K::C => ("KeyC", Std),
            K::D => ("KeyD", Std), K::E => ("KeyE", Std), K::F => ("KeyF", Std),
            K::G => ("KeyG", Std), K::H => ("KeyH", Std), K::I => ("KeyI", Std),
            K::J => ("KeyJ", Std), K::K => ("KeyK", Std), K::L => ("KeyL", Std),
            K::M => ("KeyM", Std), K::N => ("KeyN", Std), K::O => ("KeyO", Std),
            K::P => ("KeyP", Std), K::Q => ("KeyQ", Std), K::R => ("KeyR", Std),
            K::S => ("KeyS", Std), K::T => ("KeyT", Std), K::U => ("KeyU", Std),
            K::V => ("KeyV", Std), K::W => ("KeyW", Std), K::X => ("KeyX", Std),
            K::Y => ("KeyY", Std), K::Z => ("KeyZ", Std),
            // numbers
            K::Digit0 => ("Digit0", Std), K::Digit1 => ("Digit1", Std),
            K::Digit2 => ("Digit2", Std), K::Digit3 => ("Digit3", Std),
            K::Digit4 => ("Digit4", Std), K::Digit5 => ("Digit5", Std),
            K::Digit6 => ("Digit6", Std), K::Digit7 => ("Digit7", Std),
            K::Digit8 => ("Digit8", Std), K::Digit9 => ("Digit9", Std),
            K::Pad(pad) => (pad.to_js_code(), L::NumPad),
            K::Media(media) => (media.to_js_code(), Std),
            K::Mod(mod_key) => mod_key.to_js_code(),
            //
            K::Fn(num) => match num {
                1..=48 => (F_KEYS[(num - 1) as usize], Std),
                _ => ("Unknown", Std),
            },
            K::Char(c) => {
                // Returns the ASCII character, or Unknown otherwise
                is![Char(c).is_ascii(); return (Char(c).as_ascii_unchecked(), Std)];
                ("Unknown", Std)
            },
            // NOTE: Javascript has no concept of scan codes, so everything else is unknown
            // Key::Scancode(_) => ("Unknown", Std),
            // K::Unknown => ("Unknown", Std),
            _ => ("Unknown", Std),
        }
    }

    /// Attempts to construct a `KeyFfi` from a JavaScript `KeyboardEvent`
    /// semantic [key] + a [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    // Returns `None` for unmappable codes to distinguish invalid JS input
    // from the internal `Key::Unknown` fallback.
    pub const fn from_js_key(key: &str, location: WebKeyLocation) -> Option<Self> {
        use KeyFfi as K;
        match key.as_bytes() {
            b"Backspace" => Some(K::Backspace),
            b"Enter" => Some(K::Enter),
            b"Tab" => Some(K::Tab),
            b"Escape" => Some(K::Escape),
            b" " => Some(K::Space),
            //
            b"ArrowLeft" => Some(K::Left),
            b"ArrowRight" => Some(K::Right),
            b"ArrowUp" => Some(K::Up),
            b"ArrowDown" => Some(K::Down),
            //
            b"Home" => Some(K::Home),
            b"End" => Some(K::End),
            b"PageUp" => Some(K::PageUp),
            b"PageDown" => Some(K::PageDown),
            //
            b"Delete" => Some(K::Delete),
            b"Insert" => Some(K::Insert),
            //
            b"CapsLock" => Some(K::CapsLock),
            b"ScrollLock" => Some(K::ScrollLock),
            b"NumLock" => Some(K::NumLock),
            //
            b"PrintScreen" => Some(K::PrintScreen),
            b"Pause" => Some(K::Pause),
            b"ContextMenu" => Some(K::Menu),
            // letters
            b"A" => Some(K::A), b"B" => Some(K::B), b"C" => Some(K::C), b"D" => Some(K::D),
            b"E" => Some(K::E), b"F" => Some(K::F), b"G" => Some(K::G), b"H" => Some(K::H),
            b"I" => Some(K::I), b"J" => Some(K::J), b"K" => Some(K::K), b"L" => Some(K::L),
            b"M" => Some(K::M), b"N" => Some(K::N), b"O" => Some(K::O), b"P" => Some(K::P),
            b"Q" => Some(K::Q), b"R" => Some(K::R), b"S" => Some(K::S), b"T" => Some(K::T),
            b"U" => Some(K::U), b"V" => Some(K::V), b"W" => Some(K::W), b"X" => Some(K::X),
            b"Y" => Some(K::Y), b"Z" => Some(K::Z),
            // digits
            b"0" => Some(K::Digit0), b"1" => Some(K::Digit1), b"2" => Some(K::Digit2),
            b"3" => Some(K::Digit3), b"4" => Some(K::Digit4), b"5" => Some(K::Digit5),
            b"6" => Some(K::Digit6), b"7" => Some(K::Digit7), b"8" => Some(K::Digit8),
            b"9" => Some(K::Digit9),
            _ => {
                // Try converting from inner enums
                if let Some(p) = KeyPad::from_js_key(key) { return Some(K::Pad(p)); }
                if let Some(m) = KeyMedia::from_js_key(key) { return Some(K::Media(m)); }
                if let Some(m) = KeyMod::from_js_key(key, location) { return Some(K::Mod(m)); }
                let kbytes = key.as_bytes();
                if let Some(first) = kbytes.first() {
                    // Check if the key starts with "F" (Function keys)
                    if *first == b'F' {
                        if let Some(n) = Self::parse_u8_from_bytes(Slice::range_from(kbytes, 1)) {
                            if n >= 1 && n <= 48 { return Some(K::Fn(n)); }
                        }
                    }
                    // Handle single UTF-8 characters (only meaningful for semantic keys)
                    let (code, _) = Char(kbytes).to_scalar_unchecked(0);
                    return Some(K::Char(code));
                }
                None
            }
        }
    }
    /// Returns a JavaScript `KeyboardEvent` semantic [key] + a [location]
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
    pub const fn to_js_key(self) -> (&'static str, WebKeyLocation) {
        use WebKeyLocation::{self as L, Standard as Std};
        use KeyFfi as K;
        match self {
            K::Backspace => ("Backspace", Std),
            K::Enter => ("Enter", Std),
            K::Tab => ("Tab", Std),
            K::Escape => ("Escape", Std),
            K::Space => (" ", Std),
            //
            K::Left => ("ArrowLeft", Std),
            K::Right => ("ArrowRight", Std),
            K::Up => ("ArrowUp", Std),
            K::Down => ("ArrowDown", Std),
            //
            K::Home => ("Home", Std),
            K::End => ("End", Std),
            K::PageUp => ("PageUp", Std),
            K::PageDown => ("PageDown", Std),
            //
            K::Delete => ("Delete", Std),
            K::Insert => ("Insert", Std),
            //
            K::CapsLock => ("CapsLock", Std),
            K::ScrollLock => ("ScrollLock", Std),
            K::NumLock => ("NumLock", Std),
            //
            K::PrintScreen => ("PrintScreen", Std),
            K::Pause => ("Pause", Std),
            K::Menu => ("ContextMenu", Std),
            // letters
            K::A => ("A", Std), K::B => ("B", Std), K::C => ("C", Std), K::D => ("D", Std),
            K::E => ("E", Std), K::F => ("F", Std), K::G => ("G", Std), K::H => ("H", Std),
            K::I => ("I", Std), K::J => ("J", Std), K::K => ("K", Std), K::L => ("L", Std),
            K::M => ("M", Std), K::N => ("N", Std), K::O => ("O", Std), K::P => ("P", Std),
            K::Q => ("Q", Std), K::R => ("R", Std), K::S => ("S", Std), K::T => ("T", Std),
            K::U => ("U", Std), K::V => ("V", Std), K::W => ("W", Std), K::X => ("X", Std),
            K::Y => ("Y", Std), K::Z => ("Z", Std),
            // numbers
            K::Digit0 => ("0", Std), K::Digit1 => ("1", Std), K::Digit2 => ("2", Std),
            K::Digit3 => ("3", Std), K::Digit4 => ("4", Std), K::Digit5 => ("5", Std),
            K::Digit6 => ("6", Std), K::Digit7 => ("7", Std), K::Digit8 => ("8", Std),
            K::Digit9 => ("9", Std),
            //
            K::Pad(pad) => (pad.to_js_key(), L::NumPad),
            K::Media(media) => (media.to_js_key(), Std),
            K::Mod(mod_key) => mod_key.to_js_key(),
            //
            K::Fn(num) => match num {
                1..=48 => (F_KEYS[(num - 1) as usize], Std),
                _ => ("Unknown", Std),
            },
            K::Char(c) => {
                // Returns the ASCII character, or Unknown otherwise
                is![Char(c).is_ascii(); return (Char(c).as_ascii_unchecked(), Std)];
                ("Unknown", Std)
                // IMPROVE using a static atomic buffer
                // (Char(c).to_utf8_str_unchecked(), Std)
            },
            // NOTE: Javascript has no concept of scan codes, so everything else is unknown
            // Key::Scancode(_) => ("Unknown", Std),
            // K::Unknown => ("Unknown", Std),
            _ => ("Unknown", Std),
        }
    }
}

#[cfg(test)]
#[cfg(all(feature = "js", not(windows)))]
mod tests {
    use super::*;
    use WebKeyLocation as L;

    /* physical */
    #[test]
    fn key_ffi_to_js_code() {
        assert_eq!(KeyFfi::Enter.to_js_code(), ("Enter", L::Standard));
        assert_eq!(KeyFfi::Space.to_js_code(), ("Space", L::Standard));
        assert_eq!(KeyFfi::Fn(5).to_js_code(), ("F5", L::Standard));
        assert_eq!(KeyFfi::Pad(KeyPad::Num0).to_js_code(), ("Numpad0", L::NumPad));
        assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("Unknown", L::Standard));
    }
    #[test]
    fn key_ffi_from_js_code() {
        assert_eq!(KeyFfi::from_js_code("Enter", L::Standard), Some(KeyFfi::Enter));
        assert_eq!(KeyFfi::from_js_code("F5", L::Standard), Some(KeyFfi::Fn(5)));
        assert_eq!(KeyFfi::from_js_code("Numpad0", L::NumPad), Some(KeyFfi::Pad(KeyPad::Num0)));
        assert_eq!(KeyFfi::from_js_code("Unknown", L::Standard), None);
    }
    /* semantic */
    #[test]
    fn key_ffi_to_js_key() {
        assert_eq!(KeyFfi::Enter.to_js_key(), ("Enter", L::Standard));
        assert_eq!(KeyFfi::Space.to_js_key(), (" ", L::Standard));
        assert_eq!(KeyFfi::Char('a' as u32).to_js_key(), ("a", L::Standard));
        assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("Unknown", L::Standard));
        // assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("é", L::Standard)); // IMPROVE
    }
    #[test]
    fn key_ffi_from_js_key() {
        assert_eq!(KeyFfi::from_js_key("Enter", L::Standard), Some(KeyFfi::Enter));
        assert_eq!(KeyFfi::from_js_key(" ", L::Standard), Some(KeyFfi::Space));
        assert_eq!(KeyFfi::from_js_key("a", L::Standard), Some(KeyFfi::Char('a' as u32)));
        assert_eq!(KeyFfi::from_js_key("é", L::Standard), Some(KeyFfi::Char('é' as u32)));
    }
}
