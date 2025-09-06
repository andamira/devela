// devela_base_core::ui::front::term::ansi
//
//! ANSI escape codes.
//!
//! See <https://en.wikipedia.org/wiki/ANSI_escape_code>.
//

#![expect(non_snake_case, reason = "uppercase const fns as related constants")]

mod color; // AnsiColor3b, AnsiColor8b
mod namespace; // Ansi

crate::structural_mods! { // _mods
    _mods {
        pub use super::{color::*, namespace::*};
    }
}
