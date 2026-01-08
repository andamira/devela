// devela::ui::event::key::dif
//
//! Defines [`KeyMod`], [`KeyMods`].
//
// TOC
// - enum KeyMod
// - struct KeyMods
// - impls

#![allow(unused, missing_docs)] // WIP

#[cfg(all(feature = "js", not(windows)))]
use crate::WebKeyLocation;
use crate::{ConstInit, impl_trait, is};

/* definitions */

#[doc = crate::_tags!(interaction)]
/// Modifier key codes (when pressed by themselves)
#[doc = crate::_doc_location!("ui/event")]
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
    /// Left **Control** (Control) key.
    LeftControl,
    /// Left **Alt** key.
    LeftAlt,
    /// Left **Super** key (Windows key on Windows, Command ⌘ on macOS).
    LeftSuper,
    /// Right **Shift** key.
    RightShift,
    /// Right **Control** key.
    RightControl,
    /// Right **Alt** key.
    RightAlt,
    /// Right **Super** key (Windows key on Windows, Command ⌘ on macOS).
    RightSuper,
    /// Used to access alternative characters on some keyboards.
    ///
    /// Also known as *ISO Level 3 Shift*.
    AltGr,
    /// **ISO Level 5 Shift** key (used in some advanced keyboard layouts).
    IsoLevel5Shift,
}
impl ConstInit for KeyMod {
    const INIT: Self = Self::LeftShift;
}
#[allow(non_upper_case_globals)]
impl KeyMod {
    /// AltGr key.
    pub const IsoLevel3Shift: KeyMod = KeyMod::AltGr;
}

#[doc = crate::_tags!(interaction)]
/// A bitfield of key modifiers (Shift, Control…) + extra (repeating, composing).
#[doc = crate::_doc_location!("ui/event")]
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyMods(u16);
impl ConstInit for KeyMods {
    const INIT: Self = Self(0);
}
impl_trait! { fmt::Debug for KeyMods |self, f| {
    let c = is![self.has_control(); "C"; "-"];
    let s = is![self.has_shift(); "S"; "-"];
    let a = is![self.has_alt(); "A"; "-"];
    let g = is![self.has_alt_gr(); "G"; "-"];
    let u = is![self.has_super(); "U"; "-"];
    let l = is![self.has_caps_lock(); "L"; "-"];
    let n = is![self.has_num_lock(); "N"; "-"];
    let r = is![self.is_repeating(); "R"; "-"];
    let p = is![self.is_composing(); "P"; "-"];
    write![f, "{c}{s}{a}{g}{u}{l}{n}{r}{p}"]
} }
#[rustfmt::skip]
impl KeyMods {
    pub(crate) const CONTROL: u16 = 1 << 0;
    pub(crate) const SHIFT: u16 = 1 << 1;
    pub(crate) const ALT: u16 = 1 << 2;
    pub(crate) const SUPER: u16 = 1 << 3;
    pub(crate) const ALT_GRAPH: u16 = 1 << 4;
    pub(crate) const CAPS_LOCK: u16 = 1 << 5;
    pub(crate) const NUM_LOCK: u16 = 1 << 6;
    pub(crate) const SCROLL_LOCK: u16 = 1 << 7;
    pub(crate) const LEVEL5: u16 = 1 << 8;
    pub(crate) const REPEATING: u16 = 1 << 9;
    pub(crate) const COMPOSING: u16 = 1 << 10;

    /// Constructs an empty `KeyMods`.
    pub const fn empty() -> Self { Self::INIT }

    /// Constructs a `KeyMods` from a bitfield representation.
    pub const fn from_bits(bits: u16) -> Self { Self(bits) }

    /// Returns a bitfield representation.
    pub const fn to_bits(self) -> u16 { self.0 }

    // /// Checks if a given modifier is active.
    // pub const fn has(self, mod_mask: u16) -> bool { self.0 & mod_mask != 0 }
    /// Checks if the *Control* modifier is set.
    pub const fn has_control(self) -> bool { self.0 & Self::CONTROL != 0 }
    /// Checks if the *Shift* modifier is set.
    pub const fn has_shift(self) -> bool { self.0 & Self::SHIFT != 0 }
    /// Checks if the *Alt* modifier is set.
    pub const fn has_alt(self) -> bool { self.0 & Self::ALT != 0 }
    /// Checks if the *AltGraph* modifier is set.
    pub const fn has_alt_gr(self) -> bool { self.0 & Self::ALT_GRAPH != 0 }
    /// Checks if the *Super* modifier is set.
    pub const fn has_super(self) -> bool { self.0 & Self::SUPER != 0 }
    /// Checks if the *Caps Lock* modifier is set.
    pub const fn has_caps_lock(self) -> bool { self.0 & Self::CAPS_LOCK != 0 }
    /// Checks if the *Num Lock* modifier is set.
    pub const fn has_num_lock(self) -> bool { self.0 & Self::NUM_LOCK != 0 }
    /// Checks if the *Scroll Lock* modifier is set.
    pub const fn has_scroll_lock(self) -> bool { self.0 & Self::SCROLL_LOCK != 0 }
    /// Checks if the *IsoLevel5Shift* modifier is set.
    pub const fn has_level5(self) -> bool { self.0 & Self::LEVEL5 != 0 }

    /// Queries if a key event is repeating.
    pub const fn is_repeating(self) -> bool { self.0 & Self::REPEATING != 0 }
    /// Queries if a key event is composing (IME input).
    pub const fn is_composing(self) -> bool { self.0 & Self::COMPOSING != 0 }

    /* setters */

    /// Sets the *Control* modifier.
    pub const fn set_control(&mut self) { self.0 |= Self::CONTROL; }
    /// Sets the *Shift* modifier.
    pub const fn set_shift(&mut self) { self.0 |= Self::SHIFT; }
    /// Sets the *Alt* modifier.
    pub const fn set_alt(&mut self) { self.0 |= Self::ALT; }
    /// Sets the *AltGraph* modifier.
    pub const fn set_alt_gr(&mut self) { self.0 |= Self::ALT_GRAPH; }
    /// Sets the *Super* modifier.
    pub const fn set_super(&mut self) { self.0 |= Self::SUPER; }
    /// Sets the *Caps Lock* modifier.
    pub const fn set_caps_lock(&mut self) { self.0 |= Self::CAPS_LOCK; }
    /// Sets the *Num Lock* modifier.
    pub const fn set_num_lock(&mut self) { self.0 |= Self::NUM_LOCK; }
    /// Sets the *Scroll Lock* modifier.
    pub const fn set_scroll_lock(&mut self) { self.0 |= Self::SCROLL_LOCK; }
    /// Sets the *IsoLevel5Shift* modifier.
    pub const fn set_level5(&mut self) { self.0 |= Self::LEVEL5; }
    /// Sets the repeating modifier.
    pub const fn set_repeating(&mut self) { self.0 |= Self::REPEATING; }
    /// Sets the composing modifier.
    pub const fn set_composing(&mut self) { self.0 |= Self::COMPOSING; }

    /// Unsets the *Control* modifier.
    pub const fn unset_control(&mut self) { self.0 &= !Self::CONTROL; }
    /// Unsets the *Shift* modifier.
    pub const fn unset_shift(&mut self) { self.0 &= !Self::SHIFT; }
    /// Unsets the *Alt* modifier.
    pub const fn unset_alt(&mut self) { self.0 &= !Self::ALT; }
    /// Unsets the *AltGraph* modifier.
    pub const fn unset_alt_gr(&mut self) { self.0 &= !Self::ALT_GRAPH; }
    /// Unsets the *Super* modifier.
    pub const fn unset_super(&mut self) { self.0 &= !Self::SUPER; }
    /// Unsets the *Caps Lock* modifier.
    pub const fn unset_caps_lock(&mut self) { self.0 &= !Self::CAPS_LOCK; }
    /// Unsets the *Num Lock* modifier.
    pub const fn unset_num_lock(&mut self) { self.0 &= !Self::NUM_LOCK; }
    /// Unsets the *Scroll Lock* modifier.
    pub const fn unset_scroll_lock(&mut self) { self.0 &= !Self::SCROLL_LOCK; }
    /// Unsets the *IsoLevel5Shift* modifier.
    pub const fn unset_level5(&mut self) { self.0 &= !Self::LEVEL5; }
    /// Unsets the repeating modifier.
    pub const fn unset_repeating(&mut self) { self.0 &= !Self::REPEATING; }
    /// Unsets the composing modifier.
    pub const fn unset_composing(&mut self) { self.0 &= !Self::COMPOSING; }
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
