// devela/src/data/codec/radix/mod.rs
//
//! Radix-based encodings.
//

#[cfg(test)]
mod _test_base;
#[cfg(test)]
mod _test;

mod base; // Base*
mod define; // Radix

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            define::*,
        };
    }
}
