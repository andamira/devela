// devela/src/data/id/_.rs
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id: handle, local, uuid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Identity distinguishes one value from another
//! independently of its current representation or physical position.
//!
//! Different identity forms make different assumptions about scope:
//! - Handles are meaningful through a resolver.
//! - Local identities are bounded by a local execution or allocation context.
//! - UUIDs carry identity across contexts without a shared allocator.
//!
//! Choosing an identifier means choosing both how distinction is represented
//! and where that distinction remains meaningful.
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
