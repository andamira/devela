// devela/src/sys/os/c/mod.rs
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
