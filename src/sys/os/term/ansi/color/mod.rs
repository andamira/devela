// devela::sys::os::term::ansi::color
//
//! ANSI codes related to color.
//

mod bit3; // AnsiColor3
mod bit8; // AnsiCOlor8
mod color; // AnsiColor

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            color::*,
            bit3::*,
            bit8::*,
        };
    }
}
