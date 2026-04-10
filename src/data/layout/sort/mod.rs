// devela::data::layout::sort
//
//! Sorting functionality.
//

// implementations
mod generic;
mod primitives;
#[cfg(feature = "alloc")]
mod impls_alloc;

mod definition; // Sort

crate::structural_mods! { // _mods
    _mods {
        pub use super::definition::*;
    }
}
