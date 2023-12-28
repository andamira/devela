// devela::os::term::ansi
//
//! ANSI codes.
//

#![allow(non_snake_case)]

mod codes;
mod color;

pub use {
    codes::Ansi,
    color::{AnsiColor3b, AnsiColor8b},
};
