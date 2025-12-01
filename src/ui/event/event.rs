// devela::ui::event::event
//
//! Defines [`Event`], [`EventTag`] and [`EventKind`].
//
// TOC
// - struct Event
// - enum EventTag
// - enum EventKind

use crate::{
    ConstInit, EventKey, EventMouse, EventPointer, EventTimestamp, EventWindow, NonZeroU64,
};

/// A fully-typed event with optional timing and metadata.
///
/// `Event` separates three notions of time/state:
/// - **emitted:** when the backend or OS generated the event.
/// - **processed:** when the engine or input system handled it.
/// - **count:** the update-loop tick in which the event was observed.
///
/// These values cover different needs: real-time latency, internal
/// scheduling, and frame-deterministic logic.
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    /// The concrete event payload.
    pub kind: EventKind,

    /// When the backend or OS emitted the event, if provided.
    ///
    /// Represents external, real-time origin. Useful for latency
    /// measurements, input smoothing, debouncing, or repeat logic.
    pub emitted: Option<EventTimestamp>,

    /// When the input system or engine processed this event.
    ///
    /// Marks the moment it entered your pipeline. This is distinct
    /// from `emitted`: the OS timestamp reflects *when it happened*,
    /// while `processed` reflects *when you handled it*. Useful for
    /// profiling and internal scheduling.
    pub processed: Option<EventTimestamp>,

    /// The update-loop counter snapshot when this event was observed.
    ///
    /// A monotonic, frame-based logical time. This does not measure
    /// real time, only "which tick" the change occurred in.
    /// Useful for deterministic game logic or state comparisons.
    pub count: Option<NonZeroU64>,
}
impl ConstInit for Event {
    const INIT: Event = Self::None;
}

/// A lightweight, data-less identifier for `EventKind`.
///
/// Used when only the *category* of the event is relevant and the
/// payload of the variant is not needed.
///
/// This avoids carrying the full `EventKind` and is suitable for
/// quick routing, filtering, or statistics.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventTag {
    /// No event or an empty placeholder.
    #[default]
    None,
    /// A keyboard event.
    Key,
    /// A mouse button or motion event.
    Mouse,
    /// A pointing-device event (mouse, pen, stylus, touch).
    Pointer,
    /// A window-related event (focus, resize, etc.).
    Window,
}

/// An enumeration of concrete event variants.
///
/// High-level, typed grouping of input and window interactions.
/// Backends normalize their raw events into these variants.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
//   - https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs
// - https://docs.rs/notcurses/latest/notcurses/struct.Input.html
//   - https://docs.rs/notcurses/latest/notcurses/enum.Received.html
// - https://docs.rs/sdl2/latest/sdl2/event/enum.Event.html
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventKind {
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A keyboard event.
    Key(EventKey),

    /// A mouse event.
    Mouse(EventMouse),

    /// A pointing-device event (mouse, pen, stylus, or multi-touch).
    Pointer(EventPointer),

    /// A window event.
    Window(EventWindow),
    // /// A gamepad event.
    // Gamepad(GamepadEvent),

    // /// A midi event.
    // Midi(MidiEvent),
}
impl ConstInit for EventKind {
    const INIT: Self = Self::None;
}
impl EventKind {
    /// Returns the coarse category of this event without its associated data.
    ///
    /// Useful for fast dispatch or matching on *type* rather than *content*.
    pub const fn tag(&self) -> EventTag {
        match self {
            Self::None => EventTag::None,
            Self::Key(_) => EventTag::Key,
            Self::Mouse(_) => EventTag::Mouse,
            Self::Pointer(_) => EventTag::Pointer,
            Self::Window(_) => EventTag::Window,
        }
    }
}

/* impls */

#[rustfmt::skip]
impl Event {
    /* constructors */

    /// A non-existent event.
    ///
    /// Equivalent to an empty placeholder with no timestamps or payload.
    #[allow(non_upper_case_globals)]
    pub const None: Event = Self {
        kind: EventKind::None,
        emitted: None,
        processed: None,
        count: None,
    };

    /// Creates a new event with a `kind` and an optional backend `emitted` timestamp.
    ///
    /// `processed` and `count` are left unset and should be filled by the engine.
    #[inline(always)]
    pub const fn new(kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, emitted, processed: None, count: None }
    }

    /* setters */

    /// Marks when the engine processed this event.
    ///
    /// Distinct from `emitted`: this represents internal handling time.
    #[inline(always)]
    pub const fn mark_processed(&mut self, ts: EventTimestamp) { self.processed = Some(ts); }

    /// Sets the update-loop count snapshot for this event.
    ///
    /// Zero is ignored and leaves the field unset.
    #[inline(always)]
    pub const fn mark_count(&mut self, count: u64) {
        if let Some(nz) = NonZeroU64::new(count) { self.count = Some(nz); }
    }

    /// Clears the loop-count metadata, restoring it to the unset state.
    #[inline(always)]
    pub fn clear_count(&mut self) { self.count = None; }

    /// Sets both the processed timestamp and loop count in one step.
    ///
    /// Used by runtimes that stamp events during dispatch.
    #[inline(always)]
    pub const fn finalize(&mut self, processed: EventTimestamp, count: u64) {
        self.processed = Some(processed);
        self.mark_count(count);
    }

    /* queries */

    /// Returns the timestamp of the moment the event was emitted, or `None` if unknown.
    #[must_use] #[inline(always)]
    pub const fn emitted(&self) -> Option<EventTimestamp> { self.emitted }

    /// Returns the timestamp of the moment the event was processed, or `None` if unknown.
    #[must_use] #[inline(always)]
    pub const fn processed(&self) -> Option<EventTimestamp> { self.processed }

    /// Returns the loop-count snapshot when this event was observed.
    ///
    /// Returns `0` if the count is unset.
    #[inline(always)]
    pub fn count(&self) -> u64 { if let Some(nz) = self.count { nz.get() } else { 0 } }

    /// Returns the kind of event.
    #[must_use] #[inline(always)]
    pub const fn kind(&self) -> &EventKind { &self.kind }

    /// Returns the categorical tag of this event.
    #[must_use] #[inline(always)]
    pub const fn tag(&self) -> EventTag { self.kind.tag() }

    //

    /// Whether there's no event.
    #[must_use] #[inline(always)]
    pub const fn is_none(&self) -> bool { self.kind.is_none() }

    /// Whether it's some event.
    #[must_use] #[inline(always)]
    pub const fn is_some(&self) -> bool { self.kind.is_some() }

    /// Whether it's a keyboard event.
    #[must_use] #[inline(always)]
    pub const fn is_key(&self) -> bool { self.kind.is_key() }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub const fn is_mouse(&self) -> bool { self.kind.is_mouse() }

    /// Whether it's a pointer event.
    #[must_use] #[inline(always)]
    pub const fn is_pointer(&self) -> bool { self.kind.is_pointer() }

    /// Whether it's a window event.
    #[must_use] #[inline(always)]
    pub const fn is_window(&self) -> bool { self.kind.is_window() }

    // /// Returns true if it's a gamepad event.
    // pub const fn is_gamepad(&self) -> bool { self.kind.is_gamepad() }

    // /// Returns true if it's a midi event.
    // pub const fn is_midi(&self) -> bool { self.kind.is_midi() }

    //

    /// Returns some keyboard event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_key(&self) -> Option<&EventKey> { self.kind.some_key() }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_mouse(&self) -> Option<&EventMouse> { self.kind.some_mouse() }

    /// Returns some pointer event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_pointer(&self) -> Option<&EventPointer> { self.kind.some_pointer() }

    /// Returns some window event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_window(&self) -> Option<&EventWindow> { self.kind.some_window() }

    // /// Returns some gamepad event, if that's the kind.
    // pub const fn some_gamepad(&self) -> Option<&GamepadEvent> { self.kind.some_gamepad() }

    // /// Returns some midi event, if that's the kind.
    // pub const fn some_midi(&self) -> Option<&MidiEvent> { self.kind.some_midi() }
}

impl From<EventKind> for Event {
    fn from(kind: EventKind) -> Event {
        Self { kind, emitted: None, processed: None, count: None }
    }
}

#[rustfmt::skip]
impl EventKind {
    /// Whether there's no event.
    #[must_use] #[inline(always)]
    pub const fn is_none(&self) -> bool { matches![self, EventKind::None] }

    /// Whether there's some event.
    #[must_use] #[inline(always)]
    pub const fn is_some(&self) -> bool { !matches![self, EventKind::None] }

    /// Whether it's a window event.
    #[must_use] #[inline(always)]
    pub const fn is_window(&self) -> bool { matches![self, EventKind::Window(_)] }

    /// Whether it's a keyboard event.
    #[must_use] #[inline(always)]
    pub const fn is_key(&self) -> bool { matches![self, EventKind::Key(_)] }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub const fn is_mouse(&self) -> bool { matches![self, EventKind::Mouse(_)] }

    /// Whether it's a pointer event.
    #[must_use] #[inline(always)]
    pub const fn is_pointer(&self) -> bool { matches![self, EventKind::Pointer(_)] }

    // /// Returns `true` if it's a keyboard event.
    // pub const fn is_midi(&self) -> bool { matches![self, EventKind::Midi(_)] }

    // /// Returns `true` if it's a mouse event.
    // pub const fn is_gamepad(&self) -> bool { matches![self, EventKind::Gamepad(_)] }

    /// Returns some window event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_window(&self) -> Option<&EventWindow> {
        if let EventKind::Window(e) = &self { Some(e) } else { None }
    }

    /// Returns some keyboard event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_key(&self) -> Option<&EventKey> {
        if let EventKind::Key(e) = &self { Some(e) } else { None }
    }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_mouse(&self) -> Option<&EventMouse> {
        if let EventKind::Mouse(e) = &self { Some(e) } else { None }
    }

    /// Returns some pointer event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_pointer(&self) -> Option<&EventPointer> {
        if let EventKind::Pointer(e) = &self { Some(e) } else { None }
    }

    // /// Returns some gamepad event, if that's the kind.
    // pub const fn some_gamepad(&self) -> Option<&GamepadEvent> {
    //     if let EventKind::Gamepad(e) = &self { Some(e) } else { None }
    // }

    // /// Returns some midi event, if that's the kind.
    // pub const fn some_midi(&self) -> Option<&MidiEvent> {
    //     if let EventKind::Midi(e) = &self { Some(e) } else { None }
    // }
}
