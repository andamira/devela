// devela/src/sys/mem/view/slice/namespace/mod.rs
//
//! Defines the [`Slice`] namespace.
//

#[cfg(test)]
mod _test;

// impls
mod core;
mod range;
mod take;
mod split;
mod chunk;
mod bytes;
mod eq;

mod define; // Slice
mod r#macro; // slice!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::Slice,
            r#macro::slice,
        };
    }
}
