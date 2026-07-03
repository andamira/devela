// devela/src/data/layout/stack/stack/mod.rs
//
//! A type that can be used as a single-ended stack.
//

// impls
mod impl_traits;
mod methods;

mod define; // Stack, StackIter, …

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::*;
    }
}
