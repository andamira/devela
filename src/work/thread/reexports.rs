// devela::work::thread::reexports
//
//! Reexported items.
//

use crate::code::reexport;

/* types */

reexport! { rust: std::thread, local_module: "work",
    doc: "An error returned by [`ThreadLocalKey::try_with`].",
    @AccessError as ThreadAccessError
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Thread factory, which can be used in order to configure the properties
        of a new thread.",
    @Builder as ThreadBuilder
}
reexport! { rust: std::thread, local_module: "work",
    doc: "An owned permission to join on a thread (block on its termination).",
    @JoinHandle as ThreadJoinHandle
}
reexport! { rust: std::thread, local_module: "work",
    doc: "A thread local storage key which owns its contents.",
    @LocalKey as ThreadLocalKey
}
reexport! { rust: std::thread, local_module: "work",
    doc: "A scope to spawn scoped threads in.",
    @Scope as ThreadScope
}
reexport! { rust: std::thread, local_module: "work",
    doc: "An owned permission to join on a scoped thread (block on its termination).",
    @ScopedJoinHandle as ThreadScopedJoinHandle
}
reexport! { rust: std::thread, local_module: "work",
    doc: "A handle to a thread.",
    Thread
}
reexport! { rust: std::thread, local_module: "work",
    doc: "A unique identifier for a running thread.",
    ThreadId
}

/* aliases */
reexport! { rust: std::thread, local_module: "work",
    doc: "A specialized [`Result`] type for threads.",
    @Result as ThreadResult
}

/* functions */

reexport! { rust: std::thread, local_module: "work",
    doc: "Returns an estimate of the default amount of parallelism a program should use.",
    @available_parallelism as thread_parallelism
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Gets a handle to the thread that invokes it.",
    @current as thread_current
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Determines whether the current thread is unwinding because of panic.",
    @panicking as thread_panicking
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Blocks unless or until the current thread’s token is made available.",
    @park as thread_park
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Blocks unless or until the current thread’s token is made available
        or the specified duration has been reached (may wake spuriously).",
    @park_timeout as thread_park_timeout
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Create a scope for spawning scoped threads.",
    @scope as thread_scope
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Puts the current thread to sleep for at least the specified amount of time.",
    @sleep as thread_sleep
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Spawns a new thread, returning a [`ThreadJoinHandle`] for it.",
    @spawn as thread_spawn
}
reexport! { rust: std::thread, local_module: "work",
    doc: "Cooperatively gives up a timeslice to the OS scheduler.",
    @yield_now as thread_yield_now
}
