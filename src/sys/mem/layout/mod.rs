// devela::sys::mem::layout
//
#![doc = crate::_DOC_SYS_MEM_LAYOUT!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; layout)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow, slice)]
//

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
mod pod; // MemPod

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     _::_all::*,
        // };
        #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_layout")))]
        pub use super::pod::MemPod;
    }
}
