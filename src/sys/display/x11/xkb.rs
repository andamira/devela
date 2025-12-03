// devela::sys::display::x11::xkb
//
//! Defines [`KeyRepeatFilter`], [`XkbInfo`], [`XkbState`].
//
// TOC
// - struct KeyRepeatFilter
// - struct XkbInfo
// - struct XkbState

use super::raw;
use crate::{Key, KeyDead, KeyMod, KeyMods, KeyPad, KeyState, XError, is};

/// Tracks the minimal state needed to classify `Press` vs `Repeat`.
///
/// This is stored in [`XDisplay`][crate::XDisplay], and used in [`crate::XEvent::to_event_key`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct KeyRepeatFilter {
    /// The last pressed keycode
    last_keycode: u8,
    /// Whether we are currently in a repeating cycle.
    repeating: bool,
}
#[rustfmt::skip]
impl KeyRepeatFilter {
    /// Constructs an idle repeat-filter with no key repeating.
    pub const fn new() -> Self { Self { last_keycode: 0, repeating: false } }

    /// Classifies an incoming `(keycode, is_press)` pair as `Press`, `Repeat`, or `Release`.
    ///
    /// Assumes synthetic X11 `(Release, Press)` autorepeat pairs have
    /// already been removed by [`classify_release`][super::XDisplay::classify_release].
    ///
    /// Rules:
    /// - A press of the same key while `repeating == true` → `Repeat`
    /// - Any new key press → `Press` and begin repeating for that key
    /// - Any release → `Release` and stop repeating
    #[inline(always)]
    pub fn filter(&mut self, keycode: u8, is_press: bool) -> KeyState {
        if is_press {
            // is this a repeated press of the same key?
            is![self.repeating && self.last_keycode == keycode; return KeyState::Repeat];
            // new press: start repeating for this key
            self.last_keycode = keycode;
            self.repeating = true;
            KeyState::Press
        } else {
            // a real release resets the repeating state
            self.repeating = false;
            KeyState::Release
        }
    }
}

/// Intermediate result combining semantic + physical keys + modifiers.
///
/// This is produced by [`XkbState::translate_key`] and consumed by [`crate::XEvent::to_event_key]`.
#[derive(Debug)]
pub(crate) struct XkbKeyInfo {
    pub(crate) semantic: Key,
    pub(crate) physical: Key,
    pub(crate) mods: KeyMods,
}
impl XkbKeyInfo {
    #[inline(always)]
    pub fn new(semantic: Key, physical: Key, mods: KeyMods) -> Self {
        Self { semantic, physical, mods }
    }
}

// WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
// #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
// struct XkbModIndices {
//     shift: u32,
//     lock: u32,
//     control: u32,
//     mod1: u32,
//     mod2: u32,
//     mod3: u32,
//     mod4: u32,
//     mod5: u32,
//     level3: u32,
//     level5: u32,
// }
// impl XkbModIndices {
//     fn new(keymap: *mut raw::xkb_keymap) -> Self {
//         Self {
//             shift:   Self::index(keymap, "Shift\0"),
//             lock:    Self::index(keymap, "Lock\0"),
//             control: Self::index(keymap, "Control\0"),
//             mod1:    Self::index(keymap, "Mod1\0"),
//             mod2:    Self::index(keymap, "Mod2\0"),
//             mod3:    Self::index(keymap, "Mod3\0"),
//             mod4:    Self::index(keymap, "Mod4\0"),
//             mod5:    Self::index(keymap, "Mod5\0"),
//             // level3:  Self::index(keymap, "LevelThree\0"),
//             // level5:  Self::index(keymap, "LevelFive\0"),
//         }
//     }
//     #[inline(always)]
//     fn index(keymap: *mut raw::xkb_keymap, name: &str) -> u32 {
//         unsafe { raw::xkb_keymap_mod_get_index(keymap, name.as_ptr().cast()) }
//     }
// }
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// struct XkbLedIndices {
//     caps: u32,
//     num: u32,
//     scroll: u32,
// }
// impl XkbLedIndices {
//     fn new(keymap: *mut raw::xkb_keymap) -> Self {
//         Self {
//             caps:   Self::index(keymap, "Caps Lock\0"),
//             num:    Self::index(keymap, "Num Lock\0"),
//             scroll: Self::index(keymap, "Scroll Lock\0"),
//         }
//     }
//     #[inline(always)]
//     fn index(keymap: *mut raw::xkb_keymap, name: &str) -> u32 {
//         unsafe { raw::xkb_keymap_led_get_index(keymap, name.as_ptr().cast()) }
//     }
// }

/// XKB translation state.
///
/// Provides keysym interpretation, physical-key identification,
/// and modifier mapping for X11 keyboard events.
///
/// This is an internal helper object used by the X11 backend.
/// It is not exposed publicly.
#[derive(Debug)]
pub(crate) struct XkbState {
    /// The XKB context.
    #[allow(dead_code, reason = "appears unused, but must outlive `keymap` and `state`")]
    ctx: *mut raw::xkb_context,

    /// Compiled keymap for the current keyboard device.
    ///
    /// Provides layout, symbol tables, physical positions, etc.
    keymap: *mut raw::xkb_keymap,

    /// Active XKB state for querying keysyms and modifiers.
    ///
    /// Updated implicitly according to X11 events.
    pub(crate) state: *mut raw::xkb_state,
    // // WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
    // /// Cached key modifier indices.
    // mod_idx: XkbModIndices,
    // /// Cached keyboard led indices.
    // led_idx: XkbLedIndices,
}

#[rustfmt::skip]
impl XkbState {
    /// Creates an XKB interpreter for the given X11 connection.
    ///
    /// Loads the keymap and constructs an internal XKB state for:
    /// - keysym translation,
    /// - physical scancode lookup,
    /// - modifier interpretation.
    ///
    /// Returns an [`XError`] if the XKB extension cannot be used.
    pub fn new(conn: *mut raw::xcb_connection_t) -> Result<Self, XError> {
        // context
        let ctx = unsafe { raw::xkb_context_new(0) };
        is![ctx.is_null(); return Err(XError::ExtensionUnavailable("xkb-context"))];

        // find core keyboard device
        let device_id = unsafe { raw::xkb_x11_get_core_keyboard_device_id(conn) };
        is![device_id < 0; return Err(XError::ExtensionUnavailable("xkb-core-keyboard"))];

        // load keymap from device
        let keymap = unsafe { raw::xkb_x11_keymap_new_from_device(ctx, conn, device_id, 0) };
        is![keymap.is_null(); return Err(XError::ExtensionUnavailable("xkb-keymap"))];

        // create XKB state
        let state = unsafe { raw::xkb_x11_state_new_from_device(keymap, conn, device_id) };
        if state.is_null() {
            unsafe { raw::xkb_keymap_unref(keymap) };
            return Err(XError::ExtensionUnavailable("xkb-state"));
        }

        // WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
        // cache indices of key modifiers and leds
        // let mod_idx = XkbModIndices::new(keymap);
        // let led_idx = XkbLedIndices::new(keymap);
        // Ok(Self { ctx, keymap, state, mod_idx, led_idx })

        Ok(Self { ctx, keymap, state })
    }

    /// Updates the internal XKB state with a key press or release.
    ///
    /// This applies the given `keycode` and direction (`Press` or `Release`)
    /// to the XKB state machine so that subsequent queries
    /// (`key_mods`, `key_semantic`, etc.) reflect the new modifier and group state.
    ///
    /// This is called from [`crate::XEvent::to_update_key`].
    pub fn update(&self, keycode: u8, dir: raw::xkb_key_direction ) {
        unsafe { raw::xkb_state_update_key(self.state, keycode as u32, dir); }
    }

    /// Translates an X11 keycode and modifier bitmask into semantic + physical keys.
    ///
    /// This is the high-level entry point used by the XEvent → EventKey conversion.
    // WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
    pub fn translate_key(&self, keycode: u8, mods: u16) -> XkbKeyInfo {
        XkbKeyInfo::new(self.key_semantic(keycode), self.key_physical(keycode), self.key_mods(mods))
        // XkbKeyInfo::new(self.key_semantic(keycode), self.key_physical(keycode), self.key_mods())
    }

    // WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
    // NOTE: https://github.com/xkbcommon/libxkbcommon/issues/583
    // #[inline(always)]
    // pub fn key_mods(&self) -> KeyMods {
    //     let mut m = KeyMods::empty();
    //     // basic modifiers
    //     if self.mod_active(self.mod_idx.shift)   { m.set_shift(); }
    //     if self.mod_active(self.mod_idx.control) { m.set_control(); }
    //     if self.mod_active(self.mod_idx.mod1)    { m.set_alt(); }
    //     if self.mod_active(self.mod_idx.mod4)    { m.set_super(); }
    //     // locks
    //     if self.mod_active(self.mod_idx.lock)    { m.set_caps_lock(); }
    //     if self.mod_active(self.mod_idx.mod2)    { m.set_num_lock(); }
    //     if self.mod_active(self.mod_idx.mod3)    { m.set_scroll_lock(); }
    //     // extended layers
    //     if self.mod_active(self.mod_idx.level3)  { m.set_alt_gr(); }
    //     if self.mod_active(self.mod_idx.level5)  { m.set_level5(); }
    //     m
    // }
    // helper to check effective modifier state
    // #[inline(always)]
    // fn mod_active(&self, idx: u32) -> bool {
    //     idx != raw::XKB_MOD_INVALID && unsafe {
    //         raw::xkb_state_mod_index_is_active(self.state, idx,
    //             raw::xkb_state_component::XKB_STATE_MODS_EFFECTIVE) } != 0
    // }

    /// Converts an X11 modifier bitmask into a [`KeyMods`] representation.
    pub fn key_mods(&self, xcb_modifiers: u16) -> KeyMods {
        let (x, mut m) = (xcb_modifiers, KeyMods::empty());
        if x & raw::XCB_MOD_MASK_SHIFT != 0 { m.set_shift(); }
        if x & raw::XCB_MOD_MASK_CONTROL  != 0 { m.set_control(); }
        if x & raw::XCB_MOD_MASK_LOCK != 0 { m.set_caps_lock(); }
        if x & raw::XCB_MOD_MASK_1 != 0 { m.set_alt(); }
        if x & raw::XCB_MOD_MASK_2 != 0 { m.set_num_lock(); }
        // if x & raw::XCB_MOD_MASK_3 != 0 { unimplemented![] }
        if x & raw::XCB_MOD_MASK_4 != 0 { m.set_super(); }
        if x & raw::XCB_MOD_MASK_5 != 0 { m.set_alt_gr(); }
        m
    }

    /// Returns the logical (layout-dependent) key for the given X11 keycode.
    ///
    /// This is the “semantic” key: the character or meaning according to the
    /// active keyboard layout (e.g., `'q'`, `'Q'`, `'é'`, `Left`, `F5`, etc.).
    #[inline(always)]
    pub fn key_semantic(&self, keycode: u8) -> Key {
        // try to get special keys via keysym (arrows, F1, Shift, etc.)
        let sym = unsafe { raw::xkb_state_key_get_one_sym(self.state, keycode as u32) };
        if let Some(key) = Self::map_special_keys(sym) { return key; }

        // try to get Unicode (ñ, ç, á, € …)
        let utf32 = unsafe { raw::xkb_state_key_get_utf32(self.state, keycode as u32) };
        if utf32 != 0 {
            #[cfg(any(feature = "safe_sys", not(feature = "unsafe_str")))]
            return Key::Char(char::from_u32(utf32).expect("valid unicode scalar"));

            #[cfg(all(not(feature = "safe_sys"), feature = "unsafe_str"))]
            unsafe { return Key::Char(char::from_u32_unchecked(utf32)); }
        }
        // fallback (dead keys etc.)
        Key::Unknown
    }

    /// Returns the physical key for the given X11 keycode.
    ///
    /// This is the “layout-independent” key: the actual physical scancode
    /// (e.g., “key in row 2, column 1”), stable across layouts.
    ///
    /// Useful for games and universal keybindings.
    #[inline(always)]
    pub fn key_physical(&self, keycode: u8) -> Key {
        // X11 core keycodes on Linux are usually evdev + 8.
        // We subtract 8 to get a layout-independent "physical" scancode.
        let scancode = keycode.saturating_sub(8) as u32;
        Self::map_scancode_to_key(scancode)
    }

    /* internals */

    // IMPROVE: add more LUTs like in dead key recognition, for faster conversion.
    /// Maps special keys before converting to UTF32 in [`XkbState::key_semantic`].
    #[inline(always)]
    const fn map_special_keys(sym: u32) -> Option<Key> {
        let k = match sym {
            /* control keys */
            raw::XKB_KEY_Return       => Key::Enter,
            raw::XKB_KEY_Tab          => Key::Tab,
            raw::XKB_KEY_BackSpace    => Key::Backspace,
            raw::XKB_KEY_Escape       => Key::Escape,
            raw::XKB_KEY_Delete       => Key::Delete,
            raw::XKB_KEY_KP_Delete    => Key::Delete,
            raw::XKB_KEY_Insert       => Key::Insert,
            raw::XKB_KEY_KP_Insert    => Key::Insert,

            /* navigation */
            raw::XKB_KEY_Left         => Key::Left,
            raw::XKB_KEY_Right        => Key::Right,
            raw::XKB_KEY_Up           => Key::Up,
            raw::XKB_KEY_Down         => Key::Down,
            raw::XKB_KEY_Home         => Key::Home,
            raw::XKB_KEY_End          => Key::End,
            raw::XKB_KEY_Page_Up      => Key::PageUp,
            raw::XKB_KEY_Page_Down    => Key::PageDown,

            /* lock keys */
            raw::XKB_KEY_Num_Lock     => Key::NumLock,
            raw::XKB_KEY_Caps_Lock    => Key::CapsLock,
            raw::XKB_KEY_Scroll_Lock  => Key::ScrollLock,

            /* dead keys */
            0xfe50 ..= 0xfe93 => Key::Dead(KeyDead::from_keysym(sym)),

            /* modifiers */
            raw::XKB_KEY_Shift_L      => Key::Mod(KeyMod::LeftShift),
            raw::XKB_KEY_Shift_R      => Key::Mod(KeyMod::RightShift),
            raw::XKB_KEY_Control_L    => Key::Mod(KeyMod::LeftControl),
            raw::XKB_KEY_Control_R    => Key::Mod(KeyMod::RightControl),
            raw::XKB_KEY_Alt_L        => Key::Mod(KeyMod::LeftAlt),
            raw::XKB_KEY_Alt_R        => Key::Mod(KeyMod::RightAlt),
            raw::XKB_KEY_Super_L      => Key::Mod(KeyMod::LeftSuper),
            raw::XKB_KEY_Super_R      => Key::Mod(KeyMod::RightSuper),
            0xfe03..=0xfe05           => Key::Mod(KeyMod::IsoLevel3Shift),
            raw::XKB_KEY_Mode_switch  => Key::Mod(KeyMod::AltGr),
            0xfe11..=0xfe13           => Key::Mod(KeyMod::IsoLevel5Shift),

            /* function keys */
            sym if (sym >= raw::XKB_KEY_F1 && sym <= raw::XKB_KEY_F35) => {
                let n = (sym - raw::XKB_KEY_F1 + 1) as u8;
                Key::Fn(n)
            }

            /* keypad numeric keys */
            raw::XKB_KEY_KP_0         => Key::Pad(KeyPad::Num0),
            raw::XKB_KEY_KP_1         => Key::Pad(KeyPad::Num1),
            raw::XKB_KEY_KP_2         => Key::Pad(KeyPad::Num2),
            raw::XKB_KEY_KP_3         => Key::Pad(KeyPad::Num3),
            raw::XKB_KEY_KP_4         => Key::Pad(KeyPad::Num4),
            raw::XKB_KEY_KP_5         => Key::Pad(KeyPad::Num5),
            raw::XKB_KEY_KP_6         => Key::Pad(KeyPad::Num6),
            raw::XKB_KEY_KP_7         => Key::Pad(KeyPad::Num7),
            raw::XKB_KEY_KP_8         => Key::Pad(KeyPad::Num8),
            raw::XKB_KEY_KP_9         => Key::Pad(KeyPad::Num9),
            raw::XKB_KEY_KP_Add       => Key::Pad(KeyPad::Add),
            raw::XKB_KEY_KP_Subtract  => Key::Pad(KeyPad::Subtract),
            raw::XKB_KEY_KP_Multiply  => Key::Pad(KeyPad::Multiply),
            raw::XKB_KEY_KP_Divide    => Key::Pad(KeyPad::Divide),
            raw::XKB_KEY_KP_Enter     => Key::Pad(KeyPad::Enter),
            raw::XKB_KEY_KP_Equal     => Key::Pad(KeyPad::Equal),
            raw::XKB_KEY_KP_Separator => Key::Pad(KeyPad::Comma),
            raw::XKB_KEY_KP_Decimal   => Key::Pad(KeyPad::Decimal),

            // no match => not a special key
            _ => return None,
        };
        Some(k)
    }

    /// Maps a physical scancode (“evdev code”) to a `Key`.
    ///
    /// Fallbacks to `Key::Unknown`.
    ///
    /// Maps:
    /// - common letter positions (scancode → KeyAlpha)
    /// - numeric row
    /// - keypad keys
    /// - basic system keys (Escape, Enter, etc.)
    #[inline(always)]
    const fn map_scancode_to_key(sc: u32) -> Key {
        // raw::SCANCODE_TO_KEY.get(sc as usize).copied().unwrap_or(Key::Unknown)
        raw::LUT_SCANCODE_TO_KEY[sc as usize]
    }
}

impl Drop for XkbState {
    fn drop(&mut self) {
        unsafe {
            raw::xkb_state_unref(self.state);
            raw::xkb_keymap_unref(self.keymap);
        }
    }
}
