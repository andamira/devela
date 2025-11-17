// devela::sys::display::x11::display
//
//! Defines [`XDisplay`].
//

#![allow(unused)]

use super::raw;
use crate::{Ptr, XError, c_int};
// use super::XkbState;

/// A connection to an X11 display server.
///
/// Wraps an `xcb_connection_t` and its associated screen information.
/// Represents the root environment required to create windows and interact with the X server.
#[derive(Debug)]
pub struct XDisplay {
    pub(super) conn: *mut raw::xcb_connection_t,
    pub(super) screen: *const raw::xcb_screen_t,
    pub(super) screen_num: c_int,
    pub(super) depth: u8,
    // pub(super) xkb: XkbState,
}

#[rustfmt::skip]
impl XDisplay {
    /// Opens the default X11 display and retrieves its first screen.
    pub fn open() -> Result<Self, XError> {
        // Open connection
        let mut screen_num = 0;
        let conn = unsafe { raw::xcb_connect(Ptr::null(), &mut screen_num as *mut c_int) };
        if conn.is_null() { return Err(XError::ConnectionFailed); }
        if unsafe { raw::xcb_connection_has_error(conn) } != 0 {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::ConnectionFailed);
        }
        // Get setup
        let setup = unsafe { raw::xcb_get_setup(conn) };
        if setup.is_null() {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::SetupFailed);
        }
        let iter = unsafe { raw::xcb_setup_roots_iterator(setup) };
        if iter.data.is_null() {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::NoScreensFound);
        }
        let screen: *const raw::xcb_screen_t = iter.data;
        let depth = unsafe { (*screen).root_depth };

        // let xkb = XkbState::new(conn)?;

        Ok(Self { conn, screen, screen_num, depth }) // xkb
    }
    /// Returns the root depth of the default screen.
    pub fn depth(&self) -> u8 { self.depth }

    /* internals */

    /// Returns the underlying XCB connection.
    pub(crate) fn conn(&self) -> *mut raw::xcb_connection_t { self.conn }

    /// Returns the default screen for this display.
    pub(crate) fn screen(&self) -> &raw::xcb_screen_t { unsafe { &*self.screen } }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe { raw::xcb_disconnect(self.conn) }
    }
}
