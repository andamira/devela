// devela::os::term
//
//! Definitions for terminal control and manipulation.
//!
//! Reexports terminal related functions from [`os::linux`][super::linux].
//!
//! The provided functionaliy is optimized for an `no_std` environment.
//!
//

mod ansi_codes;
mod macros;
pub use {
    // ansi_codes::Ansi,
    ansi_codes::{Ansi, AnsiColor3, AnsiColor8},
    macros::{ansi, ansib},
};

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
pub use macros::ansip;
