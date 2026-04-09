// devela::data::layout::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
mod primitives;

mod definition; // Sort

#[cfg(feature = "alloc")]
crate::items! {
    mod definition_alloc; // SortAlloc
    mod impls_alloc;
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::definition::*;

        #[cfg(feature = "alloc")]
        pub use super::definition_alloc::*;
    }
}
