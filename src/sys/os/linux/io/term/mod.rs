// devela::sys::os::linux::io::term
//
//! Terminal I/O control ABI definitions.
//!
//! Provides Linux terminal control structures and constants used to
//! configure line discipline, input/output modes, and control behavior
//! for terminal-backed file descriptors.
//

mod _raw; // (LINUX_TERMIOS_<I|O|C|L>FLAG)

// mod cc; // LinuxTermiosCc
mod flags; // LinuxTermios<Input|Output|Control|Local>Flags
mod termios; // LinuxTermios

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            // cc::*,
            flags::*,
            termios::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_raw::*;
    }
}
