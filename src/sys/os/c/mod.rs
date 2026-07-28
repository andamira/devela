// devela/src/sys/os/c/mod.rs
//
//! Libc
//

#[cfg(unix)]
mod _raw;

mod namespace; // Libc

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
