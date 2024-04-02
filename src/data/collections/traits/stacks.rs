// devela::data::collections::traits::stacks
//
//! DataStack and DataDesta abstract data types
//
// TOC
// - define traits DataStack, DataDesta
// - impl for
//   - VecDeque

#[cfg(feature = "alloc")]
use crate::data::DataError;
use crate::data::{DataCollection, DataResult as Result};

/// An abstract *stack* data type.
pub trait DataStack: DataCollection {
    /// Remove an element from the (back of the) stack.
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element>;
    /// Add an element to the (back of the) stack.
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
}

/// An abstract *double-ended stack* data type.
pub trait DataDesta: DataStack {
    /// Remove an element from the front of the stack.
    fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element>;
    /// Add an element to the front of the stack.
    fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;

    /// Remove an element from the back of the stack (calls [`stack_pop`]).
    fn stack_pop_back(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.stack_pop()
    }
    /// Remove an element from the back of the stack (calls [`stack_push`]).
    fn stack_push_back(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.stack_push(element)
    }
}

/* impl for VecDeque */

#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataStack for crate::data::collections::reexports::VecDeque<T> {
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.pop_back().ok_or(DataError::NotEnoughElements(Some(1)))
    }
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_back(element); Ok(())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataDesta for crate::data::collections::reexports::VecDeque<T> {
    fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.pop_front().ok_or(DataError::NotEnoughElements(Some(1)))
    }
    fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_front(element); Ok(())
    }
}
