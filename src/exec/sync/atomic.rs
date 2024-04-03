// devela::exec::sync::atomic
//
//! Atomic types.
//!
//! It also re-exports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `core`.
//

use crate::code::reexport;

/* from `core` */

// enums
reexport! { rust: core::sync::atomic,
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
reexport! { rust: core::sync::atomic,
    doc: "An atomic fence.",
    @fence as atomic_fence
}
reexport! { rust: core::sync::atomic,
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}

/* from the `atomic` crate */

reexport! { "atomic" | atomic, features: "exec",
    doc: "A generic atomic wrapper type.",
    Atomic
}

/* from `portable-atomic` */

reexport! { "portable-atomic" | portable_atomic, features: "exec",
    doc: "A floating point type which can be safely shared between threads.",
    AtomicF32, AtomicF64
}
reexport! { "portable-atomic" | portable_atomic, features: "exec",
    doc: "A signed integer type which can be safely shared between threads.",
    AtomicI128
}
reexport! { "portable-atomic" | portable_atomic, features: "exec",
    doc: "An unsigned integer type which can be safely shared between threads.",
    AtomicU128
}

/* from either `portable-atomic` or `core` */

// MAYBE: IMPROVE create new arm in `reexport` to deal with this case:

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A signed integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use crate::_deps::portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "An unsigned integer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use crate::_deps::portable_atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "16"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "32"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "64"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "8"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "ptr"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A raw pointer type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use crate::_deps::portable_atomic::AtomicPtr;
//
#[cfg(all(not(feature = "portable-atomic"), target_has_atomic = "ptr"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::AtomicPtr;

/// <span class="stab portability" title="re-exported either from `core` or from the
/// `portable-atomic` crate">`*`</span>
#[doc = "A boolean type which can be safely shared between threads.\n\n"]
#[doc = "*Re-exported from the [`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---"]
#[cfg(feature = "portable-atomic")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use crate::_deps::portable_atomic::AtomicBool;
//
#[cfg(not(feature = "portable-atomic"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
pub use core::sync::atomic::AtomicBool;

/* impl ConstDefaut */

#[cfg(feature = "atomic")]
mod impl_const_default_for_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::code::{impl_cdef, ConstDefault};
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => super::Atomic<T>];
}
#[cfg(feature = "portable-atomic")]
mod impl_const_default_for_portable_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::code::{impl_cdef, ConstDefault};

    // there are no core alternatives
    impl_cdef![Self::new(f32::DEFAULT) => super::AtomicF32];
    impl_cdef![Self::new(f64::DEFAULT) => super::AtomicF64];
    impl_cdef![Self::new(u128::DEFAULT) => super::AtomicU128];

    // core alternatives
    impl_cdef![Self::new(i8::DEFAULT) => super::AtomicI8];
    impl_cdef![Self::new(i16::DEFAULT) => super::AtomicI16];
    impl_cdef![Self::new(i32::DEFAULT) => super::AtomicI32];
    impl_cdef![Self::new(i64::DEFAULT) => super::AtomicI64];
    impl_cdef![Self::new(isize::DEFAULT) => super::AtomicIsize];
    impl_cdef![Self::new(u8::DEFAULT) => super::AtomicU8];
    impl_cdef![Self::new(u16::DEFAULT) => super::AtomicU16];
    impl_cdef![Self::new(u32::DEFAULT) => super::AtomicU32];
    impl_cdef![Self::new(u64::DEFAULT) => super::AtomicU64];
    impl_cdef![Self::new(usize::DEFAULT) => super::AtomicUsize];
    impl_cdef![<T> Self::new(<*mut T>::DEFAULT) => super::AtomicPtr<T>];
}
#[cfg(not(feature = "portable-atomic"))]
mod impl_const_default_for_core {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::code::{impl_cdef, ConstDefault};

    #[cfg(target_has_atomic = "16")]
    impl_cdef![Self::new(i16::DEFAULT) => super::AtomicI16];
    #[cfg(target_has_atomic = "16")]
    impl_cdef![Self::new(u16::DEFAULT) => super::AtomicU16];
    #[cfg(target_has_atomic = "32")]
    impl_cdef![Self::new(i32::DEFAULT) => super::AtomicI32];
    #[cfg(target_has_atomic = "32")]
    impl_cdef![Self::new(u32::DEFAULT) => super::AtomicU32];
    #[cfg(target_has_atomic = "64")]
    impl_cdef![Self::new(i64::DEFAULT) => super::AtomicI64];
    #[cfg(target_has_atomic = "64")]
    impl_cdef![Self::new(u64::DEFAULT) => super::AtomicU64];
    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![Self::new(isize::DEFAULT) => super::AtomicIsize];
    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![Self::new(usize::DEFAULT) => super::AtomicUsize];
    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![<T> Self::new(<*mut T>::DEFAULT) => super::AtomicPtr<T>];
}
