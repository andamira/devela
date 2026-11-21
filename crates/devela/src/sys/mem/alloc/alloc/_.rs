//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; alloc)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc)]

crate::mods_in! {
    mod namespace;

    #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "unsafe_layout"))))]
    mod bump; // AllocBump

    #[cfg(all(
        target_os = "linux",
        feature = "alloc",
        feature = "unsafe_layout",
        feature = "unsafe_ffi",
        not(feature = "safe_sys"),
    ))]
    mod linux; // LinuxMmapAlloc

    #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
    #[cfg_attr(
        nightly_doc,
        doc(cfg(all(feature = "alloc", feature = "unsafe_layout", target_arch = "wasm32")))
    )]
    mod wasm; // AllocWasm
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
        #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
        pub use super::{bump::*, wasm::*};
        #[cfg(all(feature = "alloc", feature = "unsafe_layout"))]
        #[cfg(all(
            target_os = "linux",
            feature = "alloc",
            feature = "unsafe_layout",
            feature = "unsafe_ffi",
            not(feature = "safe_sys"),
        ))]
        pub use super::linux::*;
    }
    _reexports {
        pub use crate::boot::rust::sys::mem_alloc::*;
    }
}
