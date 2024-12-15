// devela::ui::layout
//
//! Layout functionality.
//
// safety
#![cfg_attr(feature = "safe_layout", forbid(unsafe_code))]

mod error;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::error::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
