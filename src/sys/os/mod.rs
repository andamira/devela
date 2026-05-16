// devela::sys::os
//
#![doc = crate::_DOC_SYS_OS!()] // public
#![doc = crate::_doc!(modules: crate::sys; os: browser, fd, term, linux)] // windows
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//!
//! Kernel-backed or virtualized environments that define the core capability
//! surfaces available to applications. This includes native OS layers
//! (Linux, macOS, Windows), compatibility layers (libc), and sandboxed
//! host environments such as browser runtimes.
//
// OSes: (https://doc.rust-lang.org/beta/rustc/platform-support.html)
// - https://motor-os.org/ | https://github.com/moturus/motor-os
// - https://hermit-os.org/ | https://github.com/hermit-os/hermit-rs
// - https://source.android.com/docs/security/features/trusty
// - https://wasi.dev/ | https://github.com/WebAssembly/WASI

pub mod browser; // Web*
#[cfg(feature = "unsafe_ffi")]
mod c; // Libc
pub mod fd;

#[cfg(feature = "_linux_abi")]
#[crate::macro_apply(crate::__doc_show(feature = "linux"))]
crate::items! {
    pub mod linux;
}

// #[cfg(feature = "macos")]
// pub mod macos;
#[cfg(feature = "term")]
pub mod term; // Ansi* Term*
// #[cfg(feature = "windows")]
// pub mod windows;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[cfg(feature = "unsafe_ffi")]
        pub use super::c::*;
    }
    _pub_mods {
        pub use super::{
            browser::_all::*,
            fd::_all::*,
        };
        #[cfg(feature = "_linux_abi")]
        pub use super::linux::_all::*;
        #[cfg(feature = "term")]
        pub use super::term::_all::*;
        // #[cfg(feature = "macos")]
        // pub use super::macos::_all::*;
        // #[cfg(feature = "windows")]
        // pub use super::windows::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "term")]
        pub use super::term::_crate_internals::*;
    }
}
