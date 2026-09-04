// devela/src/sys/mem/view/replica/_.rs
//
//! Defines [`MemReplicaError`], [`MemReplicaSlice`].
//!
//! This module models a replicated logical layout over a single backing slice.
//! Each logical element is written into `N` channel-separated positions so later
//! layers can read from distinct replicas.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod error;
    mod slice;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            error::MemReplicaError,
            slice::MemReplicaSlice,
        };
    }
}
