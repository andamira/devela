// devela::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()]
//!
// #![doc = crate::_doc!(extends: alloc)]

mod reexports;
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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{namespace::*, reexports::*};
        #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
        pub use super::{bump::*, wasm::*};
    }
}
