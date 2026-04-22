// devela::sys::device::display::x11::display
//
//! Defines [`XDisplay`].
//

#![allow(unused)]

use super::{KeyRepeatFilter, XAtoms, XError, XEvent, XWindowState, XkbState, raw};
use crate::{ConstInit, Extent, Position, Ptr, Vec, c_int, is, lets, sf, vec_ as vec};
use crate::{
    Event, EventButton, EventButtonState, EventButtons, EventKind, EventMouse, EventQueue,
    EventWheel, EventWheelUnit, EventWindow,
};

/// Describes which parts of a window configuration changed.
///
/// Produced when a `ConfigureNotify` event is applied to the cached state of a registered window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct XWindowConfigureDelta {
    /// Whether the window position changed.
    pub(crate) moved: bool,
    /// Whether the window size changed.
    pub(crate) resized: bool,
}

#[doc = crate::_tags!(unix runtime guard)]
/// A connection to an X11 display server.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Wraps an `xcb_connection_t` and its associated screen information.
/// Represents the root environment required to create windows and interact with the X server.
#[derive(Debug)]
pub struct XDisplay {
    pub(super) conn: *mut raw::xcb_connection_t,
    pub(super) screen: *const raw::xcb_screen_t,
    screen_num: c_int,
    pub(super) depth: u8,
    xkb: XkbState,
    pub(super) atoms: XAtoms,
    /* repeat detection */
    pub(super) pending: Option<*mut raw::xcb_generic_event_t>,
    pub(super) repeat_filter: KeyRepeatFilter,
    pub(super) windows: Vec<(u32, XWindowState)>,
    // custom synthetic event queue
    queue: EventQueue<2>,
}

impl XDisplay {
    /// Opens the default X11 display and retrieves its first screen.
    pub fn open() -> Result<Self, XError> {
        // open connection
        let mut screen_num = 0;
        let conn = unsafe { raw::xcb_connect(Ptr::null(), &mut screen_num as *mut c_int) };
        is! { conn.is_null(), return Err(XError::ConnectionFailed) }
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
        lets![mut major = 0, mut minor = 0, mut ev = 0, mut err = 0];
        let ok = unsafe {
            raw::xkb_x11_setup_xkb_extension(
                conn, 1, 0, 0, &mut major, &mut minor, &mut ev, &mut err,
            )
        };
        if ok <= 0 {
            unsafe { raw::xcb_disconnect(conn) };
            return Err(XError::ExtensionUnavailable("xkb-setup"));
        }
        let xkb = XkbState::new(conn)?;
        let atoms = XAtoms::new(conn);

        // repeat
        let pending = None;
        let repeat_filter = KeyRepeatFilter::new();

        let windows = vec![];
        let queue = EventQueue::new();

        sf! { Ok(Self {
            conn, screen, screen_num, depth, xkb, pending, repeat_filter, atoms, windows, queue
        }) }
    }

    /// Returns the root depth of the default screen.
    pub fn depth(&self) -> u8 {
        self.depth
    }

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
        is![let Some(ev) = self.queue.pop(), return ev]; // return pending events first
        is![let Some(raw) = self.poll_raw_event(), return self.handle_raw_event(raw)];
        Event::None
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
        is![let Some(ev) = self.queue.pop(), return ev]; // return pending events first
        is![let Some(raw) = self.wait_raw_event(), self.handle_raw_event(raw), Event::None]
    }

    /// Flushes pending XCB commands.
    #[inline(always)]
    pub fn flush(&self) {
        unsafe {
            raw::xcb_flush(self.conn);
        }
    }
}

// Internal methods
impl XDisplay {
    /// Returns the underlying XCB connection.
    pub(crate) fn conn(&self) -> *mut raw::xcb_connection_t {
        self.conn
    }

    /// Returns the default screen for this display.
    pub(crate) fn screen(&self) -> &raw::xcb_screen_t {
        unsafe { &*self.screen }
    }

    /* windows */

    /// Returns `true` if the requested window is registered.
    fn has_window(&self, window_id: u32) -> bool {
        self.windows.iter().any(|&(id, _)| id == window_id)
    }
    // fn window_ids(&self) -> impl Iterator<Item = u32> { todo![] }

    /// Returns a shared reference to the requested inner window state.
    fn window_state(&self, window_id: u32) -> Option<&XWindowState> {
        self.windows.iter().find(|&&(id, _)| id == window_id).map(|(_, state)| state)
    }
    /// Returns an exclusive reference to the requested inner window state.
    fn window_state_mut(&mut self, window_id: u32) -> Option<&mut XWindowState> {
        self.windows.iter_mut().find(|(id, _)| *id == window_id).map(|(_, state)| state)
    }

    /// Registers a window.
    pub(crate) fn window_register(&mut self, window_id: u32, state: XWindowState) {
        debug_assert!(!self.has_window(window_id), "window already registered: {window_id}");
        self.windows.push((window_id, state));
    }
    /// Unregisters a window.
    fn window_unregister(&mut self, window_id: u32) -> Option<XWindowState> {
        let index = self.windows.iter().position(|&(id, _)| id == window_id)?;
        Some(self.windows.swap_remove(index).1)
    }
    /// Destroys a registered window and removes its cached state.
    ///
    /// This sends a destroy request to the X server, frees the associated
    /// graphics context, unregisters the window from this display, and flushes
    /// the connection.
    ///
    /// Returns `true` if the window was registered and a destroy request was sent.
    pub(crate) fn window_destroy(&mut self, window_id: u32, gc: u32) -> bool {
        let Some(_) = self.window_unregister(window_id) else {
            return false;
        };
        unsafe {
            raw::xcb_free_gc(self.conn, gc);
            raw::xcb_destroy_window(self.conn, window_id);
        }
        self.flush();
        true
    }

    /// Returns the dimensions of the window `(width, height)`.
    pub(crate) fn window_extent(&self, window_id: u32) -> Option<Extent<u16, 2>> {
        let state = self.window_state(window_id)?;
        Some(Extent::new([state.width, state.height]))
    }
    /// Returns the dimensions of the window.
    pub(crate) fn window_position(&self, window_id: u32) -> Option<Position<i16, 2>> {
        let state = self.window_state(window_id)?;
        Some(Position::new([state.x, state.y]))
    }
    /// Returns whether this window is currently marked for redraw.
    ///
    /// The flag is stored in [`XDisplay`] as part of the cached runtime state for this window.
    pub(crate) fn window_needs_redraw(&self, window_id: u32) -> bool {
        if let Some(state) = self.window_state(window_id) {
            state.needs_redraw
        } else {
            false
        }
    }
    /// Marks the requested window as needing redraw.
    ///
    /// Returns `true` if the window was found and updated.
    fn window_mark_redraw(&mut self, window_id: u32) -> bool {
        if let Some(state) = self.window_state_mut(window_id) {
            state.needs_redraw = true;
            true
        } else {
            false
        }
    }
    /// Clears the redraw flag of the requested window.
    ///
    /// Returns `true` if the window was found and updated.
    pub(crate) fn window_clear_redraw(&mut self, window_id: u32) -> bool {
        if let Some(state) = self.window_state_mut(window_id) {
            state.needs_redraw = false;
            true
        } else {
            false
        }
    }

    /// Updates the requested window size and/or position and returns its delta.
    fn window_update_configure(
        &mut self,
        window_id: u32,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Option<XWindowConfigureDelta> {
        let state = self.window_state_mut(window_id)?;
        let moved = state.x != x || state.y != y;
        let resized = state.width != width || state.height != height;
        state.x = x;
        state.y = y;
        state.width = width;
        state.height = height;
        is! { resized, state.needs_redraw = true }
        Some(XWindowConfigureDelta { moved, resized })
    }

    /* events */

    /// Interprets a raw X11 event and converts it into a high-level [`Event`].
    ///
    /// This handles:
    /// - key translation (press/release, repeat filtering, semantic mapping),
    /// - basic pointer and window-related notifications,
    /// - special cases such as synthetic key releases.
    ///
    /// It never performs I/O. The caller supplies the raw event pointer; the method returns
    /// the corresponding high-level `Event` or `Event::None` if the event is ignored.
    #[rustfmt::skip]
    fn handle_raw_event(&mut self, xev: XEvent) -> Event {
        use {EventKind as Kind, EventWindow as Win};
        if let Some(ev) = xev.as_raw_key() {
            let keycode = unsafe { ev.detail };
            let time_ms = unsafe { ev.time };
            if xev.is_key_release() {
                let real = self.classify_release(keycode, time_ms);
                is! { !real, return Event::default() } // skip fake release
            }
            let key = xev.to_event_key(&self.xkb, &mut self.repeat_filter);
            return Event::from_window(ev.event, Kind::Key(key), xev.timestamp());
        } else if let Some(ev) = xev.as_raw_button() {
            let x = ev.event_x.into();
            let y = ev.event_y.into();
            let timestamp = xev.timestamp();
            let buttons = XEvent::map_button_mask(ev.state);
            let unit = EventWheelUnit::Step;
            // X11 raw button 4..7 become EventWheel { unit: Step, ... }
            match ev.detail {
                4 => { return Event::from_window(ev.event, Kind::Wheel(
                        EventWheel::new(0, -1, unit, x, y, buttons)), timestamp); }
                5 => { return Event::from_window(ev.event, Kind::Wheel(
                        EventWheel::new(0, 1, unit, x, y, buttons)), timestamp); }
                6 => { return Event::from_window(ev.event, Kind::Wheel(
                        EventWheel::new(-1, 0, unit, x, y, buttons)), timestamp); }
                7 => { return Event::from_window(ev.event, Kind::Wheel(
                        EventWheel::new(1, 0, unit, x, y, buttons)), timestamp); }
                _ => {
                    let button = XEvent::map_button(ev.detail);
                    let state = xev.map_button_state();
                    return Event::from_window(ev.event, Kind::Mouse(
                        EventMouse { x, y, button: Some(button), state, buttons }),
                        timestamp,
                    );
                }
            }
        } else if let Some(ev) = xev.as_raw_motion() {
            let buttons = XEvent::map_button_mask(ev.state);
            let button = EventButton::primary_from_mask(buttons);
            return Event::from_window(
                ev.event,
                Kind::Mouse(EventMouse {
                    x: ev.event_x.into(),
                    y: ev.event_y.into(),
                    button,
                    state: EventButtonState::Moved,
                    buttons,
                }),
                xev.timestamp(),
            );
        } else if let Some(cm) = xev.as_raw_client_message() {
            let proto = unsafe { cm.data.data32[0] };
            if proto == self.atoms.wm_delete_window {
                return Event::from_window(
                    cm.window,
                    Kind::Window(Win::CloseRequested),
                    xev.timestamp(),
                );
            }
        } else if let Some(ev) = xev.as_raw_enter() {
            return Event::from_window(ev.event, Kind::Window(Win::Entered), xev.timestamp());
        } else if let Some(ev) = xev.as_raw_leave() {
            return Event::from_window(ev.event, Kind::Window(Win::Left), xev.timestamp());
        } else if let Some(ev) = xev.as_raw_expose() {
            self.window_mark_redraw(ev.window);
            return Event::from_window(
                ev.window,
                Kind::Window(Win::RedrawRequested),
                xev.timestamp(),
            );
        } else if let Some(ev) = xev.as_raw_configure() {
            let window_id = ev.window;
            let (x, y, w, h) = (ev.x, ev.y, ev.width, ev.height);
            let ts = xev.timestamp();
            if let Some(delta) = self.window_update_configure(window_id, x, y, w, h) {
                if delta.moved {
                    let position = Position::<i32, 2>::new([x as i32, y as i32]);
                    self.queue.push(Event::from_window(
                        window_id,
                        EventKind::Window(EventWindow::Moved(Some(position))),
                        ts,
                    ));
                }
                if delta.resized {
                    let extent = Extent::<u32, 2>::new([w as u32, h as u32]);
                    self.queue.push(Event::from_window(
                        window_id,
                        EventKind::Window(EventWindow::Resized(Some(extent))),
                        ts,
                    ));
                }
            }
            return Event::None;
        } else {
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
        is! { let Some(ev) = self.pending.take(), return Some(ev) }
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
            is! { !ev.is_null(), self.pending = Some(ev) }
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
        is! { let Some(ev) = self.pending.take(), return Some(XEvent { raw: ev }) }
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
                    is! { (*kpev).detail == keycode && (*kpev).time == timestamp, return false }
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
