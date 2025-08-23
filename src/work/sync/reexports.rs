// devela::work::sync::reexports
//
//! Reexported items.
//
// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)

use crate::_reexport;

/* from either `alloc` or `portable-atomic-util` */

#[doc = crate::TAG_ATOMIC!()]
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A thread-safe reference-counting pointer.\n\n"]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::Arc;
//
#[doc = crate::TAG_ATOMIC!()]
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A thread-safe reference-counting pointer.\n\n"]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::sync::Arc;

#[doc = crate::TAG_ATOMIC!()]
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A version of [`Arc`] that holds a non-owning reference.\n\n"]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::Weak as ArcWeak;
//
#[doc = crate::TAG_ATOMIC!()]
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A version of [`Arc`] that holds a non-owning reference.\n\n"]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::sync::Weak as ArcWeak;

/* `std` structs */

_reexport! { rust: std::sync,
    doc: "Enables multiple threads to synchronize the beginning of some computation.",
    Barrier
}
_reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "Returned by [`Barrier::wait()`] when all threads in it have rendezvoused.",
    BarrierWaitResult
}
_reexport! { rust: std::sync,
    doc: "A Condition Variable",
    Condvar
}
_reexport! { rust: std::sync,
    doc: "A value which is initialized on the first access.",
    LazyLock
}
_reexport! { rust: std::sync,
    doc: "A mutual exclusion primitive useful for protecting shared data.",
    Mutex
}
_reexport! { rust: std::sync,
    doc: "An RAII implementation of a “scoped lock” of a mutex.",
    MutexGuard
}
_reexport! { rust: std::sync,
    doc: "A synchronization primitive for one-time global initialization.",
    Once
}
_reexport! { rust: std::sync,
    doc: "A synchronization primitive which can be written to only once.",
    OnceLock
}
_reexport! { rust: std::sync,
    doc: "State yielded to [`Once::call_once_force()`]’s closure parameter.",
    OnceState
}
_reexport! { rust: std::sync,
    tag: crate::TAG_ERROR!(),
    doc: "A type of error which can be returned whenever a lock is acquired.",
    PoisonError
}
_reexport! { rust: std::sync,
    doc: "A reader-writer lock",
    RwLock
}
_reexport! { rust: std::sync,
    doc: "RAII structure used to release the shared read access of a lock when dropped.",
    RwLockReadGuard
}
_reexport! { rust: std::sync,
    doc: "RAII structure used to release the exclusive write access of a lock when dropped.",
    RwLockWriteGuard
}
_reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "Whether a timed wait on a condition variable returned due to a time out or not.",
    WaitTimeoutResult
}

/* `std` enums */

_reexport! { rust: std::sync,
    tag: crate::TAG_ERROR_COMPOSITE!(),
    doc: "An enumeration of possible errors associated with a [`TryLockResult`].",
    TryLockError
}

/* `std` aliases */

_reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "A type alias for the result of a lock method which can be poisoned.",
    LockResult
}
_reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "A type alias for the result of a nonblocking locking method.",
    TryLockResult
}
