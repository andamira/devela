// devela::sys::display::x11::raw::consts
//
//! Constants from `xproto.h`
//

#![allow(dead_code)]

/// X11 event mask: exposure events.
pub(crate) const XCB_EVENT_MASK_EXPOSURE: u32 = 1 << 15;
/// X11 event mask: key press events.
pub(crate) const XCB_EVENT_MASK_KEY_PRESS: u32 = 1;

/// X11 window attribute: background pixel.
pub(crate) const XCB_CW_BACK_PIXEL: u32 = 1 << 1;
/// X11 window attribute: event mask.
pub(crate) const XCB_CW_EVENT_MASK: u32 = 1 << 11;

/// X11 graphics context attribute: foreground pixel.
pub(crate) const XCB_GC_FOREGROUND: u32 = 4;

/// X11 window class: input/output window.
pub(crate) const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

/// X11 event code: exposure event.
pub(crate) const XCB_EXPOSE: u8 = 12;
/// X11 event code: key press event.
pub(crate) const XCB_KEY_PRESS: u8 = 2;
/// X11 event code: key press event.
pub(crate) const XCB_KEY_RELEASE: u8 = 3;

/// XCB SHM segment separator (for shmseg 0).
pub(crate) const XCB_SHM_SEG_SEPARATOR: u32 = 0;
/// X11 image format: Z-pixmap.
pub(crate) const XCB_IMAGE_FORMAT_Z_PIXMAP: u8 = 2;
