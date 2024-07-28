// devela::data::collections::traits::queues
//
//! DataQueue, DataDeque abstract data type
//
// TOC
// - define traits DataQueue, DataDeque
// - impl for:
//   - VecDeque

use crate::data::{DataCollection, DataResult as Result};
#[cfg(feature = "alloc")]
use crate::data::{DataError::NotEnoughElements, VecDeque};

/// An abstract *queue* data type.
pub trait DataQueue: DataCollection {
    /// Remove an element from the (front of the) queue.
    fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element>;
    /// Add an element to the (back of the) queue.
    fn queue_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
}

/// An abstract *double-ended queue* data type.
pub trait DataDeque: DataQueue {
    /// Remove an element from the back of the queue.
    fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element>;
    /// Add an element to the front of the queue.
    fn queue_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;

    /// Remove an element from the front of the queue (calls [`queue_pop`][DataQueue::queue_pop]).
    fn queue_pop_front(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.queue_pop()
    }
    /// Remove an element from the back of the queue (calls [`queue_push`][DataQueue::queue_push]).
    fn queue_push_back(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.queue_push(element)
    }
}

/* impls */

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataQueue for VecDeque<T> {
    fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.pop_front().ok_or(NotEnoughElements(Some(1)))
    }
    fn queue_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_back(element); Ok(())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataDeque for VecDeque<T> {
    fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.pop_back().ok_or(NotEnoughElements(Some(1)))
    }
    fn queue_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_front(element); Ok(())
    }
}
