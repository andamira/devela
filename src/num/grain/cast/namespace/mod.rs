// devela/src/num/grain/cast/namespace/mod.rs
//
//! Helpers for converting between primitives.
//

mod define; // Cast

// impls
mod cast;
mod join;
mod split;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
