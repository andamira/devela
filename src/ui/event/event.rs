// devela::ui::event::event
//
//! Defines [`Event`].
//

use crate::{
    ConstInit, EventKey, EventKind, EventMouse, EventPointer, EventTag, EventTarget,
    EventTimestamp, EventWindow, NonZeroU64,
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
    /// The target associated with the event.
    pub target: EventTarget,

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

#[rustfmt::skip]
impl Event {
    /* constructors */

    /// A non-existent event.
    ///
    /// Equivalent to an empty placeholder with no timestamps or payload.
    #[allow(non_upper_case_globals)]
    pub const None: Event = Self {
        kind: EventKind::None,
        target: EventTarget::Global,
        emitted: None,
        processed: None,
        count: None,
    };

    /// Creates a new event with a `kind` and an optional backend `emitted` timestamp.
    ///
    /// The `target` is set to [`Global`][EventTarget::Global],
    /// while `processed` and `count` are left unset and should be filled by the engine.
    #[inline(always)]
    pub const fn new(kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, target: EventTarget::Global, emitted, processed: None, count: None }
    }

    /// Creates a new event with the given `target`, `kind`,
    /// and an optional backend `emitted` timestamp.
    ///
    /// `processed` and `count` are left unset and should be filled by the engine.
    #[inline(always)]
    pub const fn new_with(target: EventTarget, kind: EventKind, emitted: Option<EventTimestamp>)
        -> Event {
        Self { kind, target, emitted, processed: None, count: None }
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
        Self {
            kind,
            target: EventTarget::Global,
            emitted: None,
            processed: None,
            count: None,
        }
    }
}
