// devela::data::collections::node
//
//! Nodes.
//!
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

mod index;
mod node;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{index::*, node::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
