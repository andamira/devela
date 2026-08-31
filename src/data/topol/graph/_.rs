// devela/src/data/topol/graph/_.rs
//
#![doc = crate::_DOC_DATA_TOPOL_GRAPH!()] // public
#![doc = crate::_doc!(modules: crate::data::topol; graph)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Graphs describe directed connectivity over vertex domains
//! independently of application values.
//!
//! Two generated representations are provided:
//! - [`graph_adj!`] builds mutable, append-oriented adjacency graphs.
//! - [`graph_csr!`] builds immutable, packed compressed sparse row (CSR) graphs.
//!
//! Both store topology only and generate typed vertex and edge handles;
//! application payloads may be kept separately. Self-loops,
//! parallel edges, and cycles are allowed.
//!
//! Each representation supports static or allocating storage.
//! Representation-specific ordering, mutation, and handle semantics are
//! documented by each graph generator.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod_ adj; // graph_adj!
    mod_ csr; // graph_csr!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            adj::_all::*,
            csr::_all::*,
        };
    }
    _hidden {
        pub use super::{
            adj::_hidden::*,
            csr::_hidden::*,
        };
    }
}
