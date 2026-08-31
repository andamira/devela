// devela/src/data/id/_.rs
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id: handle, local, uuid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//
//! Identifiers distinguish values across storage, position, representation,
//! or execution contexts, with guarantees that depend on the identifier form.
//! Different forms provide different scopes and resolution models:
//!
//! - [`Handles`](mod@handle) refer to stored values through a particular
//!   resolving context.
//! - [`Local identities`](mod@local) distinguish values within bounded
//!   execution, allocation, registry, or type contexts.
//! - [`UUIDs`](mod@uuid) provide standardized portable 128-bit identifiers
//!   without requiring a shared local allocator.
//

crate::mods_in! {
    pub mod_ handle; // Compact contextual references interpreted and validated by a resolver
    pub mod_ local; // Locally scoped identities and allocation mechanisms
    // mod snowflake; // FUTURE Time-ordered identifiers for distributed generation
    pub mod_ uuid; // Standardized portable 128-bit identifiers
}
crate::mods_out! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        // pub use super::snowflake::*;
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            handle::_all::*,
            local::_all::*,
            uuid::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            handle::{handle, handle_gen},
            local::id_seq,
            uuid::{Uuid, UuidV7Generator}
        };
    }
    _hidden {
        pub use super::handle::_hidden::*;
    }
}
