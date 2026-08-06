// devela/src/data/store/arena/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_ARENA!()] // public
#![doc = crate::_doc!(modules: crate::data::store; arena)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Arenas retain values by advancing through storage
//! rather than recycling arbitrary interior holes.
//!
//! Stored regions remain at stable positions while they are retained.
//! Reclamation proceeds from a boundary: by rolling back to a mark,
//! truncating the most recently stored region, or discarding the arena.
//!
//! The current surface is byte-oriented:
//!
//! - [`arena_bytes!`] generates fixed-capacity byte arenas
//!   with span handles and rollback marks.
//!
//! Arena span handles describe an offset and length; they are coordinates,
//! not generational identities. Rolling back or truncating a region invalidates
//! its handles, and later writes may reuse the same offsets.
//

mod byte; // arena!

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            byte::_all::*,
        };
    }
    _hidden {
        pub use super::{
            byte::_hidden::*,
        };
    }
}
