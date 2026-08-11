// devela/src/data/store/arena/typed/mod.rs
//
//! Typed monotonic arenas.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // arena!
mod impls; // hidden macros for arena variants

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::arena;
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
