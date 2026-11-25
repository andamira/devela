//
#![doc = crate::_DOC_DATA_STORE_KEY_SET!()] // public
#![doc = crate::_doc!(modules: crate::data::store::key; set)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Sets represent membership of distinct values
//! without associating a separate payload with each member.
//!
//! Implementations may trade memory for lookup,
//! insertion, removal, or clearing costs.
//

crate::mods_in! {
    mod alias;
    mod sparse; // SparseSet[Array|Error]
    #[cfg(all(target_os = "linux", feature = "_linux_abi", feature = "unsafe_ffi"))]
    mod linux_sparse; // LinuxSparseSet
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            alias::*,
            sparse::*,
        };
        #[cfg(all(target_os = "linux", feature = "_linux_abi", feature = "unsafe_ffi"))]
        pub use super::linux_sparse::*;
    }
    _reexports {
        pub use crate::boot::rust::data::store_key_set::*;
    }
}
