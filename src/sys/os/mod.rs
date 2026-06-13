// devela/src/sys/os/mod.rs
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

// #[cfg(feature = "android")]
// pub mod android;

#[doc = crate::_tags!(web)]
pub mod browser; // Web*
#[cfg(all(feature = "unsafe_ffi", not(feature = "safe_sys")))]
mod c; // Libc
pub mod fd;

#[cfg(feature = "_linux_abi")]
crate::__doc_hide! { (feature = "_linux_abi")
crate::__doc_show! { (feature = "linux")
#[doc = crate::_tags!(linux)]
pub mod linux; }}

// #[doc = crate::_tags!(apple)]
// #[cfg(feature = "macos")]
// pub mod macos;
#[cfg(feature = "term")]
#[doc = crate::_tags!(term)]
pub mod term; // Ansi* Term*
// #[doc = crate::_tags!(windows)]
// #[cfg(feature = "windows")]
// pub mod windows;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        #[cfg(all(feature = "unsafe_ffi", not(feature = "safe_sys")))]
        pub use super::c::*;
    }
    _pub_mods {
        pub use super::{
            browser::_all::*,
            fd::_all::*,
        };
        // #[cfg(feature = "android")]
        // pub use super::android::_all::*;
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
        #[cfg(feature = "_linux_abi")]
        pub(crate) use super::linux::_crate_internals::*;
        #[cfg(feature = "term")]
        pub(crate) use super::term::_crate_internals::*;
        pub(crate) use super::browser::_crate_internals::*;
    }
}
