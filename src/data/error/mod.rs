// devela::data::error
//
//! Data-related error types.
//

mod definitions;

crate::items! { // structural access: _mods, _all,
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definitions::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
