// devela::sys::display::x11::raw::consts
//
//! Constants from `xproto.h`
//
// TOC
// - xcb constants
// - xkb constants

#![allow(dead_code, missing_docs, non_upper_case_globals)]

use super::super::raw;

/* xcb */

/// X11 event mask: exposure events.
pub(crate) const XCB_EVENT_MASK_EXPOSURE: u32 = 1 << 15;
/// X11 event mask: key press events.
pub(crate) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;
/// X11 event mask: key release events.
pub(crate) const XCB_EVENT_MASK_KEY_RELEASE: u32 = 2;
pub(crate) const XCB_EVENT_MASK_STRUCTURE_NOTIFY: u32 = 0x20000;

/// X11 event code: exposure event.
pub(crate) const XCB_EXPOSE: u8 = 12;
/// X11 event code: key press event.
pub(crate) const XCB_KEY_PRESS: u8 = 2;
/// X11 event code: key press event.
pub(crate) const XCB_KEY_RELEASE: u8 = 3;

pub const XKB_KEY_UP: raw::xkb_key_direction = 0;
pub const XKB_KEY_DOWN: raw::xkb_key_direction = 1;

/* control keys */
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

/// X11 window attribute: event mask.
pub(crate) const XCB_CW_EVENT_MASK: u32 = 1 << 11;

//

// always this ID on all servers
pub(crate) const XCB_ATOM_WM_PROTOCOLS: u32 = 68;

///
pub(crate) const XCB_PROP_MODE_REPLACE: u8 = 0;
///
pub(crate) const XCB_PROP_MODE_PREPEND: u8 = 1;
///
pub(crate) const XCB_PROP_MODE_APPEND: u8 = 2;

/// X11 window attribute: background pixel.
pub(crate) const XCB_CW_BACK_PIXEL: u32 = 1 << 1;

/// X11 graphics context attribute: foreground pixel.
pub(crate) const XCB_GC_FOREGROUND: u32 = 4;

/// X11 window class: input/output window.
pub(crate) const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

/// XCB SHM segment separator (for shmseg 0).
pub(crate) const XCB_SHM_SEG_SEPARATOR: u32 = 0;
/// X11 image format: Z-pixmap.
pub(crate) const XCB_IMAGE_FORMAT_Z_PIXMAP: u8 = 2;
