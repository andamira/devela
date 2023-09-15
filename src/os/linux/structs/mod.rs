// devela::os::linux::structs
//
//! Linux related structs.
//

mod termios;
mod timespec;

pub use {termios::SysTermios, timespec::SysTimespec};
