// devela_base_core::sys::os::term::ansi
//
//! ANSI escape codes.
//!
//! See <https://en.wikipedia.org/wiki/ANSI_escape_code>.
//

#![expect(non_snake_case, reason = "uppercase const fns as related constants")]

mod helper; // _ANSI_CONSTS

mod strip; // impl: Ansi:strip_codes

mod namespace; // Ansi
mod color; // AnsiColor, AnsiColor3, AnsiColor8 (+Ansi impls)

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            color::*,
            namespace::*,
        };
    }
    _crate_internals {
        pub(crate) use super::helper::*;
    }
}
