// devela/src/data/layout/stack/adt.rs
//
//! Defines the [`DataStack`] abstract data type.
//

#![allow(dead_code, reason = "feature-gated implementations")]

use crate::{DataCollection, NotEnoughElements, NotEnoughSpace};

#[doc = crate::_tags!(data_structure)]
/// An abstract *stack* data type.
#[doc = crate::_doc_meta!{location("data/layout/stack")}]
pub trait DataStack: DataCollection {
    /// Remove an element from the (back of the) stack.
    fn stack_pop(&mut self) -> Result<<Self as DataCollection>::Element, NotEnoughElements>;
    /// Add an element to the (back of the) stack.
    fn stack_push(
        &mut self,
        element: <Self as DataCollection>::Element,
    ) -> Result<(), NotEnoughSpace>;
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
