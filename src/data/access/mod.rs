// devela/src/data/access/mod.rs
//
#![doc = crate::_DOC_DATA_ACCESS!()] // public
#![doc = crate::_doc!(modules: crate::data; access: iter, route)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! These abstractions operate over data supplied by another context.
//! They may change what is current or reachable without relocating or
//! taking ownership of the underlying values.
//!
//! Access does not determine identity, physical arrangement, or retention.
//! Those concerns belong to [`data::id`](crate::data::id),
//! [`data::layout`](crate::data::layout), and
//! [`data::store`](crate::data::store).
//!
//! - Addresses interpret reachability through a resolver.
// - [`Addresses`](address) express symbolic or contextual references
//   for later resolution.
// - Cursors maintain an explicit position within ordered data.
//! - [`ByteCursor`] maintains an explicit position within ordered byte data.
//! - [`Iterators`](iter) expose successive elements through traversal protocols.
//! - [`Routes`](route) represent ordered segments before domain-specific
//!   interpretation.
//

// mod address; // Symbolic and contextual references interpreted through resolution
mod cursor; // Retained positional access over ordered data
pub mod iter; // Composable external and lending traversal
mod offset; // Explicit positional access: read_at!, write_at!
pub mod route; // Segmented routes before domain-specific interpretation

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            // address::_all::*,
            cursor::_all::*,
            offset::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            iter::_all::*,
            route::_all::*,
        };
    }
}
