// devela/src/data/value/value/mod.rs
//
//! Defines Value<8|16|32|64|128>.
//

#[cfg(test)]
mod _test;

mod define; // Value*
mod regrade;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::{Value8, Value16, Value32, Value64, Value128},
        };
    }
}
