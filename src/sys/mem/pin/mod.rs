// devela::sys::mem::pin
//
#![doc = crate::_DOC_SYS_MEM_PIN!()]
//!
#![doc = crate::_doc!(extends: pin)]
//

crate::mod_path!(_c "../../../../libs/base/src/sys/mem/pin/reexports.rs");

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_ptr")))]
mod pinned; // Pinned

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::_c::*;

        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_ptr"))]
        pub use super::pinned::*;
    }
    _always {
        pub use super::_c::*;
    }
}
