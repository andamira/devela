// devela/src/data/store/pool/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_POOL!()] // public
#![doc = crate::_doc!(modules: crate::data::store; pool)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Pools retain independently reclaimable values through generational handles.
//!
//! Removing a retained value advances the generation associated
//! with its identity and releases its storage for reuse,
//! so previously issued handles no longer resolve.
//!
//! Unlike an [`arena`](mod@super::arena), a pool can reclaim one retained value
//! without reclaiming values introduced after it.
//!
//! Two pool forms are provided:
//!
//! - [`pool!`] stores each value in an independently reusable slot
//!   whose index remains fixed while the item is retained.
//! - [`pool_seq!`] stores variable-length contiguous sequences whose identities remain
//!   stable while their physical cell spans may be reclaimed or relocated.
//!
//! Handles are relative to the pool instance that issued them and generations
//! eventually wrap. Stale-handle protection is therefore bounded by the store
//! context and configured generation domain.
//

mod item; // pool!
mod seq; // pool_seq!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            item::_all::*,
            seq::_all::*,
        };
    }
}
