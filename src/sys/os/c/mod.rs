// devela::sys::os::c
//
//! Libc
//

mod raw;

mod namespace; // Libc

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
