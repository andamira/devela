// devela::sys::display::x11::event
//
//! Defines [`XEvent`].
//

use super::{KeyRepeatFilter, XkbState, raw, xcb_event_code};
use crate::{
    EventButton, EventButtonState, EventKey, EventTimestamp, KeyState, Libc, c_void, is, unwrap,
};
// use crate::{EventKind, EventWindow};

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
        type XcbAnyInputEvent = raw::xcb_key_press_event_t;
        if self.is_key() | self.is_button() | self.is_motion() | self.is_enter() | self.is_leave() {
            let ev = self.raw as *const XcbAnyInputEvent;
            Some(EventTimestamp::from_millis_u32((unsafe { *ev }).time))
        } else {
            None
        }
    }

    // TODO
    // pub fn event_kind(&self) -> EventKind {
    //     match self.response_type() {
    //         raw::XCB_EXPOSE => EventKind::Window(EventWindow::RedrawRequested),
    //         _ => EventKind::None,
    //     }
    // }

    /// Whether this is a key event.
    pub fn is_key(&self) -> bool { self.is_key_press() || self.is_key_release() }

    /// Whether this is a key-press event.
    pub fn is_key_press(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_KEY_PRESS as u8
    }
    /// Whether this is a key-release event.
    pub fn is_key_release(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_KEY_RELEASE as u8
    }

    /// Whether this is a button event.
    pub fn is_button(&self) -> bool { self.is_button_press() || self.is_button_release() }

    /// Whether this is a button-press event.
    pub fn is_button_press(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_BUTTON_PRESS as u8
    }
    /// Whether this is a button-release event.
    pub fn is_button_release(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_BUTTON_RELEASE as u8
    }

    /// Whether this is a motion event.
    pub fn is_motion(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_MOTION_NOTIFY as u8
    }

    /// Whether this is an enter event.
    pub fn is_enter(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_ENTER_NOTIFY as u8
    }
    /// Whether this is a leave event.
    pub fn is_leave(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_LEAVE_NOTIFY as u8
    }
    /// Whether this is an expose (repaint) event.
    pub fn is_expose(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_EXPOSE as u8
    }
    /// Whether this is a client message.
    pub fn is_client_message(&self) -> bool {
        self.response_type() == xcb_event_code::XCB_CLIENT_MESSAGE as u8
    }

    /* internals */

    /// Return some key event, if that's the kind.
    pub(crate) fn as_raw_key(&self) -> Option<&raw::xcb_key_press_event_t> {
        if self.is_key() {
            Some(unsafe { &*(self.raw as *const raw::xcb_key_press_event_t) })
        } else { None }
    }
    /// Return some key event, if that's the kind.
    pub(crate) fn as_raw_button(&self) -> Option<&raw::xcb_button_press_event_t> {
        if self.is_button() {
            Some(unsafe { &*(self.raw as *const raw::xcb_button_press_event_t) })
        } else { None }
    }
    /// Return some key event, if that's the kind.
    pub(crate) fn as_raw_motion(&self) -> Option<&raw::xcb_motion_notify_event_t> {
        if self.is_motion() {
            Some(unsafe { &*(self.raw as *const raw::xcb_motion_notify_event_t) })
        } else { None }
    }

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
            KeyState::Press | KeyState::Repeat => raw::xkb_key_direction::XKB_KEY_DOWN,
            KeyState::Release => raw::xkb_key_direction::XKB_KEY_UP,
        };
        xkb.update(keycode, dir);

        // translate key after both filter + update
        let info = xkb.translate_key(keycode, state_raw);
        // WAIT: until libxkbcommon ≥ 1.12 becomes widely deployed.
        // let info = xkb.translate_key(keycode);

        Some(EventKey {
            semantic: info.semantic,
            physical: info.physical,
            state,
            mods: info.mods,
            timestamp: Some(EventTimestamp::from_millis_u32(time_ms)),
        })
    }

    /// Converts this X11 button event into an `EventButton`.
    pub(crate) const fn map_button(detail: u8) -> EventButton {
        unwrap![some EventButton::new(detail)]
    }
    /// Converts this X11 button state into an `EventButton.buttons` bit-mask field.
    #[inline(always)]
    pub(crate) fn map_button_mask(state: u16) -> u8 {
        let mut mask = 0u8;
        is![state & (raw::XCB_KEY_BUT_MASK_BUTTON_1) != 0; mask |= 1]; // left
        is![state & (raw::XCB_KEY_BUT_MASK_BUTTON_3) != 0; mask |= 2]; // right
        is![state & (raw::XCB_KEY_BUT_MASK_BUTTON_2) != 0; mask |= 4]; // middle
        mask
    }

    /// Converts this X11 button state into an `EventButtonState`.
    pub(crate) fn map_button_state(&self) -> EventButtonState {
        if self.is_button_press() { EventButtonState::Pressed }
        else if self.is_button_release() { EventButtonState::Released }
        else { EventButtonState::Moved }
    }
}

impl Drop for XEvent {
    fn drop(&mut self) {
        unsafe { Libc::free(self.raw as *mut c_void) };
    }
}
