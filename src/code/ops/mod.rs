// devela::code::ops
//
//! Overloadable operators.
//!
#![doc = crate::_doc!(extends: ops)]
//

crate::mod_path!(_c "../../../libs/base/src/code/ops/reexports.rs");

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::_c::*;
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
