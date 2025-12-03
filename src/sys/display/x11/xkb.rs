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
/// This is produced by [`XkbState::translate_key`] and consumed by the
/// XEvent → EventKey conversion layer.
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
        Ok(Self { ctx, keymap, state })
    }

    // TODO
    ///
    pub(crate) fn update(&self, keycode: u8, dir: raw::xkb_key_direction ) {
        unsafe { raw::xkb_state_update_key(self.state, keycode as u32, dir); }
    }

    /// Translates an X11 keycode and modifier bitmask into semantic + physical keys.
    ///
    /// This is the high-level entry point used by the XEvent → EventKey conversion.
    pub fn translate_key(&self, keycode: u8, mods: u16) -> XkbKeyInfo {
        XkbKeyInfo::new(self.key_semantic(keycode), self.key_physical(keycode), self.key_mods(mods))
    }

    /// Converts an X11 modifier bitmask into a [`KeyMods`] representation.
    #[inline(always)]
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
        // Subtract 8 to get a layout-independent “physical” scancode.
        let scancode = keycode.saturating_sub(8) as u32;
        Self::map_scancode_to_key(scancode)
    }

    /* internals */

    #[inline(always)]
    fn map_special_keys(sym: u32) -> Option<Key> {
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
