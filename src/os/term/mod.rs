// devela::os::term
//
//! Terminal control and manipulation definitions.
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

#[cfg(any(
    all(feature = "std", not(miri)),
    all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "linux",
        feature = "unsafe_linux",
        not(miri),
    )
))]
pub use macros::ansip;
