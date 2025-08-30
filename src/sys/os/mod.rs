// devela::sys::os
//
//! OS-specific.
#![doc = crate::_doc!(modules: crate::sys; os: linux)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: os)]
//

mod print; // os_[e]print[ln]!

#[cfg(feature = "linux")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "linux")))]
pub mod linux;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::print::*;
    }
    _pub_mods {
        #[cfg(feature = "linux")]
        pub use super::linux::_all::*;
    }
}
