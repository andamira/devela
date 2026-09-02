// devela/src/build/mod.rs
//
//! Build-related utilities.
//
// safety
#![cfg_attr(feature = "safe_build", forbid(unsafe_code))]

crate::mods_in! {
    mod namespace; // Build
    // mod _util;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
