// devela::sys::os::term::ansi::definition
//
//! Defines [`Ansi`].
//

#[cfg(test)]
mod tests;

mod definition; // Ansi
mod cursor;
mod font_mouse;
mod osc; // AnsiLink

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
            osc::*,
        };
    }
}
