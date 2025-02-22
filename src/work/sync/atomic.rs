// devela::work::sync::atomic
//
//! Atomic types.
//!
//! It also re-exports the [`Atomic`] type from the
//! [`atomic`](https://docs.rs/atomic) crate,
//! and some useful definitions from `core`.
//

use crate::{reexport, TAG_ATOMIC, TAG_ATOMIC_CORE_PORTABLE};
#[cfg(feature = "dep_portable_atomic")]
use crate::DOC_ATOMIC_CORE_PORTABLE;

/* from `core` */

// enums
reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "Atomic memory ordering.",
    @Ordering as AtomicOrdering
}

// functions
reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "An atomic fence.",
    @fence as atomic_fence
}
reexport! { rust: core::sync::atomic,
    tag: TAG_ATOMIC!(),
    doc: "A compiler memory fence.",
    @compiler_fence as atomic_compiler_fence
}

/* from the `atomic` crate */

reexport! { "dep_atomic", "atomic", atomic,
    tag: TAG_ATOMIC!(),
    doc: "A generic atomic wrapper type.",
    Atomic
}

/* from `portable-atomic` */

reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: TAG_ATOMIC!(),
    doc: "A thread-safe floating-point type.",
    AtomicF32, AtomicF64
}
reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: TAG_ATOMIC!(),
    doc: "A thread-safe signed integer type.",
    AtomicI128
}
reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: TAG_ATOMIC!(),
    doc: "A thread-safe unsigned integer type.",
    AtomicU128
}

/* from either `portable-atomic` or `core` */

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[doc = "A thread-safe signed integer type.\n\n"]
#[doc = DOC_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::{AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize};

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[doc = "A thread-safe unsigned integer type.\n\n"]
#[doc = DOC_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize};

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "8"))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "16"))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "32"))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "64"))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
//
// WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
// #[doc = TAG_ATOMIC!()]
// #[doc = TAG_ATOMIC_CORE_PORTABLE!()]
// #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "128"))]
// pub use core::sync::atomic::{AtomicI128, AtomicU128};
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::AtomicPtr;
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::AtomicPtr;

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::AtomicBool;
//
#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(not(feature = "dep_portable_atomic"))]
pub use core::sync::atomic::AtomicBool;

/* impl ConstDefaut */

#[cfg(feature = "dep_atomic")]
mod impl_const_default_for_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{impl_cdef, ConstDefault};
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => super::Atomic<T>];
}
#[cfg(feature = "dep_portable_atomic")]
mod impl_const_default_for_portable_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{impl_cdef, ConstDefault};

    // without core alternatives:
    impl_cdef![Self::new(f32::DEFAULT) => super::AtomicF32];
    impl_cdef![Self::new(f64::DEFAULT) => super::AtomicF64];

    // with core alternatives:
    impl_cdef![Self::new(i8::DEFAULT) => super::AtomicI8];
    impl_cdef![Self::new(u8::DEFAULT) => super::AtomicU8];
    impl_cdef![Self::new(i16::DEFAULT) => super::AtomicI16];
    impl_cdef![Self::new(u16::DEFAULT) => super::AtomicU16];
    impl_cdef![Self::new(i32::DEFAULT) => super::AtomicI32];
    impl_cdef![Self::new(u32::DEFAULT) => super::AtomicU32];
    impl_cdef![Self::new(i64::DEFAULT) => super::AtomicI64];
    impl_cdef![Self::new(u64::DEFAULT) => super::AtomicU64];
    impl_cdef![Self::new(i128::DEFAULT) => super::AtomicI128];
    impl_cdef![Self::new(u128::DEFAULT) => super::AtomicU128];
    impl_cdef![Self::new(isize::DEFAULT) => super::AtomicIsize];
    impl_cdef![Self::new(usize::DEFAULT) => super::AtomicUsize];
    impl_cdef![<T> Self::new(<*mut T>::DEFAULT) => super::AtomicPtr<T>];
}
#[cfg(not(feature = "dep_portable_atomic"))]
mod impl_const_default_for_core {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{impl_cdef, ConstDefault};

    #[cfg(target_has_atomic = "8")]
    impl_cdef![Self::new(i8::DEFAULT) => super::AtomicI8];
    #[cfg(target_has_atomic = "8")]
    impl_cdef![Self::new(u8::DEFAULT) => super::AtomicU8];
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

    // WAIT: [AtomicU128/AtomicI128 not shown](https://github.com/rust-lang/rust/issues/130539)
    // WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
    // #[cfg(target_has_atomic = "128")]
    // #[cfg(feature = "nightly_atomics")]
    // impl_cdef![Self::new(i128::DEFAULT) => super::AtomicI128];
    // #[cfg(target_has_atomic = "128")]
    // #[cfg(feature = "nightly_atomics")]
    // impl_cdef![Self::new(u128::DEFAULT) => super::AtomicU128];

    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![Self::new(isize::DEFAULT) => super::AtomicIsize];
    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![Self::new(usize::DEFAULT) => super::AtomicUsize];
    #[cfg(target_has_atomic = "ptr")]
    impl_cdef![<T> Self::new(<*mut T>::DEFAULT) => super::AtomicPtr<T>];
}
