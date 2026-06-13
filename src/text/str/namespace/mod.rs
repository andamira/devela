// devela/src/text/str/namespace/mod.rs
//
//! Defines the [`Str`] namespace.
//

mod definition; // Str

// impls
mod range;
mod take;
mod split;

crate::structural_mods! { // _mods
    _mods {
        pub use super::definition::*;
    }
}
