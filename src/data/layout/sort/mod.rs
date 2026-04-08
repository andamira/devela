// devela::data::layout::sort
//
//! Sorting functionality.
//

#[cfg(feature = "alloc")]
crate::items! {
    mod definition_alloc; // SortAlloc
    mod impls_alloc;
}

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::definition_alloc::*;
    }
}
