// devela/src/data/store/key/map/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_KEY_MAP!()] // public
#![doc = crate::_doc!(modules: crate::data::store::key; map)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//

#[cfg(feature = "alloc")]
mod _reexport_alloc;
mod _reexport_dep;

mod fixed; // Fixed-capacity open-addressed hash maps with mutable entries
// mod perfect; // Immutable perfect-hash maps over statically known key sets

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            fixed::_all::*,
            // perfect::_all::*,
        };
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use super::_reexport_dep::*;
    }
}
