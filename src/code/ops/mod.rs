// devela::code::ops
//
//! Overloadable operators.
//!
#![doc = crate::_doc!(extends: ops)]
//

crate::mod_path!(_c "../../../libs/base_core/src/code/ops/reexports.rs");

// WIPZONE
// #[cfg(feature = "std")]
// pub mod _wip_fns;
// mod _wip_closure; // TODO

crate::structural_mods! { // _mods
    _mods {
        pub use super::_c::*;
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::_wip_fns::*;
        // pub use super::_wip_closure::*;
    }
}
