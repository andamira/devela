// devela::sys::display::x11::raw::xkb
//
//!
//
// TOC
// - constants
// - opaque types
// - enums
// - state

#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::{_TAG_FFI, c_int, c_void};

/* constants */
/* keysyms — from xkbcommon keysyms.h */

pub(crate) const XKB_KEY_Shift_L: u32 = 0xffe1;
pub(crate) const XKB_KEY_Shift_R: u32 = 0xffe2;
pub(crate) const XKB_KEY_Control_L: u32 = 0xffe3;
pub(crate) const XKB_KEY_Control_R: u32 = 0xffe4;
pub(crate) const XKB_KEY_Caps_Lock: u32 = 0xffe5;
pub(crate) const XKB_KEY_Shift_Lock: u32 = 0xffe6;
pub(crate) const XKB_KEY_Meta_L: u32 = 0xffe7;
pub(crate) const XKB_KEY_Meta_R: u32 = 0xffe8;
pub(crate) const XKB_KEY_Alt_L: u32 = 0xffe9;
pub(crate) const XKB_KEY_Alt_R: u32 = 0xffea;
pub(crate) const XKB_KEY_Super_L: u32 = 0xffeb;
pub(crate) const XKB_KEY_Super_R: u32 = 0xffec;
pub(crate) const XKB_KEY_Hyper_L: u32 = 0xffed;
pub(crate) const XKB_KEY_Hyper_R: u32 = 0xffee;
pub(crate) const XKB_KEY_ISO_Level3_Shift: u32 = 0xfe03;
pub(crate) const XKB_KEY_ISO_Level5_Shift: u32 = 0xfe11;
pub(crate) const XKB_KEY_Mode_switch: u32 = 0xff7e;

/* opaque types */

/// Opaque XKB context handle.
pub(crate) enum xkb_context {}
/// Opaque XKB keymap handle.
pub(crate) enum xkb_keymap {}
/// Opaque XKB state handle.
pub(crate) enum xkb_state {}

/* enums */

#[doc = _TAG_FFI!()]
/// Direction of a key event.
#[repr(C)]
pub(crate) enum xkb_key_direction {
    /// Key release.
    XKB_KEY_UP = 0,
    /// Key press.
    XKB_KEY_DOWN = 1,
}

/* state */

#[doc = _TAG_FFI!()]
/// Modifier and layout types for state objects
///
/// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_component.html>
#[repr(C)]
pub(crate) enum xkb_state_component {
    /// Depressed modifiers.
    /// A key is physically holding them.
    XKB_STATE_MODS_DEPRESSED = (1 << 0),
    /// Latched modifiers.
    /// Will be unset after the next non-modifier key press.
    XKB_STATE_MODS_LATCHED = (1 << 1),
    /// Locked modifiers.
    /// Will be unset after the key provoking the lock has been pressed again.
    XKB_STATE_MODS_LOCKED = (1 << 2),
    /// Effective modifiers.
    /// Currently active and affect key processing (derived from the other state components).
    /// Use this unless you explictly care how the state came about.
    XKB_STATE_MODS_EFFECTIVE = (1 << 3),
    /// Depressed layout.
    /// A key is physically holding it.
    XKB_STATE_LAYOUT_DEPRESSED = (1 << 4),
    /// Latched layout.
    /// Will be unset after the next non-modifier key press.
    XKB_STATE_LAYOUT_LATCHED = (1 << 5),
    /// Locked layout.
    /// Will be unset after the key provoking the lock has been pressed again.
    XKB_STATE_LAYOUT_LOCKED = (1 << 6),
    /// Effective layout.
    /// Currently active and affects key processing (derived from the other state components).
    /// Use this unless you explictly care how the state came about.
    XKB_STATE_LAYOUT_EFFECTIVE = (1 << 7),
    /// LEDs (derived from the other state components).
    XKB_STATE_LEDS = (1 << 8),
}

#[rustfmt::skip]
#[link(name = "xkbcommon")]
// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.html>
// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.html>
unsafe extern "C" {
    /// Create a new context.
    ///
    /// Returns a new context, or NULL on failure.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_context_new.html>
    pub(crate) fn xkb_context_new(flags: u32) -> *mut xkb_context;

    /// Release a reference on a keymap, and possibly free it.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_keymap_unref.html>
    pub(crate) fn xkb_keymap_unref(keymap: *mut xkb_keymap);

    /// Release a reference on a keybaord state object, and possibly free it.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_unref.html>
    pub(crate) fn xkb_state_unref(state: *mut xkb_state);

    /// Get the single keysym obtained from pressing a particular key in a given keyboard state.
    ///
    /// Returns The keysym. If the key does not have exactly one keysym, returns XKB_KEY_NoSymbol.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_key_get_one_sym.html>
    pub(crate) fn xkb_state_key_get_one_sym(state: *mut xkb_state, keycode: u32) -> u32;

    /// Test whether a modifier is active in a given keyboard state by index.
    ///
    /// Returns 1 if the modifier is active, 0 if it is not.
    /// If the modifier index is invalid in the keymap, returns -1.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_mod_index_is_active.html>
    pub(crate) fn xkb_state_mod_index_is_active(state: *mut xkb_state, idx: u32,
        ty: xkb_state_component) -> c_int;

    /// Update the keyboard state to reflect a given key being pressed or released.
    ///
    /// Returns A mask of state components that have changed as a result of the update.
    /// If nothing in the state has changed, returns 0.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_update_key.html>
    pub(crate) fn xkb_state_update_key(state: *mut xkb_state, key: u32, direction: xkb_key_direction)
        -> xkb_state_component;

    /// Get the Unicode codepoint obtained from pressing a particular key in a a given keyboard state.
    ///
    /// Returns the UTF-32 representation for the key, if it consists of only a single codepoint.
    /// Otherwise, returns 0.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_key_get_utf32.html>
    pub(crate) fn xkb_state_key_get_utf32(state: *mut xkb_state, key: u32) -> u32;

    // MAYBE
    // /// Get the UTF-8 string obtained from pressing a particular key in a given keyboard state.
    // ///
    // /// Returns The number of bytes required for the string, excluding the NUL byte.
    // /// If the buffer passed is too small, the string is truncated
    // /// If there is nothing to write, returns 0.
    // /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.xkb_state_key_get_utf8.html>
    // pub(crate) fn xkb_state_key_get_utf8(state: *mut xkb_state, key: u32, buf *mut u8, len: usize) -> u32;
}
