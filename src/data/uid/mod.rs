// devela::data::uid
//
#![doc = crate::_DOC_DATA_UID!()]
//!
#![doc = crate::_QUOTE_DATA_UID!()]
//!
//! Includes utilities for managing unique identifiers such as sequential IDs,
//! scoped IDs, and universal unique identifiers.
//

mod seq; // id_seq!

// #[cfg(feature = "std")]
// mod snowflake;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            seq::*,
        };
        // #[cfg(feature = "std")]
        // pub use super::snowflake::*;
    }
    _reexports {
        pub use devela_base_core::data::uid::{IdPin, IdRegistry};
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::data::uid::IdPinBox;
    }
}
