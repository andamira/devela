// devela::sys::mem::view::replica
//
//! Defines [`MemReplicaError`], [`MemReplicaSlice`].
//!
//! This module models a replicated logical layout over a single backing slice.
//! Each logical element is written into `N` channel-separated positions so later
//! layers can read from distinct replicas.
//

#[cfg(test)]
mod tests;

mod error; // MemReplicaError
mod slice; // MemReplicaSlice

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            error::*,
            slice::*,
        };
    }
}
