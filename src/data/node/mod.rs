// devela::data::node
//
//! Abstractions for structured relationships.
//!
//! Nodes are a basic unit used to build more complex structures,
//! like linked lists, graphs and trees.
//

// mod index;
// mod node;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // pub use super::index::*; // WIP
        // pub use super::node::*; // WIP
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
