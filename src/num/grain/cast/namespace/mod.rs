// devela::num::grain::cast::namespace
//
//! Helpers for converting between primitives.
//

mod definition; // Cast

// impls
mod cast;
mod join;
mod split;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
        };
    }
}
