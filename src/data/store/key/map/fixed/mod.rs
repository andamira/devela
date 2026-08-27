// devela/src/data/store/key/map/fixed/mod.rs
//
//! Fixed-capacity open-addressed hash maps with mutable entries.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // map!
mod impls; // hidden macros for map! variants
mod entry; // MapFixedEntry

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::map,
            entry::MapFixedEntry,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
