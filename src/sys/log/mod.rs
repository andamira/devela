// devela::sys::log
//
//! Logging functionality.
//

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        // #[allow(unused)]
    }
    pub(super) mod _all {
        // #[doc(inline)] pub use super::_mods::*;
    }
}
