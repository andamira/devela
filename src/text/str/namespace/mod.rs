// devela/src/text/str/namespace/mod.rs
//
//! Defines the [`Str`] namespace.
//

#[cfg(test)]
mod _test;

mod define; // Str

// impls
mod utf8_traversal; // UTF-8 conversion and traversal
mod boundary; // equality and boundary
mod writing; // writing, transliteration and repetition
mod range;
mod take;
mod split;

crate::structural_mods! { // _mods
    _mods {
        pub use super::define::Str;
    }
}
