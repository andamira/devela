// devela::ui::event::queue
//
//! Defines [`EventQueue`].
//

use crate::{ConstInit, Event, is, whilst};

/// A tiny, zero-alloc FIFO queue for `Event`.
///
/// Capacity is fixed at compile time. This structure is intended
/// for small CAP values (typically 1–4), where linear operations
/// are simpler and cheaper than managing head/tail indices.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventQueue<const CAP: usize> {
    slots: [Option<Event>; CAP],
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
    /// Creates an empty queue.
    pub const fn new() -> Self {
        Self { slots: [const { None }; CAP] }
    }

    /// Attempts to push an event into the queue.
    ///
    /// Returns `true` on success.
    /// Returns `false` if the queue is full.
    #[inline]
    pub fn push(&mut self, ev: Event) -> bool {
        whilst! { i in 0..CAP; {
            if self.slots[i].is_none() { self.slots[i] = Some(ev); return true; }
        }}
        false
    }

    /// Pops the oldest event (FIFO).
    pub fn pop(&mut self) -> Option<Event> {
        let first = self.slots[0].take()?; // it's the oldest slot
        whilst! { i in 0..CAP; {
            self.slots[i - 1] = self.slots[i].take(); // shift everything left by one
        }}
        Some(first)
    }

    /// Returns true if no events are waiting.
    pub const fn is_empty(&self) -> bool {
        self.slots[0].is_none() // if the first slot is empty, queue is empty.
    }
}
