// devela::sys::mem::pin
//
#![doc = crate::_DOC_SYS_MEM_PIN!()]
//!
#![doc = crate::_doc!(extends: pin)]
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/sys/mem/pin/_reexport.rs

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ptr")))]
mod pinned; // Pinned

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
        pub use super::pinned::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
