// devela::sys::os::term::ansi
//
//! ANSI escape codes.
//!
//! See <https://en.wikipedia.org/wiki/ANSI_escape_code>.
//

#![expect(non_snake_case, reason = "uppercase const fns as related constants")]

mod _helper; // __ansi_consts!

mod strip; // impl: Ansi:strip_codes

mod color; // AnsiColor, AnsiColor3, AnsiColor8 (+Ansi impls)
mod namespace; // Ansi
mod print; // ansi_print, ansi_print_linux, ansi_print_std
mod r#macro; // ansi!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            color::*,
            namespace::*,
            print::*,
            r#macro::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
