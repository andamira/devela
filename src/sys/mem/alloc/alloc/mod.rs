// devela::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]

#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /crates/base/alloc/src/sys/mem/alloc/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /crates/base/std/src/sys/mem/alloc/_reexport.rs

mod namespace;

#[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "unsafe_layout"))))]
mod bump;

#[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
#[cfg_attr(
    nightly_doc,
    doc(cfg(all(feature = "alloc", feature = "unsafe_layout", target_arch = "wasm32")))
)]
mod wasm;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
        #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
        pub use super::{bump::*, wasm::*};
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
