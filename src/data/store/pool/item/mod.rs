// devela/src/data/store/pool/item/mod.rs
//
//! Single-item generational pools.
//!
//! Each retained item occupies one indexed slot whose index remains fixed until
//! removal, and is addressed by a handle containing that index and generation.
//!
//! Removing an item advances the slot generation before making the slot
//! available for reuse, so previously issued handles no longer resolve.
//! Vacant slots may then be recycled independently of other retained items.
//!
//! [`pool!`] generates fixed-capacity static or growable allocating item pools.
//! [`PoolIter`] traverses the values in their currently occupied slots.
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
