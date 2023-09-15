// devela::os::linux::structs
//
//! Linux related structs.
//

mod sigaction;
mod termios;
mod timespec;

pub use {sigaction::SysSigaction, termios::SysTermios, timespec::SysTimespec};
