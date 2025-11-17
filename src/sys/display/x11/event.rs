// devela::sys::display::x11::event
//
//! Defines [`XEvent`].
//

use super::raw;
use crate::{Libc, c_void};

/// Wrapper for an XCB event.
#[derive(Debug)]
pub struct XEvent {
    pub(super) raw: *mut raw::xcb_generic_event_t,
}

#[rustfmt::skip]
impl XEvent {
    /// Returns the X11 response type (event code).
    #[inline(always)]
    pub fn response_type(&self) -> u8 { unsafe { (*self.raw).response_type & !0x80 } }

    ///
    pub fn keycode(&self) -> Option<u8> {
        if self.is_key_press() {
            let ev = self.raw as *const raw::xcb_key_press_event_t;
            Some(unsafe { (*ev).detail })
        } else {
            None
        }
    }

    ///
    pub fn modifiers(&self) -> u16 {
        unsafe { (*(self.raw as *const raw::xcb_key_press_event_t)).state }
    }

    /// Returns true if this is an expose (repaint) event.
    #[inline(always)]
    pub fn is_expose(&self) -> bool { self.response_type() == raw::XCB_EXPOSE }

    /// Returns true if this is a key-press event.
    #[inline(always)]
    pub fn is_key_press(&self) -> bool { self.response_type() == raw::XCB_KEY_PRESS }
    /// Returns true if this is a key-press event.
    #[inline(always)]
    pub fn is_key_release(&self) -> bool { self.response_type() == raw::XCB_KEY_RELEASE }

    /* internals */

    // pub(crate) fn raw(&self) -> *mut raw::xcb_generic_event_t { self.raw }
}

impl Drop for XEvent {
    fn drop(&mut self) {
        unsafe { Libc::free(self.raw as *mut c_void) };
    }
}
