// devela::sys::display::x11::raw::fns
//
//!
//
// TOC
// - xcb
//   - free
//   - xcb_connect
//   - xcb_connection_has_error
//   - xcb_get_setup
//   - xcb_setup_roots_iterator
//   - xcb_generate_id
//   - xcb_create_gc
//   - xcb_create_window
//   - xcb_map_window
//   - xcb_flush
//   - xcb_wait_for_event
//   - xcb_disconnect
//   - xcb_poly_fill_rectangle
//   - xcb_put_image
// - xcb-shm
//   - xcb_shm_attach_fd
//   - xcb_shm_put_image
// - xkb
//
// DOC LINKS:
// - https://www.x.org/releases/current/doc/man/man3/
// - https://xcb.freedesktop.org/manual/group__XCB____API.html
// - https://xkbcommon-d.dpldocs.info/~master/xkbcommon.html

#![allow(dead_code, non_camel_case_types)]

use super::{
    xcb_connection_t, xcb_gcontext_t, xcb_generic_error_t, xcb_generic_event_t,
    xcb_intern_atom_cookie_t, xcb_intern_atom_reply_t, xcb_rectangle_t, xcb_screen_iterator_t,
    xcb_setup_t, xcb_shm_seg_t, xcb_void_cookie_t, xcb_window_t,
};
use super::{xkb_context, xkb_key_direction, xkb_keymap, xkb_state, xkb_state_component};
use crate::{c_char, c_int};

#[rustfmt::skip]
#[link(name = "xcb")]
unsafe extern "C" {
    /// Connects to the X server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_connect>
    pub(crate) fn xcb_connect(display: *const c_char, screen: *mut c_int) -> *mut xcb_connection_t;

    /// Test whether the connection has shut down due to a fatal error.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_connection_has_error>
    pub(crate) fn xcb_connection_has_error(c: *mut xcb_connection_t) -> c_int;

    /// Access the data returned by the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_get_setup>
    pub(crate) fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;

    /// The iterator for the roots field inside the `xcb_setup_t` struct.
    /// - <https://xcb-d.dpldocs.info/xcb.xproto.xcb_setup_roots_iterator.html>
    pub(crate) fn xcb_setup_roots_iterator(setup: *const xcb_setup_t) -> xcb_screen_iterator_t;

    /// Allocates an XID for a new object.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_generate_id>
    pub(crate) fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;

    /// Creates a graphics context.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_create_gc.3.xhtml>
    pub(crate) fn xcb_create_gc(c: *mut xcb_connection_t, cid: xcb_gcontext_t,
        drawable: xcb_window_t, value_mask: u32, value_list: *const u32);

    /// Creates a window.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_create_window.3.xhtml>
    pub(crate) fn xcb_create_window(c: *mut xcb_connection_t, depth: u8, wid: xcb_window_t,
        parent: xcb_window_t, x: i16, y: i16, width: u16, height: u16, border_width: u16,
        class: u16, visual: u32, value_mask: u32, value_list: *const u32);

    /// Makes a window visible.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_map_window.3.xhtml>
    pub(crate) fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> u32;

    /// Forces any buffered output to be written to the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_flush>
    pub(crate) fn xcb_flush(c: *mut xcb_connection_t) -> c_int;

    /// Returns the next event or error from the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_poll_for_event>
    pub(crate) fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    // xcb_poll_for_queued_event

    /// Returns the next event or error from the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_wait_for_event>
    pub(crate) fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;

    /// Closes the connection.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_disconnect>
    pub(crate) fn xcb_disconnect(c: *mut xcb_connection_t);

    /// Fills rectangles.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_poly_fill_rectangle.3.xhtml>
    pub(crate) fn xcb_poly_fill_rectangle(c: *mut xcb_connection_t, drawable: xcb_window_t,
        gc: xcb_gcontext_t, rectangles_len: u32, rectangles: *const xcb_rectangle_t);

    /// Puts an image.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_put_image.3.xhtml>.
    pub(crate) fn xcb_put_image(c: *mut xcb_connection_t, format: u8, drawable: xcb_window_t,
        gc: xcb_gcontext_t, width: u16, height: u16, dst_x: i16, dst_y: i16,
        left_pad: u8, depth: u8, data_len: u32, data: *const u8);

    /// Changes a window property.
    ///
    /// Returns an xcb_void_cookie_t. Errors (if any) have to be handled in the event loop.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_change_property.3.xhtml>
    pub(crate) fn xcb_change_property(c: *mut xcb_connection_t, mode: u8, window: u32,
        property: u32, type_: u32, format: u8, data_len: u32, data: *const u8);

    /// Get atom identifier by name.
    // - <https://www.x.org/releases/current/doc/man/man3/xcb_intern_atom.3.xhtml> FAILS
    /// - <https://man.archlinux.org/man/xcb_intern_atom.3.en>
    pub(crate) fn xcb_intern_atom(c: *mut xcb_connection_t, only_if_exists: u8, name_len: u16,
        name: *const u8) -> xcb_intern_atom_cookie_t;

    /// Returns the reply of the request asked by.
    /// - <https://man.archlinux.org/man/xcb_intern_atom.3.en>
    pub(crate) fn xcb_intern_atom_reply(c: *mut xcb_connection_t, cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t) -> *mut xcb_intern_atom_reply_t;
}

#[rustfmt::skip]
#[link(name = "xcb-shm")]
unsafe extern "C" {
    /// Attaches a shared-memory segment to the X server using a file descriptor.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_attach.3.xhtml>
    pub(crate) fn xcb_shm_attach_fd(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t, shm_fd: c_int,
        read_only: u8) -> xcb_void_cookie_t;

    /// Puts an image into a drawable using shared memory.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_put_image.3.xhtml>
    pub(crate) fn xcb_shm_put_image(c: *mut xcb_connection_t, drawable: xcb_window_t,
        gc: xcb_gcontext_t, total_width: u16, total_height: u16, src_x: u16, src_y: u16,
        src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8,
        send_event: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t;
}

#[rustfmt::skip]
#[link(name = "xkbcommon")]
#[link(name = "xkbcommon-x11")]
// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.html>
// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.xkbcommon.html>
// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.x11.html>
unsafe extern "C" {
    /* XKB common methods */

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

    /* XKB X11 methods */

    /// Setup the XKB X11 extension for this X client.
    ///
    /// Returns 1 on success, or 0 on failure.
    /// - <https://xkbcommon-d.dpldocs.info/~master/xkbcommon.x11.xkb_x11_setup_xkb_extension.html>
    pub(crate) fn xkb_x11_setup_xkb_extension(conn: *mut xcb_connection_t, major_xkb_version: u16,
        minor_xkb_version: u16, flags: u32, out_major: *mut u16, out_minor: *mut u16,
        out_base_ev: *mut u8, out_base_err: *mut u8) -> c_int;

    /// Get the device ID for the core keyboard
    pub(crate) fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> i32;

    /// Create keymap from X11 device
    pub(crate) fn xkb_x11_keymap_new_from_device(context: *mut xkb_context,
        connection: *mut xcb_connection_t, device_id: i32, flags: u32) -> *mut xkb_keymap;

    /// Create state from X11 device
    pub(crate) fn xkb_x11_state_new_from_device(keymap: *mut xkb_keymap,
        connection: *mut xcb_connection_t, device_id: i32) -> *mut xkb_state;
}
