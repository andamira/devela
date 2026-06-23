// devela/src/sys/os/term/backend/linux/mod.rs
//
//! Linux terminal backend.
//

mod define; // TermLinux
mod poll; // impl polling
mod cap; // impl capabilities
mod impl_trait; // impl TermBackend

mod buf; // (TermLinuxInputBuf)
mod restore; // TermLinuxRestore

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::TermLinux,
            restore::TermLinuxRestore,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            buf::TermLinuxInputBuf,
        };
    }
}
