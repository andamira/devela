// devela::data::collections::list
//
//! Linked lists.
//!
//! Linked lists are sequentially accessed, homogeneous data structures.
//!
//! They enable efficient insertion and deletion at any position,
//! storing a sequence of elements of the same type, each pointing to the next.
//

mod r#const;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::r#const::*;
}
