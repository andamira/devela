// devela::ui::event::queue
//
//! Defines [`EventQueue`].
//

use crate::{ConstInit, Event, is, whilst};

#[doc = crate::_tags!(event data_structure)]
/// A tiny, zero-alloc FIFO queue for `Event`.
#[doc = crate::_doc_location!("ui/event")]
///
/// This structure is intended for small CAP values (typically 1–4),
/// where linear operations are simpler and cheaper than managing head/tail indices.
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

    /// Attempts to push an event into the queue. (O(n))
    ///
    /// Returns `true` on success, or `false` if the queue is full.
    pub fn push(&mut self, ev: Event) -> bool {
        whilst! { i in 0..CAP; {
            if self.slots[i].is_none() { self.slots[i] = Some(ev); return true; }
        }}
        false
    }

    /// Pops the oldest event (FIFO). (O(CAP))
    pub fn pop(&mut self) -> Option<Event> {
        let first = self.slots[0].take()?;
        // shift everything left by one
        whilst! { i in 1..CAP; { self.slots[i - 1] = self.slots[i].take(); }}
        Some(first)
    }

    /// Returns the exact used capacity (O(n)).
    pub const fn len(&self) -> usize {
        let mut len = 0;
        whilst! { i in 0..CAP; {
            is![self.slots[i].is_none(), return len, len += 1];
        }}
        len
    }
    /// Returns the total capacity. (O(1))
    pub const fn capacity(&self) -> usize {
        CAP
    }
    /// Returns the remaining capacity. (O(n))
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }
    /// Returns true if no events are waiting. (O(1))
    pub const fn is_empty(&self) -> bool {
        self.slots[0].is_none() // if the first slot is empty, queue is empty.
    }
    /// Returns true if there's no remaining capacity. (O(1))
    pub const fn is_full(&self) -> bool {
        self.slots[CAP - 1].is_some() // if the last slot is occupied
    }
    /// Removes all events from the queue. (O(n))
    pub fn clear(&mut self) {
        whilst! { i in 0..CAP; { self.slots[i] = None; } }
    }
}
