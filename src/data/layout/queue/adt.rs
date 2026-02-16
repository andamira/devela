// devela::data::layout::queue::adt
//
//! Defines the [`DataQueue`] & [`DataDeque`] abstract data types.
//
// TOC
// - define traits DataQueue, DataDeque
// - impl for:
//   - VecDeque

use crate::{DataCollection, NotEnoughElements, NotEnoughSpace};

#[doc = crate::_tags!(data_structure)]
/// An abstract *queue* data type.
#[doc = crate::_doc_location!("data/layout/queue")]
pub trait DataQueue: DataCollection {
    /// Remove an element from the (front of the) queue.
    /// # Errors
    /// Returns [`NotEnoughElements`] if there are not enough elements in the queue.
    fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements>;
    /// Add an element to the (back of the) queue.
    /// # Errors
    /// Returns [`NotEnoughSpace`] if there is not enough free space in the queue.
    fn queue_push(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace>;
}

#[doc = crate::_tags!(data_structure)]
/// An abstract *double-ended queue* data type.
#[doc = crate::_doc_location!("data/layout/queue")]
pub trait DataDeque: DataQueue {
    /// Remove an element from the back of the queue.
    /// # Errors
    /// Returns [`NotEnoughElements`] if there are not enough elements in the queue.
    fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements>;
    /// Add an element to the front of the queue.
    /// # Errors
    /// Returns [`NotEnoughSpace`] if there is not enough free space in the queue.
    fn queue_push_front(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace>;

    /// Remove an element from the front of the queue (calls [`queue_pop`][DataQueue::queue_pop]).
    /// # Errors
    /// Returns [`NotEnoughElements`] if there are not enough elements in the queue.
    fn queue_pop_front(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.queue_pop()
    }
    /// Remove an element from the back of the queue (calls [`queue_push`][DataQueue::queue_push]).
    /// # Errors
    /// Returns [`NotEnoughSpace`] if there is not enough free space in the queue.
    fn queue_push_back(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace> {
        self.queue_push(element)
    }
}

/* impls */

#[cfg(feature = "alloc")]
impl<T> DataQueue for crate::VecDeque<T> {
    fn queue_pop(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.pop_front().ok_or(NotEnoughElements(Some(1)))
    }
    fn queue_push(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace> {
        self.push_back(element);
        Ok(())
    }
}
#[cfg(feature = "alloc")]
impl<T> DataDeque for crate::VecDeque<T> {
    fn queue_pop_back(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.pop_back().ok_or(NotEnoughElements(Some(1)))
    }
    fn queue_push_front(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace> {
        self.push_front(element);
        Ok(())
    }
}
