// devela::data::collections
//
//! Data collections.
//!
#![doc = crate::doc_!(extends: array, collections, vec)]
//

mod array;
mod list;
mod reexports;
mod traits;

#[cfg(_destaque_·)]
mod destaque;
// #[cfg(_graph_·)]
// mod graph;
// #[cfg(_node_·)]
// mod node;
#[cfg(_stack_·)]
mod stack;
#[cfg(feature = "_tuple")]
mod tuple; // Tuple, TupleFmt
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod vec;

/* structural access */

#[allow(unused_imports)]
pub use doc_inline::*;

mod doc_inline {
    #[cfg(_destaque_·)]
    pub use super::destaque::all::*;
    #[allow(unused_imports)]
    pub use super::{array::all::*, list::all::*, reexports::*, traits::*};
    // #[cfg(_graph_·)]
    // pub use super::graph::*;
    // #[cfg(_node_·)]
    // pub use super::node::*;
    #[cfg(_stack_·)]
    pub use super::stack::all::*;
    #[cfg(feature = "_tuple")]
    pub use super::tuple::*;
    #[cfg(feature = "alloc")]
    pub use super::vec::all::*;
}
pub(super) mod all {
    #[doc(inline)]
    pub use super::doc_inline::*;
}
