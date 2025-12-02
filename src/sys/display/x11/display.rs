// devela::sys::display::x11::display
//
//! Defines [`XDisplay`].
//

#![allow(unused)]

use super::{KeyRepeatFilter, XkbState, raw};
use crate::{Event, EventKind, EventWindow, Ptr, XError, XEvent, c_int};

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
    pub(super) xkb: XkbState,
    /* repeat detection */
    pub(super) pending: Option<*mut raw::xcb_generic_event_t>,
    pub(super) repeat_filter: KeyRepeatFilter,
}

#[rustfmt::skip]
impl XDisplay {
    /// Opens the default X11 display and retrieves its first screen.
    pub fn open() -> Result<Self, XError> {
        // open connection
        let mut screen_num = 0;
        let conn = unsafe { raw::xcb_connect(Ptr::null(), &mut screen_num as *mut c_int) };
        if conn.is_null() { return Err(XError::ConnectionFailed); }
        if unsafe { raw::xcb_connection_has_error(conn) } != 0 {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::ConnectionFailed);
        }
        // get setup
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

        // extension setup hand-shake
        let mut major = 0u16; let mut minor = 0u16; let mut ev = 0u8; let mut err = 0u8;
        let ok = unsafe { raw::xkb_x11_setup_xkb_extension(conn, 1, 0, 0,
            &mut major, &mut minor, &mut ev, &mut err) };
        if ok <= 0 {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::ExtensionUnavailable("xkb-setup"));
        }
        let xkb = XkbState::new(conn)?;

        let pending = None;
        let repeat_filter = KeyRepeatFilter::new();

        Ok(Self { conn, screen, screen_num, depth, xkb, pending, repeat_filter })
    }
    /// Returns the root depth of the default screen.
    pub fn depth(&self) -> u8 { self.depth }

    /// Polls the next generic event from the X server.
    pub fn poll_event(&mut self) -> Event {
        if let Some(xev) = self.poll_raw_event() {
            if xev.is_key() {
                let ev = xev.raw as *const raw::xcb_key_press_event_t;
                let keycode = unsafe { (*ev).detail };
                let time_ms = unsafe { (*ev).time };
                if xev.is_key_release() {
                    let real = self.classify_release(keycode, time_ms);
                    if !real { return Event::default(); } // skip fake release
                }
                if let Some(key) = xev.to_event_key(&self.xkb, &mut self.repeat_filter) {
                    return Event::new(EventKind::Key(key), xev.timestamp());
                }
            } else if xev.is_expose() {
                return Event::new(EventKind::Window(EventWindow::RedrawRequested),xev.timestamp());
            }
        }
        Event::default()
    }

    /* internals */

    /// Returns the underlying XCB connection.
    pub(crate) fn conn(&self) -> *mut raw::xcb_connection_t { self.conn }

    /// Returns the default screen for this display.
    pub(crate) fn screen(&self) -> &raw::xcb_screen_t { unsafe { &*self.screen } }

    /// Polls the next raw X11 event.
    fn poll_raw_event(&mut self) -> Option<XEvent> {
        let ev = self.next_raw_event()?;
        Some(XEvent { raw: ev })
    }

    fn next_raw_event(&mut self) -> Option<*mut raw::xcb_generic_event_t> {
        if let Some(ev) = self.pending.take() { return Some(ev); }
        let ev = unsafe { raw::xcb_poll_for_event(self.conn) };
        if ev.is_null() { None } else { Some(ev) }
    }
    fn peek_raw_event(&mut self) -> Option<*mut raw::xcb_generic_event_t> {
        if self.pending.is_none() {
            let ev = unsafe { raw::xcb_poll_for_event(self.conn) };
            if !ev.is_null() { self.pending = Some(ev); }
        }
        self.pending
    }
    fn classify_release(&mut self, keycode: u8, timestamp: u32) -> bool {
        if let Some(next) = self.peek_raw_event() {
            unsafe {
                let ty = (*next).response_type & 0x7F;
                if ty == raw::XCB_KEY_PRESS {
                    let kpev = next as *const raw::xcb_key_press_event_t;
                    if (*kpev).detail == keycode && (*kpev).time == timestamp { return false; }
                }
            }
        }
        true
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe { raw::xcb_disconnect(self.conn) }
    }
}
