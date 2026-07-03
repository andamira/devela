// devela/src/num/fin/bit/wise/mod.rs
//
//! Defines the [`Bitwise`] namespace.
//

#[cfg(test)]
mod _test;

mod define; // Bitwise
mod impls;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
