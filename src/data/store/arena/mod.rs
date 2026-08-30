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
//! Stored values remain at stable positions while they are retained.
//!
//! Reclamation proceeds collectively from the current frontier:
//! by clearing an arena, rolling back where marks are enabled,
//! truncating byte regions, or discarding the arena.
//!
//! Three arena forms are provided:
//!
//! - [`arena!`] generates typed arenas over either fixed-capacity static storage
//!   or growable allocating storage, with compact index handles and optional rollback marks.
//! - [`arena_bytes!`] generates byte arenas over either fixed-capacity static storage
//!   or growable allocating storage, with span handles and optional rollback marks.
//! - [`arena_string!`] generates packed UTF-8 string arenas over either
//!   fixed-capacity static storage or growable allocating storage,
//!   with compact index handles and optional rollback marks.
//!
//! Typed- and string-arena handles contain only an index.
//! Byte-arena handles describe an offset and length.
//! Both are storage coordinates rather than generational identities:
//! rollback, truncation, or clearing invalidates reclaimed handles, and later
//! insertion may reuse the same coordinates so an old handle can resolve again.
//

mod bytes; // arena_bytes!
mod string; // arena_string!
mod typed; // arena!

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            bytes::_all::*,
            string::_all::*,
            typed::_all::*,
        };
    }
    _hidden {
        pub use super::{
            bytes::_hidden::*,
        };
    }
}
