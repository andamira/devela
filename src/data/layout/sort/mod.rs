// devela/src/data/layout/sort/mod.rs
//
//! Sorting functionality.
//

// implementations
mod generic;
mod primitives;
#[cfg(feature = "alloc")]
mod impls_alloc;

mod define; // Sort

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::*;
    }
}
