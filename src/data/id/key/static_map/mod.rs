// devela::data::id::key::static_map
//
//! Defines the [`define_static_map!`] macro and the [`StaticEntry`] enum.
//

#[cfg(test)]
mod tests;

mod define; // define_static_map!
mod entry; // StaticEntry

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
            entry::*,
        };
    }
}
