// devela_base::build
//
//! Build-related utilities.
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_build"), forbid(unsafe_code))]

mod namespace; // Build
// mod _util;

devela_base::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
