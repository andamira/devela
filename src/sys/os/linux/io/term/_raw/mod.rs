// devela/src/sys/os/linux/io/term/_raw/mod.rs

#![allow(
    dead_code,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::zero_prefixed_literal
)]

mod input; // LINUX_TERMIOS_IFLAG
mod output; // LINUX_TERMIOS_OFLAG
mod control; // LINUX_TERMIOS_CFLAG
mod local; // LINUX_TERMIOS_LFLAG
mod cc; // LINUX_TERMIOS_CC

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            input::*,
            output::*,
            control::*,
            local::*,
            cc::*,
        };
    }
}
