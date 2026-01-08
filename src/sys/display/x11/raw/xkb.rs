// devela::sys::display::x11::raw::xkb
//
//!
//
// TOC
// - keysyms
// - opaque types
// - enums
// - state

#![allow(non_camel_case_types, non_upper_case_globals, clippy::upper_case_acronyms)]

use crate::{c_char, c_int, c_void};

/* from xkbcommon.h */

pub(crate) const XKB_KEYCODE_INVALID: u32 = 0xffff_ffff;
pub(crate) const XKB_LAYOUT_INVALID: u32 = 0xffff_ffff;
pub(crate) const XKB_LEVEL_INVALID: u32 = 0xffff_ffff;
pub(crate) const XKB_MOD_INVALID: u32 = 0xffff_ffff;
pub(crate) const XKB_LED_INVALID: u32 = 0xffff_ffff;
pub(crate) const XKB_KEYCODE_MAX: u32 = 0xffff_ffff - 1;

/* xkbcommon-keysyms.h */

// TTY function keys
pub(crate) const XKB_KEY_BackSpace: u32 = 0xff08;
pub(crate) const XKB_KEY_Tab: u32 = 0xff09;
pub(crate) const XKB_KEY_Linefeed: u32 = 0xff0a;
pub(crate) const XKB_KEY_Clear: u32 = 0xff0b;
pub(crate) const XKB_KEY_Return: u32 = 0xff0d;
pub(crate) const XKB_KEY_Pause: u32 = 0xff13;
pub(crate) const XKB_KEY_Scroll_Lock: u32 = 0xff14;
pub(crate) const XKB_KEY_Sys_Req: u32 = 0xff15;
pub(crate) const XKB_KEY_Escape: u32 = 0xff1b;
pub(crate) const XKB_KEY_Delete: u32 = 0xffff;

// Cursor control & motion
pub(crate) const XKB_KEY_Home: u32 = 0xff50;
pub(crate) const XKB_KEY_Left: u32 = 0xff51;
pub(crate) const XKB_KEY_Up: u32 = 0xff52;
pub(crate) const XKB_KEY_Right: u32 = 0xff53;
pub(crate) const XKB_KEY_Down: u32 = 0xff54;
pub(crate) const XKB_KEY_Prior: u32 = 0xff55;
pub(crate) const XKB_KEY_Page_Up: u32 = 0xff55;
pub(crate) const XKB_KEY_Next: u32 = 0xff56;
pub(crate) const XKB_KEY_Page_Down: u32 = 0xff56;
pub(crate) const XKB_KEY_End: u32 = 0xff57;
pub(crate) const XKB_KEY_Begin: u32 = 0xff58;

// Misc. functions
pub(crate) const XKB_KEY_Select: u32 = 0xff60;
pub(crate) const XKB_KEY_Print: u32 = 0xff61;
pub(crate) const XKB_KEY_Execute: u32 = 0xff62;
pub(crate) const XKB_KEY_Insert: u32 = 0xff63;
pub(crate) const XKB_KEY_Undo: u32 = 0xff65;
pub(crate) const XKB_KEY_Redo: u32 = 0xff66;
pub(crate) const XKB_KEY_Menu: u32 = 0xff67;
pub(crate) const XKB_KEY_Find: u32 = 0xff68;
pub(crate) const XKB_KEY_Cancel: u32 = 0xff69;
pub(crate) const XKB_KEY_Help: u32 = 0xff6a;
pub(crate) const XKB_KEY_Break: u32 = 0xff6b;
pub(crate) const XKB_KEY_Mode_switch: u32 = 0xff7e;
// pub(crate) const XKB_KEY_script_switch: u32 = 0xff7e;
pub(crate) const XKB_KEY_Num_Lock: u32 = 0xff7f;

// Keypad functions
pub(crate) const XKB_KEY_KP_Space: u32 = 0xff80;
pub(crate) const XKB_KEY_KP_Tab: u32 = 0xff89;
pub(crate) const XKB_KEY_KP_Enter: u32 = 0xff8d;
pub(crate) const XKB_KEY_KP_F1: u32 = 0xff91;
pub(crate) const XKB_KEY_KP_F2: u32 = 0xff92;
pub(crate) const XKB_KEY_KP_F3: u32 = 0xff93;
pub(crate) const XKB_KEY_KP_F4: u32 = 0xff94;
pub(crate) const XKB_KEY_KP_Home: u32 = 0xff95;
pub(crate) const XKB_KEY_KP_Left: u32 = 0xff96;
pub(crate) const XKB_KEY_KP_Up: u32 = 0xff97;
pub(crate) const XKB_KEY_KP_Right: u32 = 0xff98;
pub(crate) const XKB_KEY_KP_Down: u32 = 0xff99;
pub(crate) const XKB_KEY_KP_Prior: u32 = 0xff9a;
pub(crate) const XKB_KEY_KP_Page_Up: u32 = 0xff9a;
pub(crate) const XKB_KEY_KP_Next: u32 = 0xff9b;
pub(crate) const XKB_KEY_KP_Page_Down: u32 = 0xff9b;
pub(crate) const XKB_KEY_KP_End: u32 = 0xff9c;
pub(crate) const XKB_KEY_KP_Begin: u32 = 0xff9d;
pub(crate) const XKB_KEY_KP_Insert: u32 = 0xff9e;
pub(crate) const XKB_KEY_KP_Delete: u32 = 0xff9f;
pub(crate) const XKB_KEY_KP_Equal: u32 = 0xffbd;
pub(crate) const XKB_KEY_KP_Multiply: u32 = 0xffaa;
pub(crate) const XKB_KEY_KP_Add: u32 = 0xffab;
pub(crate) const XKB_KEY_KP_Separator: u32 = 0xffac;
pub(crate) const XKB_KEY_KP_Subtract: u32 = 0xffad;
pub(crate) const XKB_KEY_KP_Decimal: u32 = 0xffae;
pub(crate) const XKB_KEY_KP_Divide: u32 = 0xffaf;
pub(crate) const XKB_KEY_KP_0: u32 = 0xffb0;
pub(crate) const XKB_KEY_KP_1: u32 = 0xffb1;
pub(crate) const XKB_KEY_KP_2: u32 = 0xffb2;
pub(crate) const XKB_KEY_KP_3: u32 = 0xffb3;
pub(crate) const XKB_KEY_KP_4: u32 = 0xffb4;
pub(crate) const XKB_KEY_KP_5: u32 = 0xffb5;
pub(crate) const XKB_KEY_KP_6: u32 = 0xffb6;
pub(crate) const XKB_KEY_KP_7: u32 = 0xffb7;
pub(crate) const XKB_KEY_KP_8: u32 = 0xffb8;
pub(crate) const XKB_KEY_KP_9: u32 = 0xffb9;

// Auxiliary functions
pub(crate) const XKB_KEY_F1: u32 = 0xffbe;
// …
pub(crate) const XKB_KEY_F35: u32 = 0xffe0;

// Modifiers
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

// XKB extension function and modifier keys (partially)
// AltGr
pub(crate) const XKB_KEY_ISO_Level3_Shift: u32 = 0xfe03;
pub(crate) const XKB_KEY_ISO_Level3_Latch: u32 = 0xfe04;
pub(crate) const XKB_KEY_ISO_Level3_Lock: u32 = 0xfe05;
// IsoLevel5Shift
pub(crate) const XKB_KEY_ISO_Level5_Shift: u32 = 0xfe11;
pub(crate) const XKB_KEY_ISO_Level5_Latch: u32 = 0xfe12;
pub(crate) const XKB_KEY_ISO_Level5_Lock: u32 = 0xfe13;

/* opaque types */

/// Opaque XKB context handle.
pub(crate) enum xkb_context {}
/// Opaque XKB keymap handle.
pub(crate) enum xkb_keymap {}
/// Opaque XKB state handle.
pub(crate) enum xkb_state {}

/* enums */

#[doc = crate::_tags!(ffi)]
/// Direction of a key event.
#[repr(C)]
pub(crate) enum xkb_key_direction {
    /// Key release.
    XKB_KEY_UP = 0,
    /// Key press.
    XKB_KEY_DOWN = 1,
}

/* state */

#[doc = crate::_tags!(ffi)]
/// Modifier and layout types for state objects
///
/// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_component.html>
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
// - <https://xkbcommon-d.dpldocs.info/xkbcommon.html>
// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.html>
unsafe extern "C" {
    /// Create a new context.
    ///
    /// Returns a new context, or NULL on failure.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_context_new.html>
    pub(crate) fn xkb_context_new(flags: u32) -> *mut xkb_context;

    /// Release a reference on a keymap, and possibly free it.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_keymap_unref.html>
    pub(crate) fn xkb_keymap_unref(keymap: *mut xkb_keymap);

    /// Get the index of a modifier by name.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_keymap_mod_get_index.html>
    pub(crate) fn xkb_keymap_mod_get_index(keymap: *mut xkb_keymap, name: *const c_char) -> u32;

    /// Get the index of a LED by name.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_keymap_led_get_index.html>
    pub(crate) fn xkb_keymap_led_get_index(keymap: *mut xkb_keymap, name: *const c_char) -> u32;

    /// Release a reference on a keybaord state object, and possibly free it.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_unref.html>
    pub(crate) fn xkb_state_unref(state: *mut xkb_state);

    /// Get the single keysym obtained from pressing a particular key in a given keyboard state.
    ///
    /// Returns The keysym. If the key does not have exactly one keysym, returns XKB_KEY_NoSymbol.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_key_get_one_sym.html>
    pub(crate) fn xkb_state_key_get_one_sym(state: *mut xkb_state, keycode: u32) -> u32;

    /// Test whether a modifier is active in a given keyboard state by index.
    ///
    /// Returns 1 if the modifier is active, 0 if it is not.
    /// If the modifier index is invalid in the keymap, returns -1.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_mod_index_is_active.html>
    pub(crate) fn xkb_state_mod_index_is_active(state: *mut xkb_state, idx: u32,
        ty: xkb_state_component) -> c_int;

    /// Update the keyboard state to reflect a given key being pressed or released.
    ///
    /// Returns A mask of state components that have changed as a result of the update.
    /// If nothing in the state has changed, returns 0.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_update_key.html>
    pub(crate) fn xkb_state_update_key(state: *mut xkb_state, key: u32, direction: xkb_key_direction)
        -> xkb_state_component;

    /// Get the Unicode codepoint obtained from pressing a particular key in a a given keyboard state.
    ///
    /// Returns the UTF-32 representation for the key, if it consists of only a single codepoint.
    /// Otherwise, returns 0.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_key_get_utf32.html>
    pub(crate) fn xkb_state_key_get_utf32(state: *mut xkb_state, key: u32) -> u32;

    // MAYBE
    // /// Get the UTF-8 string obtained from pressing a particular key in a given keyboard state.
    // ///
    // /// Returns The number of bytes required for the string, excluding the NUL byte.
    // /// If the buffer passed is too small, the string is truncated
    // /// If there is nothing to write, returns 0.
    // /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.xkbcommon.xkb_state_key_get_utf8.html>
    // pub(crate) fn xkb_state_key_get_utf8(state: *mut xkb_state, key: u32, buf *mut u8, len: usize) -> u32;
}
