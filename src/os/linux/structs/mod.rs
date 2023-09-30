// devela::os::linux::structs
//
//! Linux related structs.
//

// most linux struct fields and methods are only used with `unsafe_linux` enabled
#![cfg_attr(not(feature = "unsafe_linux"), allow(unused))]

mod sigaction;
mod termios;
mod timespec;

#[cfg_attr(feature = "nightly", doc(cfg(feature = "linux")))]
pub use {
    sigaction::{LinuxSigaction, LinuxSigset},
    termios::{LinuxTerminalSize, LinuxTermios},
    timespec::LinuxTimespec,
};
