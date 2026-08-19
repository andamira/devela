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
//! - [`UUIDs`](mod@uuid) provide standardized, portable 128-bit identifiers
//!   without requiring a shared local allocator.
//!   - [`Uuid`] represents a standardized portable 128-bit identity.
//!   - [`UuidV7Generator`] generates strictly ordered version 7 UUID sequences.
//! - Locally generated or anchored identifiers distinguish values within
//!   a bounded execution scope.
//

#[cfg(any(test, feature = "_docs_examples"))]
mod _example;

pub mod handle; // Compact contextual references resolved against external state
pub mod uuid; // Standardized portable 128-bit identifiers

mod pin; // IdPin
mod registry; // IdRegistry
mod seq; // id_seq!

#[cfg(feature = "alloc")]
mod pin_box; // IdPinBox
// #[cfg(feature = "std")]
// mod snowflake;

crate::structural_mods! { // _mods, _pub_mods, _reexports
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

        #[cfg(any(test, feature = "_docs_examples"))]
        pub use super::_example::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            handle::_all::*,
            uuid::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            handle::{handle, handle_gen},
            uuid::{Uuid, UuidV7Generator}
        };
    }
}
