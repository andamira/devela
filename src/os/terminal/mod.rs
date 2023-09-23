// devela::os::terminal
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
    ansi_codes::Ansi,
    macros::{ansi, ansib},
};
