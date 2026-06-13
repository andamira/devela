// devela/src/sys/os/linux/io/term/mod.rs
//
//! Terminal I/O control ABI definitions.
//!
//! Provides Linux terminal control structures and constants used to
//! configure line discipline, input/output modes, and control behavior
//! for terminal-backed file descriptors.
//

mod _raw; // (LINUX_TERMIOS_<I|O|C|L>FLAG, LINUX_TERMIOS_CC)

mod termios; // LinuxTermios, LinuxTermiosCharSize
mod flags; // LinuxTermios<Input|Output|Control|Local>Flags
mod cc; // LinuxTermiosCc

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            termios::*,
            flags::*,
            cc::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_raw::_crate_internals::*;
    }
}
