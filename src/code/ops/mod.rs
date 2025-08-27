// devela::code::ops
//
//! Overloadable operators.
//!
#![doc = crate::_doc!(extends: ops)]
//

mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::reexports::*;
        // WIPZONE
        // pub use super::closure::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIPZONE
// mod closure;
