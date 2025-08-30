// devela::ui::front::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod color; // AnsiColor3b, AnsiColor8b
mod namespace; // Ansi
mod r#macro; // ansi!

mod print;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{color::*, namespace::*, r#macro::*};
    }
}
