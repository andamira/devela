// devela/src/data/topol/_.rs
//
#![doc = crate::_DOC_DATA_TOPOL!()] // public
#![doc = crate::_doc!(modules: crate::data; topol: graph, link, ord)] // spatial
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Topology describes structural relations
//! independently of the values participating in them.
//!
//! It expresses connectivity, adjacency, succession, and related structure.
//!
//! The same relations can be represented through indices,
//! handles, links, or compact relation tables.
//

crate::mods_in! {
    pub mod_ graph; // Graph connectivity over indexed vertex domains
    pub mod_ link; // Named fixed-arity links over externally interpreted targets
    pub mod_ ord; // Constrained ordering relations
    // pub mod_ spatial; // TODO Locality / neighborhood
}
crate::mods_out! { // _pub_mods, _reexports, _hidden
    _pub_mods {
        pub use super::{
            graph::_all::*,
            link::_all::*,
            ord::_all::*,
            // spatial::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            graph::{graph_adj, graph_csr},
            link::link,
        };
    }
    _hidden {
        pub use super::graph::_hidden::*;
    }
}
