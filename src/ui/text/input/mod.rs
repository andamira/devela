// devela/src/ui/text/input/mod.rs
//
//! Interactive text state for input surfaces.
//

#[cfg(test)]
mod _tests;

mod define; // TextInput[Config|Action|Reject|View]

// impls
mod _helper;
mod core;
#[cfg(feature = "alloc")]
mod alloc;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
