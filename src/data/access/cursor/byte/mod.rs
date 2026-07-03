// devela/src/data/access/cursor/byte/mod.rs
//
//! Cursor-based access over ordered byte regions.
//

#[cfg(test)]
mod _test;

mod define; // ByteCursor (+ impl common)
mod read;
mod write;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
