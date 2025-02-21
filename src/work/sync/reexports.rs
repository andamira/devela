// devela::work::sync::reexports
//
//! Reexported items.
//
// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)

use crate::reexport;

/* from either `alloc` or `portable-atomic-util` */

#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A thread-safe reference-counting pointer.\n\n"]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::Arc;
//
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A thread-safe reference-counting pointer.\n\n"]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::sync::Arc;

#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A version of [`Arc`] that holds a non-owning reference to the managed allocation.\n\n"]
#[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::portable_atomic_util::Weak as ArcWeak;
//
#[doc = crate::TAG_ATOMIC_ALLOC_PORTABLE_UTIL!()]
#[doc = "A version of [`Arc`] that holds a non-owning reference to the managed allocation.\n\n"]
#[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub use crate::_dep::_alloc::sync::Weak as ArcWeak;

/* `std` structs */

reexport! { rust: std::sync,
    doc: "Enables multiple threads to synchronize the beginning of some computation.",
    Barrier
}
reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "Returned by [`Barrier::wait()`] when all threads in the Barrier have rendezvoused.",
    BarrierWaitResult
}
reexport! { rust: std::sync,
    doc: "A Condition Variable",
    Condvar
}
reexport! { rust: std::sync,
    doc: "A value which is initialized on the first access.",
    LazyLock
}
reexport! { rust: std::sync,
    doc: "A mutual exclusion primitive useful for protecting shared data.",
    Mutex
}
reexport! { rust: std::sync,
    doc: "An RAII implementation of a “scoped lock” of a mutex.",
    MutexGuard
}
reexport! { rust: std::sync,
    doc: "A synchronization primitive for one-time global initialization.",
    Once
}
reexport! { rust: std::sync,
    doc: "A synchronization primitive which can be written to only once.",
    OnceLock
}
reexport! { rust: std::sync,
    doc: "State yielded to [`Once::call_once_force()`]’s closure parameter.",
    OnceState
}
reexport! { rust: std::sync,
    tag: crate::TAG_ERROR!(),
    doc: "A type of error which can be returned whenever a lock is acquired.",
    PoisonError
}
reexport! { rust: std::sync,
    doc: "A reader-writer lock",
    RwLock
}
reexport! { rust: std::sync,
    doc: "RAII structure used to release the shared read access of a lock when dropped.",
    RwLockReadGuard
}
reexport! { rust: std::sync,
    doc: "RAII structure used to release the exclusive write access of a lock when dropped.",
    RwLockWriteGuard
}
reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "Whether a timed wait on a condition variable returned due to a time out or not.",
    WaitTimeoutResult
}

/* `std` enums */

reexport! { rust: std::sync,
    tag: crate::TAG_ERROR_COMPOSITE!(),
    doc: "An enumeration of possible errors associated with a [`TryLockResult`].",
    TryLockError
}

/* `std` aliases */

reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "A type alias for the result of a lock method which can be poisoned.",
    LockResult
}
reexport! { rust: std::sync,
    tag: crate::TAG_RESULT!(),
    doc: "A type alias for the result of a nonblocking locking method.",
    TryLockResult
}
