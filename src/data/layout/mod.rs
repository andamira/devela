// devela/src/data/layout/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT!()] // public
#![doc = crate::_doc!(modules: crate::data; layout: array, buffer, dst, linked)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, vec)]
//!
//! Layout determines how values are positioned and grouped.
//!
//! It describes where elements reside, how positions correspond,
//! and which portion of a capacity is logically occupied.
//!
//! It does not provide durable identity or govern reclamation;
//! those concerns belong to [`data::id`](crate::data::id) and
//! [`data::store`](crate::data::store).
//!
//! - [`Arrays`](mod@array) add dimensions and coordinate mappings.
//! - [`Buffers`](buffer) add bounded occupancy in linear or cyclic order.
//! - [`DST storage`](dst) supports dynamically sized representations
//!   without requiring heap allocation.
//

pub mod array; // Contiguous homogeneous storage with dimensional projections
pub mod buffer; // Capacity-managed storage with explicit occupancy state
mod collection; // DataCollection
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst; // Dynamically-sized types stored without need of heap allocation
// pub mod erased; // TODO
pub mod linked; // Homogeneous, sequentially accessed structures
mod ord; // Sort
mod queue; // Homogeneous data structures that process elements in FIFO order
mod stack; // Homogeneous data structures that process elements in LIFO order
// pub mod table; // TODO Tabular and heterogeneous data processing

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            collection::DataCollection,
            queue::_all::*,
            ord::_all::Sort,
            stack::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            array::_all::*,
            buffer::_all::*,
            // erased::_all::*,
            linked::_all::*,
            // table::_all::*,
        };
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub use super::dst::_all::*;
    }
    _crate_internals {
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        pub(crate) use super::dst::_crate_internals::*;
    }
}
