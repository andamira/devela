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

pub use {ansi_codes::*, macros::*};

pub(super) mod all {
    pub use super::{ansi_codes::*, macros::*};
}
