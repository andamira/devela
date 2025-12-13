// devela::sys::os::linux::signal
//
//! Signal handling ABI definitions.
//!
//! Provides Linux signal-related structures, constants, and control
//! interfaces used to configure signal delivery, handlers, and masks
//! at the process and thread level.
//

mod sigaction; // LinuxSigaction, LinuxSiginfo, LinuxSigset
mod consts; // LINUX_SIGACTION, LINUX_SIGNAL

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
mod restorer;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            consts::*,
            sigaction::*,
        };
    }
}
