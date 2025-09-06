// devela::work::sync::atomic::reexports
//
//! Re-exports items from core, [portable-atomic](https://docs.rs/portable-atomic),
//! and the [`Atomic`] type from the [atomic](https://docs.rs/atomic) crate.
//

#[cfg(feature = "dep_portable_atomic")]
use crate::DOC_ATOMIC_CORE_PORTABLE;
use crate::{_reexport, TAG_ATOMIC, TAG_ATOMIC_CORE_PORTABLE};

crate::mod_path!(+pub _c "../../../../libs/base_core/src/work/sync/atomic/reexports.rs");

/* from the `atomic` crate */

_reexport! { "dep_atomic", "atomic", atomic,
    tag: TAG_ATOMIC!(),
    doc: "A generic atomic wrapper type.",
    Atomic
}

/* from `portable-atomic` */

_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: TAG_ATOMIC!(),
    doc: "A thread-safe floating-point type.",
    AtomicF32, AtomicF64
}
_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: TAG_ATOMIC!(),
    doc: "A thread-safe signed integer type.",
    AtomicI128
}
_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
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
pub use crate::_dep::portable_atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize};

#[doc = TAG_ATOMIC!()]
#[doc = TAG_ATOMIC_CORE_PORTABLE!()]
#[doc = "A thread-safe unsigned integer type.\n\n"]
#[doc = DOC_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize};

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
    use crate::{ConstDefault, impl_cdef};
    impl_cdef![<T: ConstDefault> Self::new(T::DEFAULT) => super::Atomic<T>];
}
#[cfg(feature = "dep_portable_atomic")]
mod impl_const_default_for_portable_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{ConstDefault, impl_cdef};

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
    use crate::{ConstDefault, impl_cdef};

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
