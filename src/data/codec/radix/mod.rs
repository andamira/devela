// devela/src/data/codec/radix/mod.rs
//
//! Radix-based encodings.
//

#[cfg(test)]
mod _test;

mod base; // Base*
// mod new_base; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            // new_base::*,
        };
    }
}
