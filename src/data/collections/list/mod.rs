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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::r#const::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
