// devela::sys::os::term::ansi::definition
//
//! Defines [`Ansi`].
//

#[cfg(test)]
mod tests;

mod definition; // Ansi, control prefixes, erase escape codes
mod cursor; // cursor escape codes
mod terminal; // terminal modes
mod font_mouse; // mouse and font effect escape codes
mod osc; // AnsiLink

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            definition::*,
            osc::*,
        };
    }
}
