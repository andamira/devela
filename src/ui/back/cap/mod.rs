// devela::ui::service::cap
//
//! Ui service capabilities.
//

mod definitions;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::definitions::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
