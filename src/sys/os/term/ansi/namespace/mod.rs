// devela/src/sys/os/term/ansi/namespace/mod.rs
//
//! Defines [`Ansi`].
//

#[cfg(test)]
mod _test;

mod define; // Ansi, control prefixes, erase escape codes

mod terminal; // terminal modes
mod cursor; // cursor escape codes
mod mouse; // mouse escape codes
mod font; // font effect escape codes
mod color; // color escape codes
mod osc; // AnsiLink, OSC codes

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::*,
            osc::*,
        };
    }
}
