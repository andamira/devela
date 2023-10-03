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
pub use ::atomic::Atomic;

/* reexport from the `portable-atomic` crate */

#[cfg(feature = "atomic")]
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
pub use portable_atomic::{AtomicI128, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

#[cfg(feature = "atomic")]
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
pub use portable_atomic::{AtomicU128, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(feature = "atomic")]
#[doc = "A floating point type which can be safely shared between threads..\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
pub use portable_atomic::{AtomicF32, AtomicF64};

#[cfg(feature = "atomic")]
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
pub use portable_atomic::AtomicBool;

#[cfg(feature = "atomic")]
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Reexported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
pub use portable_atomic::AtomicPtr;

/* reexport from `libcore` */

#[doc = "An atomic fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
pub use core::sync::atomic::fence;

#[doc = "A compiler memory fence.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
pub use core::sync::atomic::compiler_fence;

#[doc = "Atomic memory ordering.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
pub use core::sync::atomic::Ordering as AtomicOrdering;

// NOTE: replaced by portable-atomic types:

// #[doc = "A signed integer type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// pub use core::sync::atomic::{AtomicI16, AtomicI32, AtomicI8, AtomicIsize};
// #[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// pub use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU8, AtomicUsize};

// #[doc = "A signed integer type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// #[cfg(target_has_atomic = "64")]
// pub use core::sync::atomic::AtomicU64;
//
// #[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// #[cfg(target_has_atomic = "64")]
// pub use core::sync::atomic::AtomicU64;

// #[doc = "A boolean type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// pub use core::sync::atomic::AtomicBool;
//
// #[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
// #[doc = "*Reexported from"]
// #[doc = "`core::sync::`[`atomic`](https://doc.rust-lang.org/core/sync/atomic)*.\n\n---"]
// pub use core::sync::atomic::AtomicPtr;
