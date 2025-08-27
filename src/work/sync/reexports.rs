// devela::work::sync::reexports
//
//!
//
// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)

crate::mod_path!(std +pub _s "../../../libs/base_std/src/work/sync/reexports.rs");

/* from either `alloc` or `portable-atomic-util` */

#[cfg(feature = "alloc")]
crate::items! {
    #[doc = crate::TAG_ATOMIC!()]
    #[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
    #[doc = "A thread-safe reference-counting pointer.\n\n"]
    #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::portable_atomic_util::Arc;
    //
    #[doc = crate::TAG_ATOMIC!()]
    #[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
    #[doc = "A thread-safe reference-counting pointer.\n\n"]
    #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::_alloc::sync::Arc;

    #[doc = crate::TAG_ATOMIC!()]
    #[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
    #[doc = "A version of [`Arc`] that holds a non-owning reference.\n\n"]
    #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::portable_atomic_util::Weak as ArcWeak;
    //
    #[doc = crate::TAG_ATOMIC!()]
    #[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
    #[doc = "A version of [`Arc`] that holds a non-owning reference.\n\n"]
    #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::_alloc::sync::Weak as ArcWeak;
}
