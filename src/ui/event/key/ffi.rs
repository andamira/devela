// devela::ui::event::key::key
//
//! Defines [`EventKeyFfi`], [`KeyFfi`].
//
// TOC
// - struct EventKeyFfi
// - enum KeyFfi
// - static F_KEYS
// - impls `js`

use super::*;
use crate::{is, unwrap};
#[cfg(all(feature = "js", not(windows)))]
crate::items! {
    use crate::{Char, WebKeyLocation, Slice};
    crate::_use! {basic::from_utf8}
}

#[doc = crate::_TAG_FFI!()]
/// An FFI-safe version of [`EventKey`].
#[repr(C)]
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EventKeyFfi {
    #[doc = crate::_TAG_FFI!()]
    /// The key representing the human-readable code.
    pub semantic: KeyFfi,
    #[doc = crate::_TAG_FFI!()]
    /// The key representing the hardware scan code.
    pub physical: KeyFfi,
    /// The state of the key (pressed or released).
    pub state: KeyState,
    /// The active modifiers of the key (e.g., Shift, Ctrl).
    pub mods: KeyMods,
    #[doc = crate::_TAG_FFI!()]
    /// The time stamp of when the event occurred.
    // TODO IMPROVE: store the f32 bits as u32 & CHECK with web-api example
    pub time_stamp: f32,
}

impl EventKey {
    /// Converts `EventKey` to `EventKeyFfi`.
    pub const fn to_ffi(&self) -> EventKeyFfi {
        EventKeyFfi {
            semantic: self.semantic.to_ffi(),
            physical: self.physical.to_ffi(),
            state: self.state,
            mods: self.mods,
            time_stamp: if let Some(t) = self.time_stamp { t.as_millis_f32() } else { 0.0 },
        }
    }
    /// Converts `EventKeyFfi` to `EventKey`.
    pub const fn from_ffi(from: &EventKeyFfi) -> EventKey {
        EventKey {
            semantic: Key::from_ffi(from.semantic),
            physical: Key::from_ffi(from.physical),
            state: from.state,
            mods: from.mods,
            time_stamp: Some(EventTimestamp::from_millis_f32(from.time_stamp)),
        }
    }
}
crate::items! {
    impl From<&EventKey> for EventKeyFfi { fn from(e: &EventKey) -> Self { EventKey::to_ffi(e) } }
    impl From<&EventKeyFfi> for EventKey { fn from(e: &EventKeyFfi) -> Self { Self::from_ffi(e) } }
    impl From<EventKey> for EventKeyFfi { fn from(e: EventKey) -> Self { EventKey::to_ffi(&e) } }
    impl From<EventKeyFfi> for EventKey { fn from(e: EventKeyFfi) -> Self { Self::from_ffi(&e) } }
}

#[doc = crate::_TAG_FFI!()]
/// An FFI-safe version of [`Key`], used in [`EventKeyFfi`][crate::EventKeyFfi].
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyFfi {
    Backspace, Enter, Tab, Escape, Space,
    Left, Right, Up, Down,
    Home, End, PageUp, PageDown,
    Delete, Insert,
    CapsLock, ScrollLock, NumLock,
    PrintScreen, Pause, Menu,
    /// Function keys (F1-F48).
    F(u8),
    /// An alphanumeric key (A-Z, 0-9).
    Alpha(KeyAlpha),
    #[doc = crate::_TAG_FFI!()]
    /// A unicode character (FFI-safe, stored as `u32`).
    Char(u32),
    /// A keypad key.
    Pad(KeyPad),
    /// A multimedia key.
    Media(KeyMedia),
    /// A modifier key.
    Mod(KeyMod),
    #[default]
    Unknown,
}

impl Key {
    /// Converts `Key` to `KeyFfi`.
    pub const fn to_ffi(self) -> KeyFfi {
        use {Key as K, KeyFfi as F};
        match self {
            K::Backspace => F::Backspace,
            K::Enter => F::Enter,
            K::Tab => F::Tab,
            K::Escape => F::Escape,
            K::Space => F::Space,
            K::Left => F::Left,
            K::Right => F::Right,
            K::Up => F::Up,
            K::Down => F::Down,
            K::Home => F::Home,
            K::End => F::End,
            K::PageUp => F::PageUp,
            K::PageDown => F::PageDown,
            K::Delete => F::Delete,
            K::Insert => F::Insert,
            K::CapsLock => F::CapsLock,
            K::ScrollLock => F::ScrollLock,
            K::NumLock => F::NumLock,
            K::PrintScreen => F::PrintScreen,
            K::Pause => F::Pause,
            K::Menu => F::Menu,
            K::F(f) => F::F(f),
            K::Alpha(a) => F::Alpha(a),
            K::Char(c) => F::Char(c as u32), // Convert char to u32
            K::Pad(p) => F::Pad(p),
            K::Media(m) => F::Media(m),
            K::Mod(m) => F::Mod(m),
            K::Unknown => F::Unknown,
        }
    }

    /// Converts `KeyFfi` to `Key`.
    pub const fn from_ffi(from: KeyFfi) -> Key {
        use {Key as K, KeyFfi as F};
        match from {
            F::Backspace => K::Backspace,
            F::Enter => K::Enter,
            F::Tab => K::Tab,
            F::Escape => K::Escape,
            F::Space => K::Space,
            F::Left => K::Left,
            F::Right => K::Right,
            F::Up => K::Up,
            F::Down => K::Down,
            F::Home => K::Home,
            F::End => K::End,
            F::PageUp => K::PageUp,
            F::PageDown => K::PageDown,
            F::Delete => K::Delete,
            F::Insert => K::Insert,
            F::CapsLock => K::CapsLock,
            F::ScrollLock => K::ScrollLock,
            F::NumLock => K::NumLock,
            F::PrintScreen => K::PrintScreen,
            F::Pause => K::Pause,
            F::Menu => K::Menu,
            F::F(f) => K::F(f),
            F::Alpha(a) => K::Alpha(a),
            F::Char(c) => K::Char(unwrap![some_or char::from_u32(c), '?']), // IMPROVE: ?
            F::Pad(p) => K::Pad(p),
            F::Media(m) => K::Media(m),
            F::Mod(m) => K::Mod(m),
            F::Unknown => K::Unknown,
        }
    }
}
crate::items! {
    impl From<Key> for KeyFfi { fn from(k: Key) -> Self { Key::to_ffi(k) } }
    impl From<KeyFfi> for Key { fn from(k: KeyFfi) -> Self { Key::from_ffi(k) } }
}

#[cfg(all(feature = "js", not(windows)))]
pub(crate) static F_KEYS: [&str; 48] = [
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12", "F13", "F14", "F15",
    "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24", "F25", "F26", "F27", "F28",
    "F29", "F30", "F31", "F32", "F33", "F34", "F35", "F36", "F37", "F38", "F39", "F40", "F41",
    "F42", "F43", "F44", "F45", "F46", "F47", "F48",
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

    /// Attempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// physical [code] + a [location].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
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
            _ => {
                // Try converting from inner enums
                if let Some(alpha) = KeyAlpha::from_js_code(code) {
                    return Some(K::Alpha(alpha));
                }
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
                            if n >= 1 && n <= 48 { return Some(K::F(n)); }
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
        use KeyFfi as K;
        use WebKeyLocation as L;
        match self {
            K::Backspace => ("Backspace", L::Standard),
            K::Enter => ("Enter", L::Standard),
            K::Tab => ("Tab", L::Standard),
            K::Escape => ("Escape", L::Standard),
            K::Space => ("Space", L::Standard),
            //
            K::Left => ("ArrowLeft", L::Standard),
            K::Right => ("ArrowRight", L::Standard),
            K::Up => ("ArrowUp", L::Standard),
            K::Down => ("ArrowDown", L::Standard),
            //
            K::Home => ("Home", L::Standard),
            K::End => ("End", L::Standard),
            K::PageUp => ("PageUp", L::Standard),
            K::PageDown => ("PageDown", L::Standard),
            //
            K::Delete => ("Delete", L::Standard),
            K::Insert => ("Insert", L::Standard),
            //
            K::CapsLock => ("CapsLock", L::Standard),
            K::ScrollLock => ("ScrollLock", L::Standard),
            K::NumLock => ("NumLock", L::Standard),
            //
            K::PrintScreen => ("PrintScreen", L::Standard),
            K::Pause => ("Pause", L::Standard),
            K::Menu => ("ContextMenu", L::Standard),
            //
            K::Alpha(alpha) => (alpha.to_js_code(), L::Standard),
            K::Pad(pad) => (pad.to_js_code(), L::NumPad),
            K::Media(media) => (media.to_js_code(), L::Standard),
            K::Mod(mod_key) => mod_key.to_js_code(),
            //
            K::F(num) => match num {
                1..=48 => (F_KEYS[(num - 1) as usize], L::Standard),
                _ => ("Unknown", L::Standard),
            },
            K::Char(c) => {
                // Returns the ASCII character, or Unknown otherwise
                is![Char(c).is_ascii(); return (Char(c).as_ascii_unchecked(), L::Standard)];
                ("Unknown", L::Standard)
            },
            K::Unknown => ("Unknown", L::Standard),
        }
    }

    /// Attempts to construct a `KeyMod` from a JavaScript `KeyboardEvent`
    /// semantic [key] + a [location].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    /// [location]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location
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
            _ => {
                // Try converting from inner enums
                if let Some(a) = KeyAlpha::from_js_key(key) { return Some(K::Alpha(a)); }
                if let Some(p) = KeyPad::from_js_key(key) { return Some(K::Pad(p)); }
                if let Some(m) = KeyMedia::from_js_key(key) { return Some(K::Media(m)); }
                if let Some(m) = KeyMod::from_js_key(key, location) { return Some(K::Mod(m)); }
                let kbytes = key.as_bytes();
                if let Some(first) = kbytes.first() {
                    // Check if the key starts with "F" (Function keys)
                    if *first == b'F' {
                        if let Some(n) = Self::parse_u8_from_bytes(Slice::range_from(kbytes, 1)) {
                            if n >= 1 && n <= 48 { return Some(K::F(n)); }
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
        use KeyFfi as K;
        use WebKeyLocation as L;
        match self {
            K::Backspace => ("Backspace", L::Standard),
            K::Enter => ("Enter", L::Standard),
            K::Tab => ("Tab", L::Standard),
            K::Escape => ("Escape", L::Standard),
            K::Space => (" ", L::Standard),
            //
            K::Left => ("ArrowLeft", L::Standard),
            K::Right => ("ArrowRight", L::Standard),
            K::Up => ("ArrowUp", L::Standard),
            K::Down => ("ArrowDown", L::Standard),
            //
            K::Home => ("Home", L::Standard),
            K::End => ("End", L::Standard),
            K::PageUp => ("PageUp", L::Standard),
            K::PageDown => ("PageDown", L::Standard),
            //
            K::Delete => ("Delete", L::Standard),
            K::Insert => ("Insert", L::Standard),
            //
            K::CapsLock => ("CapsLock", L::Standard),
            K::ScrollLock => ("ScrollLock", L::Standard),
            K::NumLock => ("NumLock", L::Standard),
            //
            K::PrintScreen => ("PrintScreen", L::Standard),
            K::Pause => ("Pause", L::Standard),
            K::Menu => ("ContextMenu", L::Standard),
            //
            K::Alpha(alpha) => (alpha.to_js_key(), L::Standard),
            K::Pad(pad) => (pad.to_js_key(), L::NumPad),
            K::Media(media) => (media.to_js_key(), L::Standard),
            K::Mod(mod_key) => mod_key.to_js_key(),
            //
            K::F(num) => match num {
                1..=48 => (F_KEYS[(num - 1) as usize], L::Standard),
                _ => ("Unknown", L::Standard),
            },
            K::Char(c) => {
                // Returns the ASCII character, or Unknown otherwise
                is![Char(c).is_ascii(); return (Char(c).as_ascii_unchecked(), L::Standard)];
                ("Unknown", L::Standard)
                // IMPROVE using a static atomic buffer
                // (Char(c).to_utf8_str_unchecked(), L::Standard)
            },
            K::Unknown => ("Unknown", L::Standard),
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
        assert_eq!(KeyFfi::F(5).to_js_code(), ("F5", L::Standard));
        assert_eq!(KeyFfi::Pad(KeyPad::Num0).to_js_code(), ("Numpad0", L::NumPad));
        assert_eq!(KeyFfi::Char('é' as u32).to_js_key(), ("Unknown", L::Standard));
    }
    #[test]
    fn key_ffi_from_js_code() {
        assert_eq!(KeyFfi::from_js_code("Enter", L::Standard), Some(KeyFfi::Enter));
        assert_eq!(KeyFfi::from_js_code("F5", L::Standard), Some(KeyFfi::F(5)));
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
