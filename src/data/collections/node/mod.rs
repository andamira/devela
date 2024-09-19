// devela::data::collections::node
//
//! Nodes.
//!
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

mod index;
mod node;
pub use {index::*, node::*};

pub(crate) mod all {
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use super::{index::*, node::*};
}
