// devela/src/sys/os/c/mod.rs
//
//! Libc
//

#[cfg(unix)]
mod _raw;

mod namespace; // Libc

crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
