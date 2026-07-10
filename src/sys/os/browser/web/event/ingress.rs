// devela/src/sys/os/browser/web/event/ingress.rs
//
//! Defines [`WebEventIngress`].

use crate::{Event, EventKind, EventKindTimed, EventQueue, EventTarget, Mem};
use crate::{WebEventKey, WebEventMouse, WebEventPointer, WebEventWheel};

#[doc = crate::_tags!(event web)]
/// Pollable ingress queue for normalized web events.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: WebEventIngress<0> = 8|64),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: WebEventIngress<1> = 68|544; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: WebEventIngress<0> = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: WebEventIngress<1> = 80|640; niche Option),
}]
/// Receives browser callback payloads, translates them into normalized
/// [`Event`] values, and stores them until polled by a runtime or UI surface.
///
/// # Timestamps
///
/// When the `time` feature is enabled, browser event timestamps are preserved as
/// [`Event::emitted`]. Without `time`, events are still queued normally,
/// but no emitted timestamp is attached.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WebEventIngress<const CAP: usize> {
    queue: EventQueue<CAP>,
    dropped: u32,
}
#[rustfmt::skip]
impl<const CAP: usize> WebEventIngress<CAP> {
    /* construct */

    /// Creates an empty web event ingress queue.
    pub const fn new() -> Self { Self { queue: EventQueue::new(), dropped: 0 } }

    /* query */

    /// Returns the number of queued events.
    #[must_use]
    pub const fn len(&self) -> usize { self.queue.len() }
    /// Returns the total event capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize { self.queue.capacity() }

    /// Returns whether no events are queued.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.queue.is_empty() }
    /// Returns whether the queue is full.
    #[must_use]
    pub const fn is_full(&self) -> bool { self.queue.is_full() }

    /// Returns the number of events dropped because the queue was full.
    #[must_use]
    pub const fn dropped(&self) -> u32 { self.dropped }

    /* modify */

    /// Returns and resets the number of dropped events.
    pub fn take_dropped(&mut self) -> u32 {
        Mem::take(&mut self.dropped)
    }

    /// Clears all queued events.
    pub fn clear(&mut self) { self.queue.clear(); }

    /* collect and poll */

    /// Collects queued events into `out`.
    ///
    /// Returns the number of written events.
    pub fn collect_events(&mut self, out: &mut [Event]) -> usize {
        let mut written = 0;
        while written < out.len() {
            let Some(event) = self.queue.pop() else { break };
            out[written] = event;
            written += 1;
        }
        written
    }
    /// Polls the next normalized event previously received through callbacks.
    ///
    /// This does not poll the browser event loop.
    pub fn poll_event(&mut self) -> Option<Event> {
        self.queue.pop()
    }

    /* push: already normalized */

    /// Pushes an already-normalized event.
    ///
    /// Returns `true` if stored, or `false` if the queue was full.
    pub fn push_event(&mut self, event: Event) -> bool {
        if self.queue.push(event) { true }
        else {
            self.dropped = self.dropped.saturating_add(1);
            false
        }
    }
    /// Attempts to push an already-normalized event.
    ///
    /// Returns the event back if the queue was full.
    pub fn try_push_event(&mut self, event: Event) -> Result<(), Event> {
        match self.queue.try_push(event) {
            Ok(()) => Ok(()),
            Err(event) => {
                self.dropped = self.dropped.saturating_add(1);
                Err(event)
            }
        }
    }

    /* push: event kind */

    /// Pushes an event kind with the given target.
    ///
    /// This constructs an event without an emitted timestamp.
    pub fn push_kind_with(&mut self, target: EventTarget, kind: EventKind) -> bool {
        self.push_event(Event::new_with(target, kind, None))
    }
    /// Pushes a timed event kind with the given target.
    ///
    /// The kind timestamp is stored as [`Event::emitted`].
    pub fn push_kind_timed_with(&mut self, target: EventTarget, kind: EventKindTimed) -> bool {
        self.push_event(Event::from_kind_timed_with(target, kind))
    }

    /* push: global web events */

    /// Pushes a keyboard event as a global event.
    pub fn push_key(&mut self, ev: WebEventKey) -> bool {
        self.push_key_with(EventTarget::Global, ev)
    }
    /// Pushes a mouse event as a global event.
    pub fn push_mouse(&mut self, ev: WebEventMouse) -> bool {
        self.push_mouse_with(EventTarget::Global, ev)
    }
    /// Pushes a pointer event as a global event.
    pub fn push_pointer(&mut self, ev: WebEventPointer) -> bool {
        self.push_pointer_with(EventTarget::Global, ev)
    }
    /// Pushes a wheel event as a global event.
    pub fn push_wheel(&mut self, ev: WebEventWheel) -> bool {
        self.push_wheel_with(EventTarget::Global, ev)
    }

    /* push: targeted web events */

    /// Pushes a keyboard event with the given target.
    pub fn push_key_with(&mut self, target: EventTarget, ev: WebEventKey) -> bool {
        self.push_kind_timed_with(target, ev.to_event_kind_timed())
    }
    /// Pushes a mouse event with the given target.
    pub fn push_mouse_with(&mut self, target: EventTarget, ev: WebEventMouse) -> bool {
        self.push_kind_timed_with(target, ev.to_event_kind_timed())
    }
    /// Pushes a pointer event with the given target.
    pub fn push_pointer_with(&mut self, target: EventTarget, ev: WebEventPointer) -> bool {
        self.push_kind_timed_with(target, ev.to_event_kind_timed())
    }
    /// Pushes a wheel event with the given target.
    pub fn push_wheel_with(&mut self, target: EventTarget, ev: WebEventWheel) -> bool {
        self.push_kind_timed_with(target, ev.to_event_kind_timed())
    }
}
