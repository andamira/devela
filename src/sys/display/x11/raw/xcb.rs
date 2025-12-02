// devela::sys::display::x11::raw::xcb
//
//! All from xcb.h and xproto.h.
//
// DOC LINKS:
// - https://www.x.org/releases/current/doc/man/man3/
// - https://xcb.freedesktop.org/manual/group__XCB____API.html
// - https://xkbcommon-d.dpldocs.info/~master/xkbcommon.html
//
// TOC
// * basic protocol scalar types
//   - xcb_window_t
//   - xcb_gcontext_t
//   - xcb_keycode_t
//   - xcb_timestamp_t
// * connection
//   - xcb_connection_t
//   - xcb_connect()
//   - xcb_connection_has_error()
//   - xcb_disconnect()
//   - xcb_flush()
//   - xcb_generate_id()
// * setup
//   - xcb_setup_t
//   - xcb_screen_t
//   - xcb_screen_iterator_t
//   - xcb_get_setup()
//   - xcb_setup_roots_iterator()
// * window/gc
//   - xcb_create_gc()
//   - xcb_create_window()
//   - xcb_map_window()
//   - xcb_change_property()
// * image/rectangle
//   - xcb_rectangle_t
//   - xcb_poly_fill_rectangle()
//   - xcb_put_image()
// * atom/intern
//   - xcb_void_cookie_t
//   - xcb_intern_atom_cookie_t
//   - xcb_intern_atom_reply_t
//   - xcb_intern_atom()
//   - xcb_intern_atom_reply()
// * error/event
//   - xcb_generic_error_t
//   - xcb_generic_event_t
//   - xcb_key_press_event_t
//   - xcb_poll_for_event()
//   - xcb_wait_for_event()

#![allow(non_camel_case_types)]

use crate::{_TAG_FFI, Extent, Position, c_char, c_int};

/* basic protocol scalar types */

/// X11 window identifier.
pub(crate) type xcb_window_t = u32;
/// X11 graphics context identifier.
pub(crate) type xcb_gcontext_t = u32;
/// X11 keycode (hardware scancode). Example: 24 is 'q' on US layout.
pub(crate) type xcb_keycode_t = u8;
/// X11 timestamp in milliseconds.
pub(crate) type xcb_timestamp_t = u32;

/* connection */

#[doc = _TAG_FFI!()]
/// - <https://xcb.freedesktop.org/manual/structxcb__connection__t.html>
#[repr(C)]
#[derive(Debug)]
pub(crate) struct xcb_connection_t {
    _private: [u8; 0],
}
#[link(name = "xcb")]
unsafe extern "C" {
    /// Connects to the X server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_connect>
    pub(crate) fn xcb_connect(display: *const c_char, screen: *mut c_int) -> *mut xcb_connection_t;

    /// Test whether the connection has shut down due to a fatal error.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_connection_has_error>
    pub(crate) fn xcb_connection_has_error(c: *mut xcb_connection_t) -> c_int;

    /// Closes the connection.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_disconnect>
    pub(crate) fn xcb_disconnect(c: *mut xcb_connection_t);

    /// Forces any buffered output to be written to the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_flush>
    pub(crate) fn xcb_flush(c: *mut xcb_connection_t) -> c_int;

    /// Allocates an XID for a new object.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_generate_id>
    pub(crate) fn xcb_generate_id(c: *mut xcb_connection_t) -> u32;
}

/* setup structs */

#[doc = _TAG_FFI!()]
/// Xcb setup.
/// - <https://xcb.freedesktop.org/manual/structxcb__setup__t.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct xcb_setup_t {
    pub status: u8,
    pad0: u8,
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub length: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub vendor_len: u16,
    pub maximum_request_length: u16,
    pub roots_len: u8,
    pub pixmap_formats_len: u8,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pad1: [u8; 4],
}

#[doc = _TAG_FFI!()]
/// A screen.
/// - <https://xcb.freedesktop.org/manual/structxcb__screen__t.html>
#[repr(C)]
#[derive(Debug)]
pub(crate) struct xcb_screen_t {
    pub root: xcb_window_t,
    pub default_colormap: u32, // xcb_colormap_t
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32, // xcb_visualid_t
    pub backing_stores: u8,
    pub save_unders: u8,
    pub root_depth: u8,
    pub allowed_depths_len: u8,
}

#[doc = _TAG_FFI!()]
/// A screen iterator.
///
/// Returned by [`xcb_setup_roots_iterator`].
/// - <https://xcb.freedesktop.org/manual/structxcb__screen__iterator__t.html>
#[repr(C)]
#[derive(Debug)]
pub(crate) struct xcb_screen_iterator_t {
    pub(in super::super) data: *mut xcb_screen_t,
    pub(in super::super) rem: c_int,
    pub(in super::super) index: c_int,
}

#[link(name = "xcb")]
unsafe extern "C" {
    /// Access the data returned by the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_get_setup>
    pub(crate) fn xcb_get_setup(c: *mut xcb_connection_t) -> *const xcb_setup_t;

    /// The iterator for the roots field inside the `xcb_setup_t` struct.
    /// - <https://xcb-d.dpldocs.info/xcb.xproto.xcb_setup_roots_iterator.html>
    pub(crate) fn xcb_setup_roots_iterator(setup: *const xcb_setup_t) -> xcb_screen_iterator_t;
}

/* window / graphics context */

#[link(name = "xcb")]
unsafe extern "C" {
    /// Creates a graphics context.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_create_gc.3.xhtml>
    pub(crate) fn xcb_create_gc(
        c: *mut xcb_connection_t,
        cid: xcb_gcontext_t,
        drawable: xcb_window_t,
        value_mask: u32,
        value_list: *const u32,
    );

    /// Creates a window.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_create_window.3.xhtml>
    pub(crate) fn xcb_create_window(
        c: *mut xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: u32,
        value_mask: u32,
        value_list: *const u32,
    );

    /// Makes a window visible.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_map_window.3.xhtml>
    pub(crate) fn xcb_map_window(c: *mut xcb_connection_t, window: xcb_window_t) -> u32;

    /// Changes a window property.
    ///
    /// Returns an xcb_void_cookie_t. Errors (if any) have to be handled in the event loop.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_change_property.3.xhtml>
    pub(crate) fn xcb_change_property(
        c: *mut xcb_connection_t,
        mode: u8,
        window: u32,
        property: u32,
        type_: u32,
        format: u8,
        data_len: u32,
        data: *const u8,
    );
}

/* image/rectangle */

#[doc = _TAG_FFI!()]
/// A rectangle.
/// - <https://xcb.freedesktop.org/manual/structxcb__rectangle__t.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct xcb_rectangle_t {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}
impl xcb_rectangle_t {
    pub fn new(pos: Position<i16, 2>, ext: Extent<u16, 2>) -> Self {
        Self {
            x: pos.x(),
            y: pos.y(),
            width: ext.w(),
            height: ext.h(),
        }
    }
}

#[link(name = "xcb")]
unsafe extern "C" {
    /// Fills rectangles.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_poly_fill_rectangle.3.xhtml>
    pub(crate) fn xcb_poly_fill_rectangle(
        c: *mut xcb_connection_t,
        drawable: xcb_window_t,
        gc: xcb_gcontext_t,
        rectangles_len: u32,
        rectangles: *const xcb_rectangle_t,
    );

    /// Puts an image.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_put_image.3.xhtml>.
    pub(crate) fn xcb_put_image(
        c: *mut xcb_connection_t,
        format: u8,
        drawable: xcb_window_t,
        gc: xcb_gcontext_t,
        width: u16,
        height: u16,
        dst_x: i16,
        dst_y: i16,
        left_pad: u8,
        depth: u8,
        data_len: u32,
        data: *const u8,
    );
}

/* atom/intern */

/// Cookie returned by `xcb_intern_atom`, used to retrieve the reply.
pub(crate) type xcb_intern_atom_cookie_t = u32;

#[doc = _TAG_FFI!()]
/// Generic cookie.
///
/// - <https://xcb.freedesktop.org/manual/structxcb__void__cookie__t.html>
#[repr(C)]
#[derive(Debug)]
pub(crate) struct xcb_void_cookie_t {
    sequence: u32,
}

#[doc = _TAG_FFI!()]
/// - <https://xcb.freedesktop.org/manual/structxcb__intern__atom__reply__t.html>
#[repr(C)]
#[derive(Debug)]
pub struct xcb_intern_atom_reply_t {
    pub response_type: u8,
    pub pad0: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: u32, // xcb_atom_t
}

#[link(name = "xcb")]
unsafe extern "C" {
    /// Get atom identifier by name.
    // - <https://www.x.org/releases/current/doc/man/man3/xcb_intern_atom.3.xhtml> LINK ERROR
    /// - <https://man.archlinux.org/man/xcb_intern_atom.3.en>
    pub(crate) fn xcb_intern_atom(
        c: *mut xcb_connection_t,
        only_if_exists: u8,
        name_len: u16,
        name: *const u8,
    ) -> xcb_intern_atom_cookie_t;

    /// Returns the reply of the request asked by.
    /// - <https://man.archlinux.org/man/xcb_intern_atom.3.en>
    pub(crate) fn xcb_intern_atom_reply(
        c: *mut xcb_connection_t,
        cookie: xcb_intern_atom_cookie_t,
        e: *mut *mut xcb_generic_error_t,
    ) -> *mut xcb_intern_atom_reply_t;
}

/* error / event structs */

#[doc = _TAG_FFI!()]
/// A generic error.
/// - <https://xcb.freedesktop.org/manual/structxcb__generic__error__t.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct xcb_generic_error_t {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub resource_id: u32,
    pub minor_code: u16,
    pub major_code: u8,
    pub pad0: u8,
    pub pad: [u32; 5],
    pub full_sequence: u32,
}

#[doc = _TAG_FFI!()]
/// A generic event.
/// - <https://xcb.freedesktop.org/manual/structxcb__generic__event__t.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct xcb_generic_event_t {
    pub response_type: u8,
    pad0: u8,
    pub sequence: u16,
    pub pad: [u32; 7],
    pub full_sequence: u32,
}

#[doc = _TAG_FFI!()]
/// A key was pressed/released.
/// - <https://xcb.freedesktop.org/manual/structxcb__key__press__event__t.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct xcb_key_press_event_t {
    /// event kind
    pub response_type: u8,
    /// keycode
    pub detail: xcb_keycode_t, // u8
    pub sequence: u16,
    pub time: xcb_timestamp_t, // u32
    pub root: xcb_window_t,    // u32
    pub event: xcb_window_t,   // u32
    pub child: xcb_window_t,   // u32
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    /// modifier mask
    pub state: u16,
    pub same_screen: u8,
    pad0: u8,
}

#[link(name = "xcb")]
unsafe extern "C" {
    /// Returns the next event or error from the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_poll_for_event>
    pub(crate) fn xcb_poll_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
    // xcb_poll_for_queued_event

    /// Returns the next event or error from the server.
    /// - <https://xcb.freedesktop.org/manual/group__XCB__Core__API.html#:~:text=xcb_wait_for_event>
    pub(crate) fn xcb_wait_for_event(c: *mut xcb_connection_t) -> *mut xcb_generic_event_t;
}
