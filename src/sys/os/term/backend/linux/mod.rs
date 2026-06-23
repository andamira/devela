// devela/src/sys/os/term/backend/linux/mod.rs
//
//! Linux terminal backend.
//

mod define; // TermLinux
mod restore; // TermLinuxRestore
#[cfg(feature = "event")]
mod buf; // (TermLinuxInputBuf)

// impls
mod impl_trait; // impl TermBackend
mod cap; // impl capabilities
#[cfg(feature = "event")]
mod poll; // impl polling

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::TermLinux,
            restore::TermLinuxRestore,
        };
    }
    _crate_internals {
        #[cfg(feature = "event")]
        pub(crate) use super::{
            buf::TermLinuxInputBuf,
        };
    }
}
