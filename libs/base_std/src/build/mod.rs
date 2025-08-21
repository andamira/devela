// devela_base::build
//
//! Build-related utilities.
//

mod namespace; // Build
// mod _util;

devela_base::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::namespace::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
