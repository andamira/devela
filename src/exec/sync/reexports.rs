// devela::exec::sync::reexports
//
//! Reexported items.
//

use crate::code::reexport;

/* structs */

reexport! { rust: alloc::sync,
    doc: "A thread-safe reference-counting pointer.",
    Arc
}
reexport! { rust: std::sync,
    doc: "Enables multiple threads to synchronize the beginning of some computation.",
    Barrier
}
reexport! { rust: std::sync,
    doc: "Returned by [`Barrier::wait()`] when all threads in the Barrier have rendezvoused.",
    BarrierWaitResult
}
reexport! { rust: std::sync,
    doc: "A Condition Variable",
    Condvar
}
reexport! { rust: std::sync,
    doc: "A mutual exclusion primitive useful for protecting shared data",
    Mutex
}
reexport! { rust: std::sync,
    doc: "An RAII implementation of a “scoped lock” of a mutex.",
    MutexGuard
}
reexport! { rust: std::sync,
    doc: "A synchronization primitive which can be used to run a one-time global initialization.",
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
    doc: "A type indicating whether a timed wait on a condition variable returned
        due to a time out or not.",
    WaitTimeoutResult
}
reexport! { rust: alloc::sync,
    doc: "A version of [`Arc`] that holds a non-owning reference to the managed allocation.",
    @Weak as ArcWeak
}

/* enums */

reexport! { rust: std::sync,
    doc: "An enumeration of possible errors associated with a [`TryLockResult`].",
    TryLockError
}

/* aliases */

reexport! { rust: std::sync,
    doc: "A type alias for the result of a lock method which can be poisoned.",
    LockResult
}
reexport! { rust: std::sync,
    doc: "A type alias for the result of a nonblocking locking method.",
    TryLockResult
}
