// devela::sys::os::term::ansi::definition
//
//! Defines [`Ansi`].
//

#[cfg(test)]
mod tests;

mod definition; // Ansi

// impls
mod cursor;
mod font_mouse;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
        };
    }
}
