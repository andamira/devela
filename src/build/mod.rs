// devela::build
//
//! Build-related utilities.
//
// safety
#![cfg_attr(feature = "safe_build", forbid(unsafe_code))]

mod namespace; // Build
// mod _util;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
