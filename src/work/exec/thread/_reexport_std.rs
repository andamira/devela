// devela::work::exec::thread::_reexport_std
//
//!
//
// Note that std's standalone functions are namespaced in `devela::ThreadExt`.

use crate::{_reexport, _tags};

/* types */

_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency error),
    doc: "An error returned by [`ThreadLocalKey::try_with`].",
    @AccessError as ThreadAccessError
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(construction concurrency),
    doc: "Thread factory, which can be used to configure the properties of a new thread.",
    @Builder as ThreadBuilder
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency),
    doc: "An owned permission to join on a thread (block on its termination).",
    @JoinHandle as ThreadJoinHandle
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency),
    doc: "A thread local storage key which owns its contents.",
    @LocalKey as ThreadLocalKey
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency),
    doc: "A scope to spawn scoped threads in.",
    @Scope as ThreadScope
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency),
    doc: "An owned permission to join on a scoped thread (block on its termination).",
    @ScopedJoinHandle as ThreadScopedJoinHandle
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency uid),
    doc: "A handle to a thread.",
    +doc: "See also the [`ThreadExt`][crate::ThreadExt] trait.",
    Thread
}
_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency uid),
    doc: "A unique identifier for a running thread.",
    ThreadId
}

/* macros */

_reexport! { rust: std, location: "work/thread", tag: _tags!(concurrency code),
    doc: "Declare a new thread local storage key of type [`ThreadLocalKey`].",
    thread_local
}

/* aliases */

_reexport! { rust: std::thread, location: "work/thread", tag: _tags!(concurrency result),
    doc: "A specialized [`Result`] type for threads.",
    @Result as ThreadResult
}
