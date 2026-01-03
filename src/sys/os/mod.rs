// devela::sys::os
//
#![doc = crate::_DOC_SYS_OS!()]
#![doc = crate::_doc!(modules: crate::sys; os: linux)]
#![doc = crate::_doc!(newline)]
//!
//! Kernel-backed or virtualized environments that define the core capability
//! surfaces available to applications. This includes native OS layers
//! (Linux, macOS, Windows), compatibility layers (libc), and sandboxed
//! host environments such as browser runtimes.
#![doc = crate::_doc!(extends: os)]
//
// OSes: (https://doc.rust-lang.org/beta/rustc/platform-support.html)
// - https://motor-os.org/ | https://github.com/moturus/motor-os
// - https://hermit-os.org/ | https://github.com/hermit-os/hermit-rs
// - https://source.android.com/docs/security/features/trusty
// - https://wasi.dev/ | https://github.com/WebAssembly/WASI

mod print; // os_[e]print[ln]!

#[cfg(feature = "unsafe_syscall")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
mod c; // Libc

pub mod browser; // Web*
pub mod fd;

#[cfg(feature = "linux")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linux")))]
pub mod linux;
// #[cfg(feature = "macos")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "macos")))]
// pub mod macos;
// #[cfg(feature = "windows")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "windows")))]
// pub mod windows;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(feature = "unsafe_syscall")]
        pub use super::c::*;

        pub use super::print::*;
    }
    _pub_mods {
        pub use super::{
            browser::_all::*,
            fd::_all::*,
        };
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;
        // #[cfg(feature = "macos")]
        // pub use super::macos::_all::*;
        // #[cfg(feature = "windows")]
        // pub use super::windows::_all::*;
    }
}
