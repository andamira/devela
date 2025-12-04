// devela::sys::display::x11::display
//
//! Defines [`XDisplay`].
//

#![allow(unused)]

use super::{KeyRepeatFilter, XkbState, raw};
use crate::{
    Event, EventButton, EventButtonState, EventKind, EventMouse, EventWindow, Ptr, XError, XEvent,
    c_int, is,
};

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

    /// Polls the next event without blocking.
    ///
    /// If a raw event is available, it is processed through `handle_raw_event`
    /// and returned as a high-level [`Event`].
    /// If no event is pending, returns [`Event::None`].
    ///
    /// This is suitable for rendering loops or cooperative schedulers where
    /// the program must remain responsive without waiting.
    #[inline(always)]
    pub fn poll_event(&mut self) -> Event {
        is![let Some(raw) = self.poll_raw_event(); self.handle_raw_event(raw); Event::None]
    }

    /// Waits for the next event, blocking until one is available.
    ///
    /// The raw event is passed to `handle_raw_event` and returned as a high-level [`Event`].
    /// If the X connection enters an error state, returns [`Event::None`].
    ///
    /// This is the blocking counterpart of [`poll_event`][Self::poll_event], used when the
    /// application should sleep until user input or a window notification arrives.
    #[inline(always)]
    pub fn wait_event(&mut self) -> Event {
        is![let Some(raw) = self.wait_raw_event(); self.handle_raw_event(raw); Event::None]
    }

    /* internals */

    /// Returns the underlying XCB connection.
    pub(crate) fn conn(&self) -> *mut raw::xcb_connection_t { self.conn }

    /// Returns the default screen for this display.
    pub(crate) fn screen(&self) -> &raw::xcb_screen_t { unsafe { &*self.screen } }

    /// Interprets a raw X11 event and converts it into a high-level [`Event`].
    ///
    /// This handles:
    /// - key translation (press/release, repeat filtering, semantic mapping),
    /// - basic pointer and window-related notifications,
    /// - special cases such as synthetic key releases.
    ///
    /// It never performs I/O. The caller supplies the raw event pointer; the method returns
    /// the corresponding high-level `Event` or `Event::None` if the event is ignored.
    fn handle_raw_event(&mut self, xev: XEvent) -> Event {
        if let Some(ev) = xev.as_raw_key() {
            let keycode = unsafe { ev.detail };
            let time_ms = unsafe { ev.time };
            if xev.is_key_release() {
                let real = self.classify_release(keycode, time_ms);
                if !real { return Event::default(); } // skip fake release
            }
            if let Some(key) = xev.to_event_key(&self.xkb, &mut self.repeat_filter) {
                return Event::new(EventKind::Key(key), xev.timestamp());
            }

        } else if let Some(ev) = xev.as_raw_button() {
            let buttons = XEvent::map_button_mask(ev.state);
            let button = XEvent::map_button(ev.detail);
            let state  = xev.map_button_state();
            return Event::new(
                EventKind::Mouse(EventMouse {
                    x: ev.event_x.into(),
                    y: ev.event_y.into(),
                    button: Some(button),
                    state,
                    buttons,
                    timestamp: xev.timestamp(),
                }),
                xev.timestamp(),
            );

        } else if let Some(ev) = xev.as_raw_motion() {
            let buttons = XEvent::map_button_mask(ev.state);
            let button = EventButton::primary_from_mask(buttons);
            return Event::new(
                EventKind::Mouse(EventMouse {
                    x: ev.event_x.into(),
                    y: ev.event_y.into(),
                    button,
                    state: EventButtonState::Moved,
                    buttons,
                    timestamp: xev.timestamp(),
                }),
                xev.timestamp(),
            );
        } else if xev.is_client_message() {
            // TODO:
            // eprintln!("CLIENT MSG: {}", xev.response_type());
        } else if xev.is_enter() {
            // return Event::new(EventKind::Window(EventWindow::Enter), xev.timestamp());
        } else if xev.is_leave() {
            // return Event::new(EventKind::Window(EventWindow::Leave), xev.timestamp());
        } else if xev.is_expose() {
            return Event::new(EventKind::Window(EventWindow::RedrawRequested), xev.timestamp());
        } else {
            // TODO
            // eprintln!("(OTHER): {} {:?}", xev.response_type(), xev.timestamp());
        }
        Event::None
    }

    /// Returns the next raw X11 event pointer, if available.
    ///
    /// Consumes a previously buffered event if one exists; otherwise polls
    /// the connection with `xcb_poll_for_event`. Returns `None` if no event
    /// is available at this time.
    fn next_raw_event(&mut self) -> Option<*mut raw::xcb_generic_event_t> {
        if let Some(ev) = self.pending.take() { return Some(ev); }
        let ev = unsafe { raw::xcb_poll_for_event(self.conn) };
        if ev.is_null() { None } else { Some(ev) }
    }

    /// Peeks at the next raw X11 event without consuming it.
    ///
    /// If no event is currently buffered, attempts a non-blocking poll and,
    /// if successful, stores the event in the internal `pending` slot.
    /// Returns the buffered pointer, or `None` if no event is available.
    fn peek_raw_event(&mut self) -> Option<*mut raw::xcb_generic_event_t> {
        if self.pending.is_none() {
            let ev = unsafe { raw::xcb_poll_for_event(self.conn) };
            if !ev.is_null() { self.pending = Some(ev); }
        }
        self.pending
    }

    /// Retrieves the next X11 event, if any.
    ///
    /// Returns an `XEvent` wrapper around a raw XCB event pointer.
    /// Uses any internally buffered event before polling the X server.
    fn poll_raw_event(&mut self) -> Option<XEvent> {
        let ev = self.next_raw_event()?;
        Some(XEvent { raw: ev })
    }

    /// Blocks until the next raw X11 event is available.
    ///
    /// Returns a pointer to the raw `xcb_generic_event_t`.
    /// If a previously buffered event exists, it is returned first.
    /// Otherwise the call blocks until the X server sends a new event.
    ///
    /// Only returns `None` if the connection is in an error state.
    fn wait_raw_event(&mut self) -> Option<XEvent> {
        if let Some(ev) = self.pending.take() { return Some(XEvent { raw: ev }); }
        let ev = unsafe { raw::xcb_wait_for_event(self.conn) };
        if ev.is_null() { None } else { Some(XEvent { raw: ev }) }
    }

    /// Determines whether a key release is genuine or part of an auto-repeat sequence.
    ///
    /// X11 synthesizes a "fake" release immediately followed by a press when a key auto-repeats.
    /// This method looks ahead at the next pending event and classifies the release as:
    /// - `false`: synthetic release (auto-repeat).
    /// - `true`:  real physical release.
    ///
    /// The classification is based on matching keycode and timestamp.
    fn classify_release(&mut self, keycode: u8, timestamp: u32) -> bool {
        if let Some(next) = self.peek_raw_event() {
            unsafe {
                let ty = (*next).response_type & 0x7F;
                if ty == raw::xcb_event_code::XCB_KEY_PRESS as u8 {
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
