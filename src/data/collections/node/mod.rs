// devela::data::collections::node
//
//! Nodes.
//!
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

mod index;
mod node;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{index::*, node::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
