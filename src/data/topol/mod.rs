// devela/src/data/topol/mod.rs
//
#![doc = crate::_DOC_DATA_TOPOL!()] // public
#![doc = crate::_doc!(modules: crate::data; topol)] // graph, spatial
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Topology describes connectivity, adjacency, and ordered relations.
//!
//! It answers how values are related, independently of their physical
//! arrangement, ownership, or representation.
//!
//! A topology may be represented through references, indices, handles,
//! or explicit nodes. Changing that representation need not change the
//! relations being expressed.
//!
//! It does not assign identity, retain values, or define geometric distance;
//! those concerns belong to [`data::id`](crate::data::id),
//! [`data::store`](crate::data::store), and [`geom`](crate::geom).
//!
//! - [`ConstList`] represents immutable linear succession through shared links.
//! - [`LinkedList`] provides mutable owned linkage when allocation is available.
//

mod linked; // ConstList[Item], LinkedList
// mod ord;
// mod span;

// #[cfg(_graph··)]
// pub mod graph;
// #[cfg(_node··)]
// pub mod node;
// pub mod spatial;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            linked::_all::*,
            // ord::_all::*,
            // span::_all::*,
        };
    }
    _pub_mods {
        // pub use super::{
        //     spatial::_all::*,
        // };

        // #[cfg(_graph··)]
        // pub use super::graph::*;
        // #[cfg(_node··)]
        // pub use super::node::_all::*;
    }
}
