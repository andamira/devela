// devela::work::sync::atomic::reexports
//
//! Re-exports items from core, [portable-atomic](https://docs.rs/portable-atomic),
//! and the [`Atomic`] type from the [atomic](https://docs.rs/atomic) crate.
//

#[cfg(feature = "dep_portable_atomic")]
use crate::_DOC_ATOMIC_CORE_PORTABLE;
use crate::{_TAG_ATOMIC, _TAG_ATOMIC_CORE_PORTABLE, _reexport};

crate::mod_path!(+pub _c "../../../../libs/base_core/src/work/sync/atomic/reexports.rs");

/* from the `atomic` crate */

_reexport! { "dep_atomic", "atomic", atomic,
    tag: _TAG_ATOMIC!(),
    doc: "A generic atomic wrapper type.",
    Atomic
}

/* from `portable-atomic` */

_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: _TAG_ATOMIC!(),
    doc: "A thread-safe floating-point type.",
    AtomicF32, AtomicF64
}
_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: _TAG_ATOMIC!(),
    doc: "A thread-safe signed integer type.",
    AtomicI128
}
_reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
    tag: _TAG_ATOMIC!(),
    doc: "A thread-safe unsigned integer type.",
    AtomicU128
}

/* from either `portable-atomic` or `core` */

#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[doc = "A thread-safe signed integer type.\n\n"]
#[doc = _DOC_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize};

#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[doc = "A thread-safe unsigned integer type.\n\n"]
#[doc = _DOC_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize};

#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "8"))]
pub use core::sync::atomic::{AtomicI8, AtomicU8};
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "16"))]
pub use core::sync::atomic::{AtomicI16, AtomicU16};
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "32"))]
pub use core::sync::atomic::{AtomicI32, AtomicU32};
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "64"))]
pub use core::sync::atomic::{AtomicI64, AtomicU64};
//
// WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
// #[doc = _TAG_ATOMIC!()]
// #[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
// #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "128"))]
// pub use core::sync::atomic::{AtomicI128, AtomicU128};
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::AtomicPtr;
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
pub use core::sync::atomic::AtomicPtr;

#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(feature = "dep_portable_atomic")]
pub use crate::_dep::portable_atomic::AtomicBool;
//
#[doc = _TAG_ATOMIC!()]
#[doc = _TAG_ATOMIC_CORE_PORTABLE!()]
#[cfg(not(feature = "dep_portable_atomic"))]
pub use core::sync::atomic::AtomicBool;

/* impl ConstInit */

#[cfg(feature = "dep_atomic")]
mod impl_const_init_for_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{_impl_init, ConstInit};
    _impl_init![ConstInit: <T: ConstInit> Self::new(T::INIT) => super::Atomic<T>];
}
#[cfg(feature = "dep_portable_atomic")]
mod impl_const_init_for_portable_atomic {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{_impl_init, ConstInit};

    // without core alternatives:
    _impl_init![ConstInit: Self::new(f32::INIT) => super::AtomicF32];
    _impl_init![ConstInit: Self::new(f64::INIT) => super::AtomicF64];

    // with core alternatives:
    _impl_init![ConstInit: Self::new(i8::INIT) => super::AtomicI8];
    _impl_init![ConstInit: Self::new(u8::INIT) => super::AtomicU8];
    _impl_init![ConstInit: Self::new(i16::INIT) => super::AtomicI16];
    _impl_init![ConstInit: Self::new(u16::INIT) => super::AtomicU16];
    _impl_init![ConstInit: Self::new(i32::INIT) => super::AtomicI32];
    _impl_init![ConstInit: Self::new(u32::INIT) => super::AtomicU32];
    _impl_init![ConstInit: Self::new(i64::INIT) => super::AtomicI64];
    _impl_init![ConstInit: Self::new(u64::INIT) => super::AtomicU64];
    _impl_init![ConstInit: Self::new(i128::INIT) => super::AtomicI128];
    _impl_init![ConstInit: Self::new(u128::INIT) => super::AtomicU128];
    _impl_init![ConstInit: Self::new(isize::INIT) => super::AtomicIsize];
    _impl_init![ConstInit: Self::new(usize::INIT) => super::AtomicUsize];
    _impl_init![ConstInit: <T> Self::new(<*mut T>::INIT) => super::AtomicPtr<T>];
}
#[cfg(not(feature = "dep_portable_atomic"))]
mod impl_const_init_for_core {
    #![allow(clippy::declare_interior_mutable_const, unused_imports)]
    use crate::{_impl_init, ConstInit};

    #[cfg(target_has_atomic = "8")]
    _impl_init![ConstInit: Self::new(i8::INIT) => super::AtomicI8];
    #[cfg(target_has_atomic = "8")]
    _impl_init![ConstInit: Self::new(u8::INIT) => super::AtomicU8];
    #[cfg(target_has_atomic = "16")]
    _impl_init![ConstInit: Self::new(i16::INIT) => super::AtomicI16];
    #[cfg(target_has_atomic = "16")]
    _impl_init![ConstInit: Self::new(u16::INIT) => super::AtomicU16];
    #[cfg(target_has_atomic = "32")]
    _impl_init![ConstInit: Self::new(i32::INIT) => super::AtomicI32];
    #[cfg(target_has_atomic = "32")]
    _impl_init![ConstInit: Self::new(u32::INIT) => super::AtomicU32];
    #[cfg(target_has_atomic = "64")]
    _impl_init![ConstInit: Self::new(i64::INIT) => super::AtomicI64];
    #[cfg(target_has_atomic = "64")]
    _impl_init![ConstInit: Self::new(u64::INIT) => super::AtomicU64];

    // WAIT: [AtomicU128/AtomicI128 not shown](https://github.com/rust-lang/rust/issues/130539)
    // WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
    // #[cfg(target_has_atomic = "128")]
    // #[cfg(feature = "nightly_atomics")]
    // _impl_init![Self::new(i128::INIT) => super::AtomicI128];
    // #[cfg(target_has_atomic = "128")]
    // #[cfg(feature = "nightly_atomics")]
    // _impl_init![Self::new(u128::INIT) => super::AtomicU128];

    #[cfg(target_has_atomic = "ptr")]
    _impl_init![ConstInit: Self::new(isize::INIT) => super::AtomicIsize];
    #[cfg(target_has_atomic = "ptr")]
    _impl_init![ConstInit: Self::new(usize::INIT) => super::AtomicUsize];
    #[cfg(target_has_atomic = "ptr")]
    _impl_init![ConstInit: <T> Self::new(<*mut T>::INIT) => super::AtomicPtr<T>];
}
