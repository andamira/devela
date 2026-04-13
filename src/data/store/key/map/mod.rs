// devela::data::store::key::map
//
//! Defines the [`map!`] macro and the [`StaticEntry`] enum.
//

#[cfg(test)]
mod tests;

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
