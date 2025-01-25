// devela::data::collections::traits::stacks
//
//! Defines the [`DataStack`] & [`DataDesta`] abstract data types.
//
// TOC
// - define traits DataStack, DataDesta
// - impl for
//   - VecDeque

use crate::{DataCollection, NotEnoughElements, NotEnoughSpace};

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// An abstract *stack* data type.
pub trait DataStack: DataCollection {
    /// Remove an element from the (back of the) stack.
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements>;
    /// Add an element to the (back of the) stack.
    fn stack_push(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace>;
}

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// An abstract *double-ended stack* data type.
pub trait DataDesta: DataStack {
    /// Remove an element from the front of the stack.
    fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements>;
    /// Add an element to the front of the stack.
    fn stack_push_front(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace>;

    /// Remove an element from the back of the stack (calls [`DataStack::stack_pop`]).
    fn stack_pop_back(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.stack_pop()
    }
    /// Remove an element from the back of the stack (calls [`DataStack::stack_push`]).
    ///
    fn stack_push_back(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace> {
        self.stack_push(element)
    }
}

/* impl for VecDeque */

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataStack for crate::VecDeque<T> {
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.pop_back().ok_or(NotEnoughElements(Some(1)))
    }
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<(), NotEnoughSpace> {
        self.push_back(element); Ok(())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataDesta for crate::VecDeque<T> {
    fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements> {
        self.pop_front().ok_or(NotEnoughElements(Some(1)))
    }
    fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<(), NotEnoughSpace> {
        self.push_front(element); Ok(())
    }
}
