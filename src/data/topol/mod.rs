// devela/src/data/topol/mod.rs
//
#![doc = crate::_DOC_DATA_TOPOL!()] // public
#![doc = crate::_doc!(modules: crate::data; topol: graph)] // spatial
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Topology describes structural relations
//! independently of the values participating in them.
//!
//! It answers how positions or identities are connected, adjacent,
//! or otherwise related without determining where their values
//! are stored or how long those values live.
//!
//! A topology may be represented through indices, handles, links,
//! or compact relation tables. Changing that representation need
//! not change the relations being expressed.
//!
//! Topology does not assign durable identity, retain application values,
//! or define geometric measurement; those concerns belong to
//! [`data::id`](crate::data::id), [`data::store`](crate::data::store),
//! and [`geom`](crate::geom).
//
// //! Restrictions such as acyclicity, uniqueness of edges, or exclusion of
// //! self-loops are graph properties rather than assumptions of topology itself.
//!
//! - [`graph`] describes arbitrary connectivity over vertex domains.
//! - Spatial topology may describe locality, neighborhood, and partitioning
//!   independently of geometric coordinates and metrics.
//

pub mod graph; // WIP
// mod ord; // TODO
// pub mod spatial; // TODO

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            // ord::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            graph::_all::*,
            // spatial::_all::*,
        };
    }
}
