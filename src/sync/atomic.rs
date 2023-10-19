// devela::sync::atomic
//
//! Atomic types.
//!
//! It also reexports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `core`.
//

use crate::codegen::reexport;

/* reexport from the `atomic` crate */

reexport! { "atomic" | atomic, features: "sync",
    doc: "A generic atomic wrapper type.",
    Atomic
}

/* from `portable-atomic` */

reexport! { "portable-atomic" | portable_atomic, features: "sync",
    doc: "A floating point type which can be safely shared between threads.",
    AtomicF32, AtomicF64
}
reexport! { "portable-atomic" | portable_atomic, features: "sync",
    doc: "A signed integer type which can be safely shared between threads.",
    AtomicI128
}
reexport! { "portable-atomic" | portable_atomic, features: "sync",
    doc: "An unsigned integer type which can be safely shared between threads.",
    AtomicU128
}

/* from either `portable-atomic` or `core` */

// TODO: IMPROVE create new arm in `reexport` to deal with this case:

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "depend", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use crate::depend::portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "depend", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use crate::depend::portable_atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "16"
))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "32"
))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "64"
))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "8"
))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "ptr"
))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "depend", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use crate::depend::portable_atomic::AtomicPtr;
//
#[cfg(all(
    not(any(feature = "depend", feature = "portable-atomic")),
    target_has_atomic = "ptr"
))]
pub use core::sync::atomic::AtomicPtr;

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "depend", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sync")))]
pub use crate::depend::portable_atomic::AtomicBool;
//
#[cfg(not(any(feature = "depend", feature = "portable-atomic")))]
pub use core::sync::atomic::AtomicBool;

/* from `core` */

reexport! { rust: core::sync::atomic, local_module: "sync",
    doc: "An atomic fence.",
    fence
}
reexport! { rust: core::sync::atomic, local_module: "sync",
    doc: "A compiler memory fence.",
    compiler_fence
}
reexport! { rust: core::sync::atomic, local_module: "sync",
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}
