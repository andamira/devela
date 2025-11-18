// devela::ui::event
//
//! Defines [`Event`] and [`EventKind`].
//

use crate::{EventKey, EventMouse, EventPointer, EventTimestamp, EventWindow};

/// An event.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    ///
    pub kind: EventKind,

    ///
    pub emitted: Option<EventTimestamp>,
    // processed: Option<EventTimestamp>, // TODO
    // count: Option<EventCounter>, // gilrs
}

/// A an enumeration of kinds of events.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
//   - https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs
// - https://docs.rs/notcurses/latest/notcurses/struct.Input.html
//   - https://docs.rs/notcurses/latest/notcurses/enum.Received.html
// - https://docs.rs/sdl2/latest/sdl2/event/enum.Event.html
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventKind {
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A keyboard event.
    Key(EventKey),

    /// A mouse event.
    Mouse(EventMouse),

    /// A mouse event.
    Pointer(EventPointer),

    /// A window event.
    Window(EventWindow),
    // /// A midi event.
    // Midi(MidiEvent),

    // /// A gamepad event.
    // Gamepad(GamepadEvent),
}

/* impls */

#[rustfmt::skip]
impl Event {
    /// A `None` event.
    #[allow(non_upper_case_globals)]
    pub const None: Event = Event { kind: EventKind::None, emitted: None };

    /// Return a new event with a `kind` and an optional `emmitted` timestamp.
    #[inline(always)]
    pub fn new(kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, emitted }
    }

    /* queries */

    /// Returns some timestamp of the moment the event was emitted, or `None` if it's unknown.
    #[must_use] #[inline(always)]
    pub fn emitted(&self) -> Option<EventTimestamp> { self.emitted }

    /// Returns the kind of event.
    #[must_use] #[inline(always)]
    pub fn kind(&self) -> &EventKind { &self.kind }

    /// Whether` if there's no event.
    #[must_use] #[inline(always)]
    pub fn is_none(&self) -> bool { self.kind.is_none() }

    /// Whether` if it's some event.
    #[must_use] #[inline(always)]
    pub fn is_some(&self) -> bool { self.kind.is_some() }

    /// Whether it's a keyboard event.
    #[must_use] #[inline(always)]
    pub fn is_key(&self) -> bool { self.kind.is_key() }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub fn is_mouse(&self) -> bool { self.kind.is_mouse() }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub fn is_pointer(&self) -> bool { self.kind.is_pointer() }

    /// Whether it's a window event.
    #[must_use] #[inline(always)]
    pub fn is_window(&self) -> bool { self.kind.is_window() }

    // /// Returns true if it's a gamepad event.
    // pub fn is_gamepad(&self) -> bool { self.kind.is_gamepad() }

    // /// Returns true if it's a midi event.
    // pub fn is_midi(&self) -> bool { self.kind.is_midi() }

    //

    /// Returns some keyboard event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_key(&self) -> Option<&EventKey> { self.kind.some_key() }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_mouse(&self) -> Option<&EventMouse> { self.kind.some_mouse() }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_pointer(&self) -> Option<&EventPointer> { self.kind.some_pointer() }

    /// Returns some window event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_window(&self) -> Option<&EventWindow> { self.kind.some_window() }

    // /// Returns some gamepad event, if that's the kind.
    // pub fn some_gamepad(&self) -> Option<&GamepadEvent> { self.kind.some_gamepad() }

    // /// Returns some midi event, if that's the kind.
    // pub fn some_midi(&self) -> Option<&MidiEvent> { self.kind.some_midi() }
}

impl From<EventKind> for Event {
    fn from(kind: EventKind) -> Event {
        Self { kind, emitted: None }
    }
}

#[rustfmt::skip]
impl EventKind {
    /// Whether there's some event.
    #[must_use] #[inline(always)]
    pub fn is_some(&self) -> bool { !matches![self, EventKind::None] }

    /// Whether there's no event.
    #[must_use] #[inline(always)]
    pub fn is_none(&self) -> bool { matches![self, EventKind::None] }

    /// Whether it's a window event.
    #[must_use] #[inline(always)]
    pub fn is_window(&self) -> bool { matches![self, EventKind::Window(_)] }

    /// Whether it's a keyboard event.
    #[must_use] #[inline(always)]
    pub fn is_key(&self) -> bool { matches![self, EventKind::Key(_)] }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub fn is_mouse(&self) -> bool { matches![self, EventKind::Mouse(_)] }

    /// Whether it's a pointer event.
    #[must_use] #[inline(always)]
    pub fn is_pointer(&self) -> bool { matches![self, EventKind::Pointer(_)] }

    // /// Returns `true` if it's a keyboard event.
    // pub fn is_midi(&self) -> bool { matches![self, EventKind::Midi(_)] }

    // /// Returns `true` if it's a mouse event.
    // pub fn is_gamepad(&self) -> bool { matches![self, EventKind::Gamepad(_)] }

    /// Returns some window event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_window(&self) -> Option<&EventWindow> {
        if let EventKind::Window(e) = &self { Some(e) } else { None }
    }

    /// Returns some keyboard event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_key(&self) -> Option<&EventKey> {
        if let EventKind::Key(e) = &self { Some(e) } else { None }
    }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_mouse(&self) -> Option<&EventMouse> {
        if let EventKind::Mouse(e) = &self { Some(e) } else { None }
    }

    /// Returns some pointer event, if that's the kind.
    #[must_use] #[inline(always)]
    pub fn some_pointer(&self) -> Option<&EventPointer> {
        if let EventKind::Pointer(e) = &self { Some(e) } else { None }
    }

    // /// Returns some gamepad event, if that's the kind.
    // pub fn some_gamepad(&self) -> Option<&GamepadEvent> {
    //     if let EventKind::Gamepad(e) = &self { Some(e) } else { None }
    // }

    // /// Returns some midi event, if that's the kind.
    // pub fn some_midi(&self) -> Option<&MidiEvent> {
    //     if let EventKind::Midi(e) = &self { Some(e) } else { None }
    // }
}
