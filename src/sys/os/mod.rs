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

mod print; // os_[e]print[ln]!

#[cfg(feature = "linux")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linux")))]
pub mod linux;

// #[cfg(feature = "windows")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "windows")))]
// pub mod windows;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::print::*;
    }
    _pub_mods {
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;

        // #[cfg(feature = "windows")]
        // pub use super::windows::_all::*;
    }
}
