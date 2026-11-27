//
#![doc = crate::_DOC_DATA_ACCESS_SEARCH!()] // public
#![doc = crate::_doc!(modules: crate::data::access; search)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Search explores connectivity supplied by the caller, without requiring
//! a materialized graph or owning its storage.
//

crate::mods_in! {
    mod bfs; // Search, SearchNode
    #[cfg(test)]
    mod _test;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::bfs::{Search, SearchNode};
    }
}
