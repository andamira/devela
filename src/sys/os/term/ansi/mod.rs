// devela::sys::os::term::ansi
//
//! ANSI escape codes.
//!
//! See <https://en.wikipedia.org/wiki/ANSI_escape_code>.
//

mod print; // ansi_print, ansi_print_linux, ansi_print_std
mod r#macro; // ansi!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            print::*,
            r#macro::*,
        };
    }
}
