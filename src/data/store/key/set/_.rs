// devela/src/data/store/key/set/_.rs
//
#![doc = crate::_DOC_DATA_STORE_KEY_SET!()] // public
#![doc = crate::_doc!(modules: crate::data::store::key; set)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//

crate::mods_in! {
    #[cfg(feature = "alloc")]
    mod _reexport_alloc;
    mod _reexport_dep;

    mod sparse; // SparseSet[Array|Error]
    #[cfg(all(target_os = "linux", feature = "_linux_abi", feature = "unsafe_ffi"))]
    mod linux_sparse; // LinuxSparseSet
}
crate::mods_out! { // _mods
    _mods {
        pub use super::sparse::*;
        #[cfg(all(target_os = "linux", feature = "_linux_abi", feature = "unsafe_ffi"))]
        pub use super::linux_sparse::*;
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use super::_reexport_dep::*;
    }
}
