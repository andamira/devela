// devela/src/data/store/key/map/mod.rs
//
//! Defines the [`map!`] macro and the [`StaticEntry`] enum.
//

#[cfg(test)]
mod _test;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // map!
mod impls; // hidden macros for map! variants
mod entry; // StaticMapEntry

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::map,
            entry::StaticMapEntry,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
