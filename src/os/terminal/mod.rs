// devela::os::terminal
//
//! Definitions for terminal control and manipulation.
//!
//! Reexports terminal related functions from [`os::linux`][super::linux].
//

mod ansi;
mod color;

pub use {ansi::Ansi, color::AnsiColor};

/* reexports */

#[cfg(all(
    any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "riscv32",
        target_arch = "riscv64"
    ),
    feature = "unsafe_os",
    not(miri),
))]
#[doc(inline)]
pub use super::linux::{
    disable_raw_mode, enable_raw_mode, get_byte, is_terminal, print, print_bytes,
};
