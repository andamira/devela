// devela::sys::mem::alloc
//
//! Memory allocation.
// #![doc = crate::doc_!(extends: alloc)]

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

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{namespace::*, reexports::*};
        #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
        pub use super::{bump::*, wasm::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
