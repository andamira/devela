// devela::sys::os::linux::structs
//
//! Linux related structs.
//

mod sigaction;
mod termios;
mod timespec;

#[allow(unused)]
pub use {sigaction::*, termios::*, timespec::*};
