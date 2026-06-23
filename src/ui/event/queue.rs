// devela/src/ui/event/queue.rs
//
//! Defines [`EventQueue`].
//

use crate::{BufferRingU8, ConstInit, Event, MismatchedCapacity};

#[doc = crate::_tags!(event data_structure)]
/// A fixed-capacity FIFO queue for [`Event`].
#[doc = crate::_doc_meta!{
    location("ui/event"),
}]
///
/// Stores normalized events produced by a backend before they are consumed
/// by an application, UI, terminal, or runtime loop.
///
/// This queue is zero-alloc and uses ring-buffer indexing,
/// so pushing and popping are constant-time operations.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventQueue<const CAP: usize> {
    ring: BufferRingU8<Event, [Option<Event>; CAP]>,
}

impl<const CAP: usize> ConstInit for EventQueue<CAP> {
    const INIT: Self = Self::new();
}
impl<const CAP: usize> Default for EventQueue<CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAP: usize> EventQueue<CAP> {
    /// The maximum capacity supported by this queue.
    pub const MAX_CAPACITY: usize = u8::MAX as usize;

    /* constructors */

    /// Creates an empty event queue.
    ///
    /// # Panics
    /// Panics if `CAP > Self::MAX_CAPACITY`.
    #[must_use]
    pub const fn new() -> Self {
        assert!(CAP <= Self::MAX_CAPACITY, "EventQueue capacity does not fit in u8");
        Self { ring: BufferRingU8::new_empty() }
    }
    /// Creates an empty event queue.
    ///
    /// Returns an error if `CAP > Self::MAX_CAPACITY`.
    pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
        if CAP <= Self::MAX_CAPACITY {
            Ok(Self { ring: BufferRingU8::new_empty() })
        } else {
            Err(MismatchedCapacity::too_large(CAP, Self::MAX_CAPACITY))
        }
    }

    /* queries */

    /// Returns the number of queued events.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.ring.len() as usize
    }
    /// Returns the total event capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        CAP
    }
    /// Returns the remaining event capacity.
    #[must_use]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns `true` if no events are queued.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }
    /// Returns `true` if the queue is full.
    #[must_use]
    pub const fn is_full(&self) -> bool {
        self.ring.is_full()
    }

    /* mutating */

    /// Attempts to push an event to the back of the queue.
    ///
    /// Returns `true` on success, or `false` if the queue is full.
    pub fn push(&mut self, ev: Event) -> bool {
        self.ring.push_back(ev).is_ok()
    }
    /// Attempts to push an event to the back of the queue.
    ///
    /// Returns `Err(ev)` if the queue is full.
    pub fn try_push(&mut self, ev: Event) -> Result<(), Event> {
        self.ring.push_back(ev)
    }

    /// Pops the oldest event from the front of the queue.
    pub fn pop(&mut self) -> Option<Event> {
        self.ring.pop_front()
    }

    /// Removes all queued events.
    pub fn clear(&mut self) {
        self.ring.clear();
    }
}
