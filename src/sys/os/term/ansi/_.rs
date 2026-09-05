// devela/src/sys/os/term/ansi/_.rs
//
//! ANSI escape codes.
//!
//! See <https://en.wikipedia.org/wiki/ANSI_escape_code>.
//

#![expect(non_snake_case, reason = "uppercase const fns as related constants")]

crate::mods_in! {
    mod _helper; // __ansi_consts!

    mod_ namespace; // Ansi
    mod color; // AnsiColor, AnsiColor3, AnsiColor8
    mod strip; // impl: Ansi:strip_codes

    mod print; // ansi_print, ansi_print_linux, ansi_print_std
    mod_ r#macro; // ansi!
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            namespace::_all::{Ansi, AnsiLink, AnsiOsc},
            color::*,
            print::*,
            r#macro::_all::ansi,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
