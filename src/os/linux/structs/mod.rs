// devela::os::linux::structs
//
//! Linux related structs.
//

mod sigaction;
mod termios;
mod timespec;

#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_os")))]
pub use {
    sigaction::LinuxSigaction,
    termios::{LinuxTerminalSize, LinuxTermios},
    timespec::LinuxTimespec,
};
