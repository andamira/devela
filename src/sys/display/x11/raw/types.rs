// devela::sys::display::x11::raw::types
//
//! raw type aliases and structs from xcb.h and xproto.h.
//
// TOC
// - type aliases
//   - xcb_window_t
//   - xcb_gcontext_t
//   - xcb_shm_seg_t
//   - xcb_keycode_t
//   - xcb_timestamp_t
// - type structs
//   - xcb_void_cookie_t
//   - xcb_connection_t
//   - xcb_setup_t
//   - xcb_screen_t
//   - xcb_screen_iterator_t
//   - xcb_generic_event_t
//   - xcb_key_press_event_t
//   - xcb_rectangle_t

#![allow(dead_code, missing_docs, non_camel_case_types, clippy::upper_case_acronyms)]

use crate::{_TAG_FFI, Extent, Position, c_int, c_void};

/* type aliases: xcb */

/// X11 window identifier.
pub(crate) type xcb_window_t = u32;
/// X11 graphics context identifier.
pub(crate) type xcb_gcontext_t = u32;
/// X11 shared-memory segment identifier.
pub(crate) type xcb_shm_seg_t = u32;

/// X11 keycode (hardware scancode). Example: 24 is 'q' on US layout.
pub(crate) type xcb_keycode_t = u8;

/// X11 timestamp in milliseconds.
pub(crate) type xcb_timestamp_t = u32;

/// Cookie returned by `xcb_intern_atom`, used to retrieve the reply.
pub(crate) type xcb_intern_atom_cookie_t = u32;

/// XKB key direction (press or release).
pub(crate) type xkb_key_direction = u32;

/* xkb opaque types */

/// Opaque XKB context handle.
pub(crate) enum xkb_context {}
/// Opaque XKB keymap handle.
pub(crate) enum xkb_keymap {}
/// Opaque XKB state handle.
pub(crate) enum xkb_state {}

/* Structs */

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
/// - <https://xcb.freedesktop.org/manual/structxcb__connection__t.html>
#[repr(C)]
#[derive(Debug)]
pub(crate) struct xcb_connection_t {
    _private: [u8; 0],
}

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
