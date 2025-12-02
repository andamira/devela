// devela::sys::display::x11::event
//
//! Defines [`XEvent`].
//

use super::{KeyRepeatFilter, XkbState, raw};
use crate::{EventKey, EventTimestamp, KeyState, Libc, c_void};

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

    /// Returns the event timestamp in backend-specific milliseconds, if present.
    pub fn timestamp(&self) -> Option<EventTimestamp> {
        if self.is_key() {
            let ev = self.raw as *const raw::xcb_key_press_event_t;
            Some(EventTimestamp::from_millis_u32((unsafe { *ev }).time))
        } else {
            None
        }
    }

    /// Returns true if this is an expose (repaint) event.
    #[inline(always)]
    pub fn is_expose(&self) -> bool { self.response_type() == raw::XCB_EXPOSE }

    /// Returns true if this is a key event.
    #[inline(always)]
    pub fn is_key(&self) -> bool { self.is_key_press() || self.is_key_release() }
    /// Returns true if this is a key-press event.
    #[inline(always)]
    pub fn is_key_press(&self) -> bool { self.response_type() == raw::XCB_KEY_PRESS }
    /// Returns true if this is a key-press event.
    #[inline(always)]
    pub fn is_key_release(&self) -> bool { self.response_type() == raw::XCB_KEY_RELEASE }

    /* internals */

    /// Converts this X11 key event into a generic `EventKey` using XKB.
    pub(crate) fn to_event_key(&self, xkb: &XkbState, repeat: &mut KeyRepeatFilter)
        -> Option<EventKey> {

        let ev = self.raw as *const raw::xcb_key_press_event_t;
        let (keycode, state_raw, time_ms) = unsafe { ((*ev).detail, (*ev).state, (*ev).time) };
        let is_press = self.is_key_press();

        // filter before touching XKB
        let state = repeat.filter(keycode, is_press);

        // update XKB only for real events (Press/Repeat/Release)
        let dir = match state {
            KeyState::Press | KeyState::Repeat => raw::XKB_KEY_DOWN,
            KeyState::Release => raw::XKB_KEY_UP,
        };
        xkb.update(keycode, dir);

        // translate key after both filter + update
        let info = xkb.translate_key(keycode, state_raw);

        Some(EventKey {
            semantic: info.semantic,
            physical: info.physical,
            state,
            mods: info.mods,
            timestamp: Some(EventTimestamp::from_millis_u32(time_ms)),
        })
    }
}

impl Drop for XEvent {
    fn drop(&mut self) {
        unsafe { Libc::free(self.raw as *mut c_void) };
    }
}
