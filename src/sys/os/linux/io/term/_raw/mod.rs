// devela::sys::os::linux::io::term::_raw
//
//! Terminal I/O control ABI definitions.
//!
//! Provides Linux terminal control structures and constants used to
//! configure line discipline, input/output modes, and control behavior
//! for terminal-backed file descriptors.
//

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
