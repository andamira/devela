// devela::ui::event::key::dif
//
//! Defines [`KeyMod`], [`KeyMods`].
//
// TOC
// - enum KeyMod
// - struct KeyMods
// - impls

use crate::ConstInit;
#[cfg(all(feature = "js", not(windows)))]
use crate::WebKeyLocation;

/* definitions */

/// Modifier key codes (when pressed by themselves)
///
/// These keys modify the behavior of other keys when held down.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.ModifierKey.html
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum KeyMod {
    /// Left **Shift** key.
    LeftShift,
    /// Left **Control** (Ctrl) key.
    LeftControl,
    /// Left **Alt** key.
    LeftAlt,
    /// Left **Super** key (Windows key on Windows, Command ⌘ on macOS).
    LeftSuper,
    // obsolete:
    // /// Left **Hyper** key (historically used in some Unix systems).
    // LeftHyper,
    // /// Left **Meta** key (used in some Unix-based systems, overlaps with Super).
    // LeftMeta,
    /// Right **Shift** key.
    RightShift,
    /// Right **Control** (Ctrl) key.
    RightControl,
    /// Right **Alt** key.
    RightAlt,
    /// Right **Super** key (Windows key on Windows, Command ⌘ on macOS).
    RightSuper,
    // obsolete:
    // /// Right **Hyper** key (historically used in some Unix systems).
    // RightHyper,
    // /// Right **Meta** key (used in some Unix-based systems, overlaps with Super).
    // RightMeta,
    /// Used to access alternative characters on some keyboards.
    ///
    /// Also known as *ISO Level 3 Shift*.
    AltGr,
    /// **ISO Level 5 Shift** key (used in some advanced keyboard layouts).
    IsoLevel5Shift,
}
#[allow(non_upper_case_globals)]
impl KeyMod {
    /// AltGr key.
    pub const IsoLevel3Shift: KeyMod = KeyMod::AltGr;
}
impl ConstInit for KeyMod {
    const INIT: Self = Self::LeftShift;
}

/// A bitfield of keys modifiers (Shift, Ctrl…) + extra (repeating, composing).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyMods(u16);
#[rustfmt::skip]
impl KeyMods {
    const CTRL: u16 = 1 << 0;
    const SHIFT: u16 = 1 << 1;
    const ALT: u16 = 1 << 2;
    const SUPER: u16 = 1 << 3;
    const ALT_GRAPH: u16 = 1 << 4;
    const CAPS_LOCK: u16 = 1 << 5;
    const NUM_LOCK: u16 = 1 << 6;
    const SCROLL_LOCK: u16 = 1 << 7;
    const REPEAT: u16 = 1 << 8;
    const IS_COMPOSING: u16 = 1 << 9;

    /// Constructs a `KeyMods` from a bitfield representation.
    pub const fn from_bits(bits: u16) -> Self { Self(bits) }

    // /// Checks if a given modifier is active.
    // pub const fn has(self, mod_mask: u16) -> bool { self.0 & mod_mask != 0 }
    /// Checks if the *Control* modifier is set.
    pub const fn has_ctrl(self) -> bool { self.0 & Self::CTRL != 0 }
    /// Checks if the *Shift* modifier is set.
    pub const fn has_shift(self) -> bool { self.0 & Self::SHIFT != 0 }
    /// Checks if the *Alt* modifier is set.
    pub const fn has_alt(self) -> bool { self.0 & Self::ALT != 0 }
    /// Checks if the *AltGraph* modifier is set.
    pub const fn has_alt_gr(self) -> bool { self.0 & Self::ALT_GRAPH != 0 }
    /// Checks if the *Meta* modifier is set.
    pub const fn has_super(self) -> bool { self.0 & Self::SUPER != 0 }
    /// Checks if the *Caps Lock* modifier is set.
    pub const fn has_caps_lock(self) -> bool { self.0 & Self::CAPS_LOCK != 0 }
    /// Checks if the *Num Lock* modifier is set.
    pub const fn has_num_lock(self) -> bool { self.0 & Self::NUM_LOCK != 0 }
    /// Checks if the *Scroll Lock* modifier is set.
    pub const fn has_scroll_lock(self) -> bool { self.0 & Self::SCROLL_LOCK != 0 }

    /// Queries if a key event is a repeat.
    pub const fn is_repeating(self) -> bool { self.0 & Self::REPEAT != 0 }
    /// Queries if a key event is composing (IME input).
    pub const fn is_composing(self) -> bool { self.0 & Self::IS_COMPOSING != 0 }
}
impl ConstInit for KeyMods {
    const INIT: Self = Self(0);
}

/* impls */

#[rustfmt::skip]
#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
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
            (b"MetaLeft", L::Left) => Some(K::LeftSuper), //
            // (b"HyperLeft", L::Left) => Some(K::LeftHyper), //
            // (b"OSLeft", L::Left) => Some(K::LeftMeta), //
            (b"ShiftRight", L::Right) => Some(K::RightShift),
            (b"ControlRight", L::Right) => Some(K::RightControl),
            (b"AltRight", L::Right) => Some(K::RightAlt),
            // (b"MetaRight", L::Right) => Some(K::RightSuper), //
            // (b"HyperRight", L::Right) => Some(K::RightHyper), //
            // (b"OSRight", L::Right) => Some(K::RightMeta), //
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
            K::LeftSuper => ("MetaLeft", L::Left), //
            // K::LeftHyper => ("HyperLeft", L::Left), //
            // K::LeftMeta => ("OSLeft", L::Left), //
            K::RightShift => ("ShiftRight", L::Right),
            K::RightControl => ("ControlRight", L::Right),
            K::RightAlt => ("AltRight", L::Right),
            K::RightSuper => ("MetaRight", L::Right), //
            // K::RightHyper => ("HyperRight", L::Right), //
            // K::RightMeta => ("OSRight", L::Right), //
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
            // (b"Hyper", L::Left) => Some(K::LeftHyper),
            // (b"OS", L::Left) => Some(K::LeftMeta),
            (b"Shift", L::Right) => Some(K::RightShift),
            (b"Control", L::Right) => Some(K::RightControl),
            (b"Alt", L::Right) => Some(K::RightAlt),
            (b"Meta", L::Right) => Some(K::RightSuper),
            // (b"Hyper", L::Right) => Some(K::RightHyper),
            // (b"OS", L::Right) => Some(K::RightMeta),
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
            // K::LeftHyper => ("Hyper", L::Left),
            // K::LeftMeta => ("OS", L::Left),
            K::RightShift => ("Shift", L::Right),
            K::RightControl => ("Control", L::Right),
            K::RightAlt => ("Alt", L::Right),
            K::RightSuper => ("Meta", L::Right),
            // K::RightHyper => ("Hyper", L::Right),
            // K::RightMeta => ("OS", L::Right),
            K::AltGr => ("AltGraph", L::Standard),
            K::IsoLevel5Shift => ("Level5Shift", L::Standard),
        }
    }
}
