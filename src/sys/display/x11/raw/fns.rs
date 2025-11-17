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

#![allow(dead_code)]
#![allow(non_camel_case_types)]

use super::{
    xcb_connection_t, xcb_gcontext_t, xcb_generic_event_t, xcb_rectangle_t, xcb_screen_iterator_t,
    xcb_setup_t, xcb_shm_seg_t, xcb_void_cookie_t, xcb_window_t,
};
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
