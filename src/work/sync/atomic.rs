// devela::work::sync::atomic
//
//! Atomic types.
//!
//! It also reexports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `core`.
//

use crate::code::reexport;

/* from `core` */

// enums
reexport! { rust: core::sync::atomic, local_module: "work",
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
reexport! { rust: core::sync::atomic, local_module: "work",
    doc: "An atomic fence.",
    @fence as atomic_fence
}
reexport! { rust: core::sync::atomic, local_module: "work",
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}

/* from the `atomic` crate */

reexport! { "atomic" | atomic, features: "work",
    doc: "A generic atomic wrapper type.",
    Atomic
}

#[cfg(any(feature = "dep", feature = "atomic"))]
mod impl_const_default_for_atomic {
    #![allow(clippy::declare_interior_mutable_const)]
    use super::Atomic;
    use crate::code::{impl_cdef, ConstDefault};
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => Atomic<T>];
}

/* from `portable-atomic` */

reexport! { "portable-atomic" | portable_atomic, features: "work",
    doc: "A floating point type which can be safely shared between threads.",
    AtomicF32, AtomicF64
}
reexport! { "portable-atomic" | portable_atomic, features: "work",
    doc: "A signed integer type which can be safely shared between threads.",
    AtomicI128
}
reexport! { "portable-atomic" | portable_atomic, features: "work",
    doc: "An unsigned integer type which can be safely shared between threads.",
    AtomicU128
}

#[cfg(any(feature = "dep", feature = "portable-atomic"))]
mod impl_const_default_for_portable_atomic {
    #![allow(clippy::declare_interior_mutable_const)]
    use super::{AtomicF32, AtomicF64, AtomicU128};
    use crate::code::{impl_cdef, ConstDefault};
    impl_cdef![Self::new(f32::DEFAULT) => AtomicF32];
    impl_cdef![Self::new(f64::DEFAULT) => AtomicF64];
    impl_cdef![Self::new(u128::DEFAULT) => AtomicU128];
}

/* from either `portable-atomic` or `core` */

// MAYBE: IMPROVE create new arm in `reexport` to deal with this case:

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "dep", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use crate::_deps::portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "dep", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use crate::_deps::portable_atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "16"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "32"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "64"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "8"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "ptr"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "dep", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use crate::_deps::portable_atomic::AtomicPtr;
//
#[cfg(all(
    not(any(feature = "dep", feature = "portable-atomic")),
    target_has_atomic = "ptr"
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::AtomicPtr;

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(any(feature = "dep", feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use crate::_deps::portable_atomic::AtomicBool;
//
#[cfg(not(any(feature = "dep", feature = "portable-atomic")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
pub use core::sync::atomic::AtomicBool;

mod impl_const_default_for_either {
    #![allow(clippy::declare_interior_mutable_const)]
    use super::{
        AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16, AtomicU32,
        AtomicU64, AtomicU8, AtomicUsize,
    };
    use crate::code::{impl_cdef, ConstDefault};
    impl_cdef![Self::new(i8::DEFAULT) => AtomicI8];
    impl_cdef![Self::new(i16::DEFAULT) => AtomicI16];
    impl_cdef![Self::new(i32::DEFAULT) => AtomicI32];
    impl_cdef![Self::new(i64::DEFAULT) => AtomicI64];
    impl_cdef![Self::new(isize::DEFAULT) => AtomicIsize];
    impl_cdef![Self::new(u8::DEFAULT) => AtomicU8];
    impl_cdef![Self::new(u16::DEFAULT) => AtomicU16];
    impl_cdef![Self::new(u32::DEFAULT) => AtomicU32];
    impl_cdef![Self::new(u64::DEFAULT) => AtomicU64];
    impl_cdef![Self::new(usize::DEFAULT) => AtomicUsize];
    impl_cdef![<T> Self::new(core::ptr::null_mut()) => AtomicPtr<T>];
}
