// devela::data::uid
//
//! Abstractions for uniquely identifying data.
//!
//! Includes utilities for managing unique identifiers such as sequential IDs,
//! scoped IDs, and universal unique identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

crate::items! { // structural access: _mods, _all
    #[allow(unused_imports)]
    pub use _mods::*;

    mod _mods {
        pub use super::{pin::*, seq::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
