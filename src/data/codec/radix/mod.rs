// devela/src/data/codec/radix/mod.rs
//
//! Radix-based encodings.
//

#[cfg(test)]
mod _test;

mod define; // Radix
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::Radix,
        };
    }
}
