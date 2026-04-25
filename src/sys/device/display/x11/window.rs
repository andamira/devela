// devela::sys::device::display::x11::window
//
//! Defines [`XWindow`].
//

use super::_raw;
use crate::{Extent, Position, XDisplay, XError, lets};

/// The inner state for [`XWindow`], stored in [`XDisplay`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct XWindowState {
    pub(crate) x: i16,
    pub(crate) y: i16,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) needs_redraw: bool,
}

#[doc = crate::_tags!(unix uid guard)]
/// X11 top-level drawable host and presentation target.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Represents a top-level window backed by an X11 drawable. Holds the XCB identifiers
/// required to manage the window, its graphics context, and its basic geometry.
#[derive(Debug)]
pub struct XWindow {
    pub(super) display: *mut _raw::xcb_connection_t,
    pub(super) win: u32,
    pub(super) gc: u32,
}

#[rustfmt::skip]
impl XWindow {
    /// Creates a new window on the given display.
    pub fn new(display: &mut XDisplay, x: i16, y: i16, width: u16, height: u16, border_width: u16)
        -> Result<Self, XError> {
        let conn = display.conn;
        let win: u32 = unsafe { _raw::xcb_generate_id(conn) }; // generate window ID
        let screen = unsafe { &*display.screen }; // extract screen

        // create window, supported events:
        let values: [u32; 2] = [screen.black_pixel,
            // key input
            _raw::XCB_EVENT_MASK_KEY_PRESS
            | _raw::XCB_EVENT_MASK_KEY_RELEASE
            // mouse button input
            | _raw::XCB_EVENT_MASK_BUTTON_PRESS
            | _raw::XCB_EVENT_MASK_BUTTON_RELEASE
            | _raw::XCB_EVENT_MASK_POINTER_MOTION
            | _raw::XCB_EVENT_MASK_BUTTON_MOTION
            // pointer enter/leave
            | _raw::XCB_EVENT_MASK_ENTER_WINDOW
            | _raw::XCB_EVENT_MASK_LEAVE_WINDOW
            // focus in/out
            | _raw::XCB_EVENT_MASK_FOCUS_CHANGE
            // expose
            | _raw::XCB_EVENT_MASK_EXPOSURE
            | _raw::XCB_EVENT_MASK_STRUCTURE_NOTIFY
            | _raw::XCB_EVENT_MASK_VISIBILITY_CHANGE
            | _raw::XCB_EVENT_MASK_PROPERTY_CHANGE
            // | _raw::XCB_EVENT_MASK_RESIZE_REDIRECT // only intended for window managers
            // | _raw::XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY
            // | _raw::XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT
        ];
        let mask: u32 = _raw::XCB_CW_BACK_PIXEL | _raw::XCB_CW_EVENT_MASK;
        unsafe {
            _raw::xcb_create_window(conn, screen.root_depth, win, screen.root, x, y, width, height,
                border_width, _raw::XCB_WINDOW_CLASS_INPUT_OUTPUT, screen.root_visual, mask,
                values.as_ptr());
        }

        // tell WM we are intentionally specifying a position
        let hints = _raw::XSizeHints::new().set_position(x, y);
        hints.set_on(conn, win, display.atoms.wm_normal_hints);

        // create graphic context
        let gc: u32 = unsafe { _raw::xcb_generate_id(conn) };
        let gc_values: [u32; 1] = [screen.black_pixel];
        unsafe { _raw::xcb_create_gc(conn, gc, win, _raw::XCB_GC_FOREGROUND, gc_values.as_ptr()); }
        unsafe { _raw::xcb_map_window(conn, win); } // show window

        // set window properties
        display.atoms.set_property_atom(conn, win,
            display.atoms.wm_protocols, display.atoms.wm_delete_window);

        let state = XWindowState { x, y, width, height, needs_redraw: false };
        display.window_register(win, state);

        // let window = Self { display: conn, win, gc, x, y, width, height, needs_redraw: false };
        let window = Self { display: conn, win, gc };
        display.flush();
        Ok(window)
    }

    /// Destroys this window and unregisters it from the display.
    ///
    /// Returns `true` if the window was still registered and a destroy request
    /// was sent to the X server.
    pub fn destroy(&self, display: &mut XDisplay) -> bool {
        display.window_destroy(self.id(), self.gc)
    }

    /* geometry queries */

    /// Returns the X11 window ID.
    pub const fn id(&self) -> u32 { self.win }

    /// Returns the dimensions of the window `(width, height)`.
    pub fn extent(&self, display: &XDisplay) -> Extent<u16, 2> {
        display.window_extent(self.id()).expect("current window ID is valid")
    }
    /// Returns the dimensions of the window.
    pub fn position(&self, display: &XDisplay) -> Position<i16, 2> {
        display.window_position(self.id()).expect("current window ID is valid")
    }
    /// Returns whether this window is currently marked for redraw.
    #[must_use]
    pub fn needs_redraw(&self, display: &XDisplay) -> bool {
        display.window_needs_redraw(self.id())
    }
    /// Clears the redraw flag.
    pub fn clear_redraw(&self, display: &mut XDisplay) {
        display.window_clear_redraw(self.id());
    }

    /* */

    /// Writes image bytes into the window using XCB `PutImage`.
    pub fn put_image_bytes(&self, width: u16, height: u16, depth: u8, data: &[u8]) {
        lets![dst_x=0, dst_y=0, left_pad=0];
        unsafe {
            _raw::xcb_put_image(self.display, _raw::XCB_IMAGE_FORMAT_Z_PIXMAP, self.win, self.gc,
                width, height, dst_x, dst_y, left_pad, depth, data.len() as u32, data.as_ptr());
        }
    }

    /// Writes a tightly packed 32-bit pixel buffer into the window using XCB.
    pub fn put_image_u32(&self, width: u16, height: u16, depth: u8, data: &[u32]) {
        let len = data.len() * size_of::<u32>();
        let bytes = unsafe { core::slice::from_raw_parts(data.as_ptr().cast::<u8>(), len) };
        self.put_image_bytes(width, height, depth, bytes);
    }

    /// Fills a rectangle in the window
    /// using the foreground pixel defined in the window's graphics context.
    ///
    /// Coordinates are relative to the window's origin in the X11 pixel grid.
    pub fn fill_rect(&self, pos: Position<i16, 2>, ext: Extent<u16, 2>) {
        let rect = _raw::xcb_rectangle_t::new(pos, ext);
        unsafe {
            _raw::xcb_poly_fill_rectangle(self.display, self.win, self.gc, 1, &rect as *const _);
        }
    }
}
