// devela::sys::os::linux::term
//
//! Terminal I/O control ABI definitions.
//!
//! Provides Linux terminal control structures and constants used to
//! configure line discipline, input/output modes, and control behavior
//! for terminal-backed file descriptors.
//

mod consts; // LINUX_TERMIOS_[I|O|C|L]FLAG
mod termios; // LinuxTermios

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            consts::*,
            termios::*,
        };
    }
}
