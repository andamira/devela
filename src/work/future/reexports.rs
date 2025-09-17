// devela::work::future::reexports
//
//!
//

crate::mod_path!(+pub _c "../../../libs/base_core/src/work/future/reexports.rs");

/* from either `alloc` or `portable-atomic-util` and `alloc` */

#[doc = crate::_TAG_ATOMIC!()]
#[doc = crate::_TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::task::Wake as TaskWake;
//
#[doc = crate::_TAG_ATOMIC!()]
#[doc = crate::_TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::task::Wake as TaskWake;
