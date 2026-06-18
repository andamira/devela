// devela/src/data/access/mod.rs
//
#![doc = crate::_DOC_DATA_ACCESS!()] // public
#![doc = crate::_doc!(modules: crate::data; access: iter, route)]
#![doc = crate::_doc!(flat:"data")]
// #![doc = crate::_QUO_DATA_ACCESS!()]
#![doc = crate::_doc!(hr)]
//!
//! Reachability, traversal, and access structure.
//!
//! - Routes describe ordered access structure.
//! - Addresses interpret reachability through a resolver.
//! - Cursors move explicitly through ordered storage.
//

// mod address; // Addressability and directed reachability WIP
mod cursor; // traversal over ordered data
pub mod iter; // Composable external iteration
pub mod route; // Segmented access routes

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            // address::_all::*,
            cursor::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            iter::_all::*,
            route::_all::*,
        };
    }
}
