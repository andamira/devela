// devela/src/num/fin/bit/wise/mod.rs
//
//! Defines the [`Bitwise`] namespace.
//

#[cfg(test)]
mod _test;

mod define; // Bitwise
mod impls;

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
