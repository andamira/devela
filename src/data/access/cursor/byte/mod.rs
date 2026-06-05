// devela::data::access::cursor
//
//! Cursor-based access over ordered byte regions.
//

#[cfg(test)]
mod tests;

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
