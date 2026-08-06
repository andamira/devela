// devela/src/data/store/pool/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_POOL!()] // public
#![doc = crate::_doc!(modules: crate::data::store; pool)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Pools retain values in independently reclaimable slots.
//!
//! Each insertion returns a handle containing a slot index and generation.
//! Removing a value advances that slot's generation before it can be reused,
//! allowing previously issued handles to be rejected.
//!
//! Unlike an [`arena`](super::arena), a pool can reclaim and reuse one slot
//! without reclaiming values inserted after it. Live values remain in stable
//! slots while vacant slots are recycled.
//!
//! - [`pool!`] generates either fixed-capacity static pools
//!   or growable allocating pools.
//! - [`PoolIter`] traverses the currently occupied slots.
//!
//! Handles are relative to the pool instance that issued them and generations
//! eventually wrap. Their stale-handle protection is therefore bounded by the
//! store context and configured generation domain.
//

#[cfg(test)]
mod _test;
#[cfg(all(test, feature = "alloc"))]
mod _model;
#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

mod define; // pool!
mod impls; // hidden macros for pool variants
mod iter; // PoolIter

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::pool,
            iter::PoolIter,
        };
        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
}
