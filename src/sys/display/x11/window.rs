// devela::sys::display::x11::window
//
//! Defines [`XWindow`].
//

#![allow(unused)]

use super::raw;
use crate::{Extent, Libc, Position, XDisplay, XError, XEvent, lets, x11_intern_atom};

/// X11 window created through XCB.
///
/// Represents a top-level window backed by an X11 drawable. Holds the XCB identifiers
/// required to manage the window, its graphics context, and its basic geometry.
#[derive(Debug)]
pub struct XWindow {
    pub(super) display: *mut raw::xcb_connection_t,
    pub(super) win: u32,
    pub(super) gc: u32,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) depth: u8,
}

#[rustfmt::skip]
impl XWindow {
    /// Creates a new window on the given display.
    pub fn new(display: &XDisplay, x: i16, y: i16, width: u16, height: u16, border_width: u16)
        -> Result<Self, XError> {
        let conn = display.conn;
        let win: u32 = unsafe { raw::xcb_generate_id(conn) }; // generate window ID
        let screen = unsafe { &*display.screen }; // extract screen

        // create window, supported events:
        let values: [u32; 2] = [screen.black_pixel,
            // key input
            raw::XCB_EVENT_MASK_KEY_PRESS
            | raw::XCB_EVENT_MASK_KEY_RELEASE
            // mouse button input
            | raw::XCB_EVENT_MASK_BUTTON_PRESS
            | raw::XCB_EVENT_MASK_BUTTON_RELEASE
            | raw::XCB_EVENT_MASK_POINTER_MOTION
            | raw::XCB_EVENT_MASK_BUTTON_1_MOTION
            | raw::XCB_EVENT_MASK_BUTTON_2_MOTION
            | raw::XCB_EVENT_MASK_BUTTON_3_MOTION
            | raw::XCB_EVENT_MASK_BUTTON_4_MOTION
            | raw::XCB_EVENT_MASK_BUTTON_5_MOTION
            // | raw::XCB_EVENT_MASK_BUTTON_MOTION
            // pointer enter/leave
            | raw::XCB_EVENT_MASK_ENTER_WINDOW
            | raw::XCB_EVENT_MASK_LEAVE_WINDOW
            // focus in/out
            | raw::XCB_EVENT_MASK_FOCUS_CHANGE
            // expose
            | raw::XCB_EVENT_MASK_EXPOSURE
            // | raw::XCB_EVENT_MASK_VISIBILITY_CHANGE
            | raw::XCB_EVENT_MASK_STRUCTURE_NOTIFY
            // resize
            // | raw::XCB_EVENT_MASK_RESIZE_REDIRECT
            // | raw::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY
            // | raw::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT
            // | raw::XCB_EVENT_MASK_PROPERTY_CHANGE
        ];
        let mask: u32 = raw::XCB_CW_BACK_PIXEL | raw::XCB_CW_EVENT_MASK;
        unsafe {
            raw::xcb_create_window(conn, screen.root_depth, win, screen.root, x, y, width, height,
                border_width, raw::XCB_WINDOW_CLASS_INPUT_OUTPUT, screen.root_visual, mask,
                values.as_ptr());
        }
        // create graphic context
        let gc: u32 = unsafe { raw::xcb_generate_id(conn) };
        let gc_values: [u32; 1] = [screen.black_pixel];
        unsafe { raw::xcb_create_gc(conn, gc, win, raw::XCB_GC_FOREGROUND, gc_values.as_ptr()); }
        unsafe { raw::xcb_map_window(conn, win); } // show window

        // enable WM_DELETE_WINDOW so the window manager can request closure
        let wm_protocols = x11_intern_atom(conn, b"WM_PROTOCOLS");
        let wm_delete_window = x11_intern_atom(conn, b"WM_DELETE_WINDOW");
        let data = wm_delete_window.to_ne_bytes();
        unsafe { raw::xcb_change_property(conn, raw::xcb_prop_mode::XCB_PROP_MODE_REPLACE as u8,
            win, wm_protocols, raw::xcb_atom_enum_t::XCB_ATOM_ATOM as u32, 32, 1, data.as_ptr()); }

        let window = Self { display: conn, win, gc, width, height, depth: display.depth };
        window.flush();
        Ok(window)
    }

    /// Returns the X11 window ID.
    pub fn id(&self) -> u32 { self.win }

    /// Returns the dimensions of the window `(width, height)`.
    pub fn size(&self) -> (u16, u16) { (self.width, self.height) }

    /// Flushes pending XCB commands.
    #[inline(always)]
    pub fn flush(&self) { unsafe { raw::xcb_flush(self.display); } }

    /// Writes an rgba image from a byte buffer, into the window using XCB.
    pub fn put_image_bytes(&self, width: u16, height: u16, depth: u8, data: &[u8]) {
        lets![dst_x=0, dst_y=0, left_pad=0];
        unsafe { raw::xcb_put_image(self.display, raw::XCB_IMAGE_FORMAT_Z_PIXMAP, self.win, self.gc,
            width, height, dst_x, dst_y, left_pad, depth, data.len() as u32, data.as_ptr()); }
    }

    /// Writes an rgba image from a u32 buffer into the window using XCB.
    pub fn put_image_u32(&self, width: u16, height: u16, depth: u8, data: &[u32]) {
        lets![dst_x=0, dst_y=0, left_pad=0];
        unsafe {
            let bytes = core::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4);
            raw::xcb_put_image(self.display, raw::XCB_IMAGE_FORMAT_Z_PIXMAP, self.win, self.gc,
            width, height, dst_x, dst_y, left_pad, depth, data.len() as u32 * 4, bytes.as_ptr());
        }
    }

    /// Fills a rectangle in the window
    /// using the foreground pixel defined in the window’s graphics context.
    ///
    /// Coordinates are relative to the window’s origin in the X11 pixel grid.
    pub fn fill_rect(&self, pos: Position<i16, 2>, ext: Extent<u16, 2>) {
        let rect = raw::xcb_rectangle_t::new(pos, ext);
        unsafe { raw::xcb_poly_fill_rectangle(self.display, self.win, self.gc, 1, &rect as *const _); }
    }
}
