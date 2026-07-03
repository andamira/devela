// devela/src/data/layout/queue/destaque/mod.rs
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

#[cfg(all(test, feature = "_destaque_u8"))]
mod _test;

mod impl_traits;
mod methods;

mod define; // Destaque, DestaqueIter, …

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::*;
    }
}
