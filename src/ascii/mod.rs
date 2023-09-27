// devela::ascii
//
//! ASCII strings and characters, extends [`core::ascii`].
//!
//! It also reexports some [`const-str`](https://docs.rs/const-str) macros
//! directly related to ASCII, prefixed with `ascii_`, and a new description.
//

mod char;
mod fns;

pub use {char::AsciiChar, fns::*};

mod reexport_const_str;
pub use reexport_const_str::*;
