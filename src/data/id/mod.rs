// devela/src/data/id/mod.rs
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id: handle, uuid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//
//! Identifiers distinguish values across storage, position, representation,
//! or execution contexts, with guarantees that depend on the identifier form.
//! Different forms provide different scopes and resolution models:
//!
//! - [`Handles`](mod@handle) refer to stored values through a particular context.
//! - [`UUIDs`](uuid) provide standardized, portable 128-bit identifiers
//!   without requiring a shared local allocator.
//! - Locally generated or anchored identifiers distinguish values within
//!   a bounded execution scope.
//

pub mod handle; // Compact contextual references resolved against external state
pub mod uuid; // Standardized portable 128-bit identifiers

mod pin; // IdPin
mod registry; // IdRegistry
mod seq; // id_seq!

#[cfg(feature = "alloc")]
mod pin_box; // IdPinBox
// #[cfg(feature = "std")]
// mod snowflake;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[doc(inline)]
        pub use super::{
            pin::IdPin,
            registry::IdRegistry,
            seq::id_seq,
        };
        #[cfg(feature = "alloc")]
        pub use super::pin_box::IdPinBox;
        // #[cfg(feature = "std")]
        // pub use super::snowflake::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            handle::_all::*,
            uuid::_all::*,
        };
    }
}
