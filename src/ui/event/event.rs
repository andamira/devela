// devela/src/ui/event/event.rs
//
//! Defines [`Event`].
//
// Boundary: EventTarget addresses host surfaces and devices,
// not frame-resolved UiId values.
//

use crate::{_impl_init, NonZeroU64};
use crate::{
    DeviceId, EventKey, EventKind, EventKindTimed, EventMouse, EventPointer, EventTag, EventTagSet,
    EventTarget, EventTimestamp, EventWheel, EventWindow, WindowId,
};

#[doc = crate::_tags!(event)]
/// A fully-typed event with optional timing and metadata.
#[doc = crate::_doc_meta!{
    location("ui/event", struct Event),
    #[cfg(target_pointer_width = "32")]
    test_size_of(Event = 60|480; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(Event = 64|512; niche Option),
}]
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
_impl_init![Self::None => Event];

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

    /// Creates a new global event with a `kind` and an optional backend `emitted` timestamp.
    ///
    /// The `target` is set to [`Global`][EventTarget::Global],
    /// while `processed` and `count` are left unset and should be filled by the engine.
    pub const fn new(kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, target: EventTarget::Global, emitted, processed: None, count: None }
    }

    /// Creates a new event with the given `target`, `kind`,
    /// and an optional backend `emitted` timestamp.
    ///
    /// `processed` and `count` are left unset and should be filled by the engine.
    pub const fn new_with(target: EventTarget, kind: EventKind, emitted: Option<EventTimestamp>)
        -> Event {
        Self { kind, target, emitted, processed: None, count: None }
    }

    /// Creates a new window event with a `kind` and an optional backend `emitted` timestamp.
    ///
    /// The `target` is set to [`Window`][EventTarget::Window],
    /// while `processed` and `count` are left unset and should be filled by the engine.
    pub fn from_window(id: impl Into<WindowId>,
        kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, target: EventTarget::Window(id.into()), emitted, processed: None, count: None }
    }

    /// Creates a new device event with a `kind` and an optional backend `emitted` timestamp.
    ///
    /// The `target` is set to [`Device`][EventTarget::Device],
    /// while `processed` and `count` are left unset and should be filled by the engine.
    pub fn from_device(id: impl Into<DeviceId>,
        kind: EventKind, emitted: Option<EventTimestamp>) -> Event {
        Self { kind, target: EventTarget::Device(id.into()), emitted, processed: None, count: None }
    }

    /// Creates a new event with the given `target and timed `kind`.
    ///
    /// `processed` and `count` are left unset and should be filled by the engine.
    pub fn from_kind_timed_with(target: EventTarget, kind: EventKindTimed) -> Event {
        Self { kind: kind.value, target, emitted: kind.time, processed: None, count: None }
    }

    /* setters */

    /// Marks when the engine processed this event.
    ///
    /// Distinct from `emitted`: this represents internal handling time.
    pub const fn mark_processed(&mut self, ts: EventTimestamp) { self.processed = Some(ts); }

    /// Sets the update-loop count snapshot for this event.
    ///
    /// Zero is ignored and leaves the field unset.
    pub const fn mark_count(&mut self, count: u64) {
        if let Some(nz) = NonZeroU64::new(count) { self.count = Some(nz); }
    }

    /// Clears the loop-count metadata, restoring it to the unset state.
    pub fn clear_count(&mut self) { self.count = None; }

    /// Sets both the processed timestamp and loop count in one step.
    ///
    /// Used by runtimes that stamp events during dispatch.
    pub const fn finalize(&mut self, processed: EventTimestamp, count: u64) {
        self.processed = Some(processed);
        self.mark_count(count);
    }

    /* queries */

    #[must_use]
    /// Returns the timestamp of the moment the event was emitted, or `None` if unknown.
    pub const fn emitted(&self) -> Option<EventTimestamp> { self.emitted }

    #[must_use]
    /// Returns the timestamp of the moment the event was processed, or `None` if unknown.
    pub const fn processed(&self) -> Option<EventTimestamp> { self.processed }

    /// Returns the loop-count snapshot when this event was observed.
    ///
    /// Returns `0` if the count is unset.
    pub fn count(&self) -> u64 { if let Some(nz) = self.count { nz.get() } else { 0 } }

    #[must_use]
    /// Returns the kind of event.
    pub const fn kind(&self) -> &EventKind { &self.kind }

    #[must_use]
    /// Returns the categorical tag of this event.
    pub const fn tag(&self) -> EventTag { self.kind.tag() }

    #[must_use]
    /// Returns whether this event has `tag`.
    pub const fn has_tag(&self, tag: EventTag) -> bool {
        self.kind.has_tag(tag)
    }

    #[must_use]
    /// Returns whether this event belongs to `set`.
    pub const fn is_in(&self, set: EventTagSet) -> bool {
        self.kind.is_in(set)
    }

    //

    #[must_use]
    /// Whether there's no event.
    pub const fn is_none(&self) -> bool { self.kind.is_none() }

    #[must_use]
    /// Whether it's some event.
    pub const fn is_some(&self) -> bool { self.kind.is_some() }

    #[must_use]
    /// Whether it's a window event.
    pub const fn is_window(&self) -> bool { self.kind.is_window() }

    #[must_use]
    /// Whether it's a keyboard event.
    pub const fn is_key(&self) -> bool { self.kind.is_key() }

    #[must_use]
    /// Whether it's a mouse event.
    pub const fn is_mouse(&self) -> bool { self.kind.is_mouse() }

    #[must_use]
    /// Whether it's a pointer event.
    pub const fn is_pointer(&self) -> bool { self.kind.is_pointer() }

    #[must_use]
    /// Whether it's a wheel event.
    pub const fn is_wheel(&self) -> bool { matches![self.kind, EventKind::Wheel(_)] }

    // /// Returns true if it's a gamepad event.
    // pub const fn is_gamepad(&self) -> bool { self.kind.is_gamepad() }

    // /// Returns true if it's a midi event.
    // pub const fn is_midi(&self) -> bool { self.kind.is_midi() }

    //

    #[must_use]
    /// Returns some window event, if that's the kind.
    pub const fn some_window(&self) -> Option<&EventWindow> { self.kind.some_window() }

    #[must_use]
    /// Returns some keyboard event, if that's the kind.
    pub const fn some_key(&self) -> Option<&EventKey> { self.kind.some_key() }

    #[must_use]
    /// Returns some mouse event, if that's the kind.
    pub const fn some_mouse(&self) -> Option<&EventMouse> { self.kind.some_mouse() }

    #[must_use]
    /// Returns some pointer event, if that's the kind.
    pub const fn some_pointer(&self) -> Option<&EventPointer> { self.kind.some_pointer() }

    #[must_use]
    /// Returns some wheel event, if that's the kind.
    pub const fn some_wheel(&self) -> Option<&EventWheel> {
        if let EventKind::Wheel(e) = &self.kind { Some(e) } else { None }
    }

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
impl From<EventKindTimed> for Event {
    fn from(kind: EventKindTimed) -> Event {
        Self {
            kind: kind.value,
            target: EventTarget::Global,
            emitted: kind.time,
            processed: None,
            count: None,
        }
    }
}
