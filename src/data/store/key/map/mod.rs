// devela/src/data/store/key/map/mod.rs
//
//! Defines the [`map!`] macro and the [`StaticEntry`] enum.
//

#[cfg(test)]
mod _test;

mod define; // map!
mod entry; // StaticEntry

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
            entry::*,
        };
    }
}
