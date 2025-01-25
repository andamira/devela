// devela::work::process::thread::reexports
//
//! Reexported items.
//
// Note that std's standalone functions are namespaced in `ExtThread`.

use crate::reexport;

/* types */

reexport! { rust: std::thread,
    tag: crate::TAG_ERROR!(),
    doc: "An error returned by [`ThreadLocalKey::try_with`].",
    @AccessError as ThreadAccessError
}
reexport! { rust: std::thread,
    doc: "Thread factory, which can be used to configure the properties of a new thread.",
    @Builder as ThreadBuilder
}
reexport! { rust: std::thread,
    doc: "An owned permission to join on a thread (block on its termination).",
    @JoinHandle as ThreadJoinHandle
}
reexport! { rust: std::thread,
    doc: "A thread local storage key which owns its contents.",
    @LocalKey as ThreadLocalKey
}
reexport! { rust: std::thread,
    doc: "A scope to spawn scoped threads in.",
    @Scope as ThreadScope
}
reexport! { rust: std::thread,
    doc: "An owned permission to join on a scoped thread (block on its termination).",
    @ScopedJoinHandle as ThreadScopedJoinHandle
}
reexport! { rust: std::thread,
    doc: "A handle to a thread\n\n.
See also the [`ExtThread`][crate::ExtThread] trait.",
    Thread
}
reexport! { rust: std::thread,
    doc: "A unique identifier for a running thread.",
    ThreadId
}

/* macros */

reexport! { rust: std,
    doc: "Declare a new thread local storage key of type [`ThreadLocalKey`].",
    thread_local
}

/* aliases */

reexport! { rust: std::thread,
    tag: crate::TAG_RESULT!(),
    doc: "A specialized [`Result`] type for threads.",
    @Result as ThreadResult
}
