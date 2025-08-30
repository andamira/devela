// devela::data::uid
//
//! Abstractions for uniquely identifying data.
//!
//! Includes utilities for managing unique identifiers such as sequential IDs,
//! scoped IDs, and universal unique identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

// WIPZONE
// #[cfg(feature = "std")]
// mod snowflake;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{pin::*, seq::*};
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::snowflake::*;
    }
}
