// devela::data::collections::traits::stacks
//
//! DataStack and DataDestack abstract data types
//
// TOC
// - define traits DataStack, DataDestack
// - impl for
//   - VecDeque

use crate::data::{DataCollection, DataError as E, DataResult as Result};

/// An abstract *stack* data type.
pub trait DataStack: DataCollection {
    /// Remove an element from the (back of the) stack.
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element>;
    /// Add an element to the (back of the) stack.
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()>;
}

/// An abstract *double-ended stack* data type.
pub trait DataDestack: DataStack {
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
        self.pop_back().ok_or(E::NotEnoughElements(Some(1)))
    }
    fn stack_push(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_back(element); Ok(())
    }
}
#[rustfmt::skip]
#[cfg(feature = "alloc")]
impl<T> DataDestack for crate::data::collections::reexports::VecDeque<T> {
    fn stack_pop_front(&mut self) -> Result<<Self as DataCollection>::Element> {
        self.pop_front().ok_or(E::NotEnoughElements(Some(1)))
    }
    fn stack_push_front(&mut self, element: <Self as DataCollection>::Element) -> Result<()> {
        self.push_front(element); Ok(())
    }
}
