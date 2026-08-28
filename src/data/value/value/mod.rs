// devela/src/data/value/kind/value.rs
//

#[cfg(test)]
mod _test;

mod define; // Value*

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
