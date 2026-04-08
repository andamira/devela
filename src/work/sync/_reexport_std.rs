// devela::work::sync::_reexport_std
//
//!
//
// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)

#[allow(unused_imports, reason = "symlinked from devela")]
use crate::{_reexport, _tags};

/* `std` structs */

_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "Enables multiple threads to synchronize the beginning of some computation.",
    Barrier
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency result),
    doc: "Returned by [`Barrier::wait()`] when all threads in it have rendezvoused.",
    BarrierWaitResult
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "A Condition Variable.",
    Condvar
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency init),
    doc: "A value which is initialized on the first access.",
    LazyLock
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "A mutual exclusion primitive useful for protecting shared data.",
    Mutex
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency guard),
    doc: "An RAII implementation of a \"scoped lock\" of a mutex.",
    MutexGuard
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "A synchronization primitive for one-time global initialization.",
    Once
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "A synchronization primitive which can be written to only once.",
    OnceLock
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "State yielded to [`Once::call_once_force()`]'s closure parameter.",
    OnceState
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency error),
    doc: "A type of error which can be returned whenever a lock is acquired.",
    PoisonError
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "A reader-writer lock",
    RwLock
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "RAII structure used to release the shared read access of a lock when dropped.",
    RwLockReadGuard
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency),
    doc: "RAII structure used to release the exclusive write access of a lock when dropped.",
    RwLockWriteGuard
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency result),
    doc: "Whether a timed wait on a [`Condvar`] returned due to a time out or not.",
    WaitTimeoutResult
}

/* `std` enums */

_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency error_composite),
    doc: "An enumeration of possible errors associated with a [`TryLockResult`].",
    TryLockError
}

/* `std` aliases */

_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency result),
    doc: "A type alias for the result of a lock method which can be poisoned.",
    LockResult
}
_reexport! { rust: std::sync, location: "work/sync", tag: _tags!(concurrency result),
    doc: "A type alias for the result of a nonblocking locking method.",
    TryLockResult
}
