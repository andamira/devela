// devela::ui::event::key::dif
//
//! Defines [`KeyMod`], [`KeyMods`].
//
// TOC
// - enum KeyMod
// - struct KeyMods

#![allow(unused, missing_docs)] // WIP

use crate::{ConstInit, impl_trait, is, set};

/* definitions */

#[doc = crate::_tags!(interaction member)]
/// Modifier key codes (when pressed by themselves)
#[doc = crate::_doc_meta!{location("ui/event")}]
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

set! {
    #[doc = crate::_tags!(interaction set)]
    /// A bitfield of key modifiers (Shift, Control…) + extra (repeating, composing).
    #[doc = crate::_doc_meta!{
        location("ui/event"),
        test_size_of(KeyMods = 2|16), // option = 4|32
    }]
    #[repr(transparent)]
    pub struct KeyMods(u16) {
        /// The *Control* modifier.
        CONTROL = 0;
        /// The *Shift* modifier.
        SHIFT = 1;
        /// The *Alt* modifier.
        ALT = 2;
        /// The *Super* modifier.
        SUPER = 3;
        /// The *AltGraph* modifier.
        ALT_GR = 4;
        /// The *Caps Lock* modifier.
        CAPS_LOCK = 5;
        /// The *Num Lock* modifier.
        NUM_LOCK = 6;
        /// The *Scroll Lock* modifier.
        SCROLL_LOCK = 7;
        /// The *IsoLevel5Shift* modifier.
        LEVEL5 = 8;
        /// Whether a key event is repeating.
        REPEATING = 9;
        /// Whether a key event is composing (IME input).
        COMPOSING = 10;
    }
    traits(!Debug);
}
impl_trait! { fmt::Debug for KeyMods |self, f| {
    let c = is![self.has_control(), "C", "-"];
    let s = is![self.has_shift(), "S", "-"];
    let a = is![self.has_alt(), "A", "-"];
    let g = is![self.has_alt_gr(), "G", "-"];
    let u = is![self.has_super(), "U", "-"];
    let l = is![self.has_caps_lock(), "L", "-"];
    let n = is![self.has_num_lock(), "N", "-"];
    let r = is![self.has_repeating(), "R", "-"];
    let p = is![self.has_composing(), "P", "-"];
    write![f, "{c}{s}{a}{g}{u}{l}{n}{r}{p}"]
} }
