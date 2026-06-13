// devela/src/data/store/key/set/mod.rs
//
//! Keyed set structures.
//

mod sparse; // SparseSet[Array|Error]
#[cfg(all(feature = "_linux_abi", feature = "unsafe_ffi"))]
mod linux_sparse; // LinuxSparseSet

crate::structural_mods! { // _mods
    _mods {
        pub use super::sparse::*;
        #[cfg(all(feature = "_linux_abi", feature = "unsafe_ffi"))]
        pub use super::linux_sparse::*;
    }
}
