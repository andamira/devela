// devela::sync::atomic
//
//! Atomic types.
//!
//! It also reexports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `libcore`.
//

/* reexport from the `atomic` crate */

/// <span class="stab portability" title="re-exported from the `atomic` crate">`atomic`</span>
#[doc = "A generic atomic wrapper type.\n\n"]
#[doc = "*Reexported from the [`atomic`](https://docs.rs/atomic)* crate.\n\n---"]
#[doc(inline)]
#[cfg(feature = "atomic")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "atomic")))
)]
pub use ::atomic::Atomic;

/* only in `portable-atomic` */

/// <span class="stab portability" title="re-exported from the `portable-atomic`
/// crate">`portable-atomic`</span>
#[doc = "A floating point type which can be safely shared between threads..\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::{AtomicF32, AtomicF64};

/// <span class="stab portability" title="re-exported from the `portable-atomic`
/// crate">`portable-atomic`</span>
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::AtomicI128;

/// <span class="stab portability" title="re-exported from the `portable-atomic`
/// crate">`portable-atomic`</span>
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::AtomicU128;

/* in either `portable-atomic` or `core` */

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "16"))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "32"))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "64"))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "8"))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::AtomicPtr;

#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::AtomicPtr;

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::AtomicBool;

#[cfg(not(feature = "portable-atomic"))]
pub use core::sync::atomic::AtomicBool;

/* in `core` */

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "An atomic fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::fence;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "A compiler memory fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::compiler_fence;

/// <span class="stab portability" title="re-exported from `core`">`core`</span>
#[doc = "Atomic memory ordering.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::Ordering as AtomicOrdering;
