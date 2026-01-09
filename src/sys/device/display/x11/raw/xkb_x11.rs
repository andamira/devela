// devela::sys::display::x11::raw::xkb_x11
//
//! Everything from libxkbcommon-x11.so
//

use crate::{c_int, xcb_connection_t, xkb_context, xkb_keymap, xkb_state};

#[link(name = "xkbcommon-x11")]
// - <https://xkbcommon-d.dpldocs.info/xkbcommon.x11.html>
unsafe extern "C" {
    /// Setup the XKB X11 extension for this X client.
    ///
    /// Returns 1 on success, or 0 on failure.
    /// - <https://xkbcommon-d.dpldocs.info/xkbcommon.x11.xkb_x11_setup_xkb_extension.html>
    pub(crate) fn xkb_x11_setup_xkb_extension(
        conn: *mut xcb_connection_t,
        major_xkb_version: u16,
        minor_xkb_version: u16,
        flags: u32,
        out_major: *mut u16,
        out_minor: *mut u16,
        out_base_ev: *mut u8,
        out_base_err: *mut u8,
    ) -> c_int;

    /// Get the device ID for the core keyboard
    pub(crate) fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> i32;

    /// Create keymap from X11 device
    pub(crate) fn xkb_x11_keymap_new_from_device(
        context: *mut xkb_context,
        connection: *mut xcb_connection_t,
        device_id: i32,
        flags: u32,
    ) -> *mut xkb_keymap;

    /// Create state from X11 device
    pub(crate) fn xkb_x11_state_new_from_device(
        keymap: *mut xkb_keymap,
        connection: *mut xcb_connection_t,
        device_id: i32,
    ) -> *mut xkb_state;
}
