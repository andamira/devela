// devela::sys::env::arg
//
//! Parse arguments to the program.
//

#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(feature = "std", feature = "unsafe_ffi"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "std", feature = "unsafe_ffi"))))]
mod os_ref; // ArgsOsRefIter + TEMP args_os_ref_iter (make impl for Env

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(feature = "std", feature = "unsafe_ffi"))]
        pub use super::os_ref::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
