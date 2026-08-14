// devela/src/data/topol/graph/mod.rs
//
#![doc = crate::_DOC_DATA_TOPOL_GRAPH!()] // public
#![doc = crate::_doc!(modules: crate::data::topol; graph)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Graphs describe connectivity over vertex domains independently of application values.
//!
//! Representation-specific semantics and guarantees are documented by each graph generator.
//

mod adj; // graph_adj!
// mod csr; /// graph_csr!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            adj::_all::*,
            // csr::_all::*,
        };
    }
}
