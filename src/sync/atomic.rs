// devela::sync::atomic
//
//! Atomic types.
//!
//! It also reexports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `libcore`.
//

/* reexport from the `atomic` crate */

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

#[doc = "A floating point type which can be safely shared between threads..\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::{AtomicF32, AtomicF64};

#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::AtomicI128;

#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "sync", feature = "portable-atomic")))
)]
pub use portable_atomic::AtomicU128;

/* in either `portable-atomic` or `core` */

#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

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

#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::AtomicPtr;
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::AtomicPtr;

#[cfg(not(feature = "portable-atomic"))]
pub use core::sync::atomic::AtomicBool;
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use portable_atomic::AtomicBool;

/* in `core` */

#[doc = "An atomic fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::fence;

#[doc = "A compiler memory fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::compiler_fence;

#[doc = "Atomic memory ordering.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use core::sync::atomic::Ordering as AtomicOrdering;
