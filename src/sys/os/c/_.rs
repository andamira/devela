// devela/src/sys/os/c/_.rs
//
//! Libc
//

crate::mods_in! {
    #[cfg(unix)]
    mod _raw;

    mod namespace; // Libc
}
crate::mods_out! { // _mods
    _mods {
        pub use super::namespace::Libc;
    }
}
