// devela::work::sync::_reexport_alloc
//
// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)

use crate::_tags;

/* from either `alloc` or `portable-atomic-util` */

#[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
/// A thread-safe reference-counting pointer.
#[doc = crate::_doc_location!("work/sync")]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::yard::_dep::portable_atomic_util::Arc;
//
#[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
/// A thread-safe reference-counting pointer.
#[doc = crate::_doc_location!("work/sync")]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use alloc::sync::Arc;

#[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
/// A version of [`Arc`] that holds a non-owning reference.
#[doc = crate::_doc_location!("work/sync")]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::yard::_dep::portable_atomic_util::Weak as ArcWeak;

#[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
/// A version of [`Arc`] that holds a non-owning reference.
#[doc = crate::_doc_location!("work/sync")]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use alloc::sync::Weak as ArcWeak;
