// devela/src/data/layout/ord/_.rs
//
//! Sorting functionality.
//

crate::mods_in! {
    // implementations
    mod generic;
    mod primitives;
    #[cfg(feature = "alloc")]
    mod impls_alloc;

    mod define; // Sort
}
crate::mods_out! { // _mods
    _mods {
        pub use super::define::*;
    }
}
