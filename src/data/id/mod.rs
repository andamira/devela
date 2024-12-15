// devela::data::id
//
//! Data identifiers.
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
