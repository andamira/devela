// devela::lang::js
//
//! Javascript interfacing.
//

mod reexport; // js_reexport!

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexport::*;
        // WIPZONE
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
