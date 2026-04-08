// devela::data::uid
//
#![doc = crate::_DOC_DATA_ID_UID!()] // public
#![doc = crate::_doc!(modules: crate::data::id; uid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_ID_UID!()]
//!
//! Includes utilities for managing unique identifiers such as sequential IDs,
//! scoped IDs, and universal unique identifiers.
//

mod seq; // id_seq!

#[cfg(feature = "alloc")]
mod pin_box; // IdPinBox

// #[cfg(feature = "std")]
// mod snowflake;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            seq::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::pin_box::*;
        // #[cfg(feature = "std")]
        // pub use super::snowflake::*;
    }
    _reexports {
        pub use devela_base_core::data::id::{ // uid
            IdPin, IdRegistry,
        };
    }
}
