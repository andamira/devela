use crate::{_reexport, _tags};

pub use {
    exec_process::*, exec_thread::*, future::*, sync::*, sync_atomic::*, sync_mpsc::*, task_coro::*,
};

// Note that std's standalone functions are namespaced in `devela::ProcessExt`.
//
// WAIT: [exit_status_error](https://github.com/rust-lang/rust/issues/84908)
pub(crate) mod exec_process {
    use super::*;

    /* traits */

    _reexport! { rust: std::process,
        location: "work/exec/process" => trait ProcessTermination, tag: _tags!(platform runtime),
        doc: "A trait for implementing arbitrary return types in the `main` function.",
        @Termination as ProcessTermination
    }

    /* types */

    _reexport! { rust: std::process,
        location: "work/exec/process" => struct Process, tag: _tags!(concurrency),
        doc: "Representation of a running or exited child process.",
        +doc: "See also the [`ProcessExt`][crate::ProcessExt] trait.",
        @Child as Process
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct ProcessStderr, tag: _tags!(io),
        doc: "A handle to a child process's stderr.",
        @ChildStderr as ProcessStderr
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct ProcessStdin, tag: _tags!(io),
        doc: "A handle to a child process's standard input (stdin).",
        @ChildStdin as ProcessStdin
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct ProcessStdout, tag: _tags!(io),
        doc: "A handle to a child process's standard output (stdout).",
        @ChildStdout as ProcessStdout
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct Command, tag: _tags!(platform runtime),
        doc: "A builder for configuring and spawning new processes.",
        Command
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct CommandArgs, tag: _tags!(platform runtime),
        doc: "An iterator over the command arguments.",
        CommandArgs
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct CommandEnvs, tag: _tags!(platform runtime),
        doc: "An iterator over the command environment variables.",
        CommandEnvs
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct ExitCode, tag: _tags!(platform runtime result),
        doc: "The status code the process returns to its parent on normal termination.",
        ExitCode
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct ExitStatus, tag: _tags!(platform runtime result),
        doc: "Describes the result of a process after it has terminated.",
        ExitStatus
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct Output, tag: _tags!(platform runtime),
        doc: "The output of a finished process.",
        Output
    }
    _reexport! { rust: std::process,
        location: "work/exec/process" => struct Stdio, tag:  _tags!(platform runtime io),
        doc: "Specifies how to handle standard I/O streams in [`Command`].",
        Stdio
    }
}
// Note that std's standalone functions are namespaced in `devela::ThreadExt`.
pub(crate) mod exec_thread {
    use super::*;

    /* structs */

    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadAccessError, tag: _tags!(concurrency error),
        doc: "An error returned by [`ThreadLocalKey::try_with`].",
        @AccessError as ThreadAccessError
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadBuilder, tag: _tags!(construction concurrency),
        doc: "Thread factory, which can be used to configure the properties of a new thread.",
        @Builder as ThreadBuilder
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadJoinHandle, tag: _tags!(concurrency),
        doc: "An owned permission to join on a thread (block on its termination).",
        @JoinHandle as ThreadJoinHandle
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadLocalKey, tag: _tags!(concurrency),
        doc: "A thread local storage key which owns its contents.",
        @LocalKey as ThreadLocalKey
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadScope, tag: _tags!(concurrency),
        doc: "A scope to spawn scoped threads in.",
        @Scope as ThreadScope
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadScopedJoinHandle, tag: _tags!(concurrency),
        doc: "An owned permission to join on a scoped thread (block on its termination).",
        @ScopedJoinHandle as ThreadScopedJoinHandle
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct Thread, tag: _tags!(concurrency id),
        doc: "A handle to a thread.",
        +doc: "See also the [`ThreadExt`][crate::ThreadExt] trait.",
        Thread
    }
    _reexport! { rust: std::thread,
        location: "work/exec/thread" => struct ThreadId, tag: _tags!(concurrency id),
        doc: "A unique identifier for a running thread.",
        ThreadId
    }

    /* macros */

    _reexport! { rust: std,
        location: "work/exec/thread" => macro thread_local, tag: _tags!(concurrency code),
        doc: "Declare a new thread local storage key of type [`ThreadLocalKey`].",
        thread_local
    }

    /* aliases */

    _reexport! { rust: std::thread,
        location: "work/exec/thread" => type ThreadResult, tag: _tags!(concurrency result),
        doc: "A specialized [`Result`] type for threads.",
        @Result as ThreadResult
    }
}

// WAIT: [future_join](https://github.com/rust-lang/rust/issues/91642)
// WAIT: [async_drop](https://github.com/rust-lang/rust/issues/126482)
pub(crate) mod future {
    use super::*;

    /* `core` future */

    _reexport! { rust: core::future,
        location: "work/future" => struct FuturePending, tag: _tags!(concurrency runtime),
        doc: "A future which never resolves, representing a never finishing computation.",
        @Pending as FuturePending
    }
    _reexport! { rust: core::future,
        location: "work/future" => struct FuturePollFn, tag: _tags!(concurrency runtime),
        doc: "A Future that wraps a function returning [`AsyncPoll`].",
        @PollFn as FuturePollFn
    }
    _reexport! { rust: core::future,
        location: "work/future" => struct FutureReady, tag: _tags!(concurrency runtime),
        doc: "A future that is immediately ready with a value.",
        @Ready as FutureReady
    }
    _reexport! { rust: core::future,
        location: "work/future" => trait Future, tag: _tags!(concurrency runtime),
        doc: "An asynchronous computation obtained by use of
            [`async`](https://doc.rust-lang.org/std/keyword.async.html).",
        Future
    }
    _reexport! { rust: core::future,
        location: "work/future" => trait IntoFuture, tag: _tags!(value concurrency runtime),
        doc: "Conversion into a `Future`.",
        IntoFuture
    }

    /* `core` task */

    _reexport! { rust: core::task,
        location: "work/future" => struct AsyncContext, tag: _tags!(concurrency runtime),
        doc: "The context of an asynchronous task.",
        @Context as AsyncContext
    }
    _reexport! { rust: core::task,
        location: "work/future" => struct AsyncRawWaker, tag: _tags!(concurrency runtime),
        doc: "Allows the implementor of a task executor to create a [`AsyncWaker`].",
        @RawWaker as AsyncRawWaker
    }
    _reexport! { rust: core::task,
        location: "work/future" => struct AsyncRawWakerVTable, tag: _tags!(concurrency runtime),
        doc: "A virtual fn pointer table that specifies the behavior of a [`AsyncRawWaker`].",
        @RawWakerVTable as AsyncRawWakerVTable
    }
    _reexport! { rust: core::task,
        location: "work/future" => struct AsyncWaker, tag: _tags!(concurrency runtime),
        doc: "A handle for waking up a task by notifying its executor that it's ready to run.",
        @Waker as AsyncWaker
    }
    _reexport! { rust: core::task,
        location: "work/future" => enum AsyncPoll, tag: _tags!(concurrency runtime),
        doc: "Indicates whether a value is ready or if the current task is still pending.",
        @Poll as AsyncPoll
    }
    _reexport! { rust: core::task,
        location: "work/future" => macro async_ready, tag: _tags!(concurrency runtime),
        doc: "Extracts the successful type of a [`AsyncPoll<T>`].",
        @ready as async_ready
    }

    /* from either `alloc` or `portable-atomic-util` */

    #[doc = _tags!(concurrency atomic runtime atomic_alloc_portable_util)]
    #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    /// The implementation of waking a task on an executor.
    #[doc = crate::_doc_meta!{
        location("work/future", trait AsyncWake),
    }]
    #[doc = "---\n\n---\n\n"]
    pub use crate::_dep::portable_atomic_util::task::Wake as AsyncWake;
    //
    #[doc = _tags!(concurrency atomic runtime atomic_alloc_portable_util)]
    #[doc = crate::_doc_meta!{
        location("work/future", trait AsyncWake),
    }]
    #[doc = "---\n\n---\n\n"]
    #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::_alloc::task::Wake as AsyncWake;
}

// WAIT: [unique_rc_arc](https://github.com/rust-lang/rust/issues/112566)
pub(crate) mod sync {
    use super::*;

    /* from either `alloc` or `portable-atomic-util` */

    #[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
    /// A thread-safe reference-counting pointer.
    #[doc = crate::_doc_meta!{
        location("work/sync", struct Arc),
    }]
    #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::portable_atomic_util::Arc;
    //
    #[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
    /// A thread-safe reference-counting pointer.
    #[doc = crate::_doc_meta!{
        location("work/sync", struct Arc),
    }]
    #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use alloc::sync::Arc;

    #[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
    /// A version of [`Arc`] that holds a non-owning reference.
    #[doc = crate::_doc_meta!{
        location("work/sync", struct ArcWeak),
    }]
    #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use crate::_dep::portable_atomic_util::Weak as ArcWeak;

    #[doc = _tags!(concurrency atomic lifetime atomic_alloc_portable_util)]
    /// A version of [`Arc`] that holds a non-owning reference.
    #[doc = crate::_doc_meta!{
        location("work/sync", struct ArcWeak),
    }]
    #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    pub use alloc::sync::Weak as ArcWeak;

    /* `std` structs */

    _reexport! { rust: std::sync,
        location: "work/sync" => struct Barrier, tag: _tags!(concurrency),
        doc: "Enables multiple threads to synchronize the beginning of some computation.",
        Barrier
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct BarrierWaitResult, tag: _tags!(concurrency result),
        doc: "Returned by [`Barrier::wait()`] when all threads in it have rendezvoused.",
        BarrierWaitResult
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct Condvar, tag: _tags!(concurrency),
        doc: "A Condition Variable.",
        Condvar
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct LazyLock, tag: _tags!(concurrency init),
        doc: "A value which is initialized on the first access.",
        LazyLock
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct Mutex, tag: _tags!(concurrency),
        doc: "A mutual exclusion primitive useful for protecting shared data.",
        Mutex
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct MutexGuard, tag: _tags!(concurrency guard),
        doc: "An RAII implementation of a \"scoped lock\" of a mutex.",
        MutexGuard
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct Once, tag: _tags!(concurrency),
        doc: "A synchronization primitive for one-time global initialization.",
        Once
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct OnceLock, tag: _tags!(concurrency),
        doc: "A synchronization primitive which can be written to only once.",
        OnceLock
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct OnceState, tag: _tags!(concurrency),
        doc: "State yielded to [`Once::call_once_force()`]'s closure parameter.",
        OnceState
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct PoisonError, tag: _tags!(concurrency error),
        doc: "A type of error which can be returned whenever a lock is acquired.",
        PoisonError
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct RwLock, tag: _tags!(concurrency),
        doc: "A reader-writer lock",
        RwLock
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct RwLockReadGuard, tag: _tags!(concurrency),
        doc: "RAII structure used to release the shared read access of a lock when dropped.",
        RwLockReadGuard
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct RwLockWriteGuard, tag: _tags!(concurrency),
        doc: "RAII structure used to release the exclusive write access of a lock when dropped.",
        RwLockWriteGuard
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => struct WaitTimeoutResult, tag: _tags!(concurrency result),
        doc: "Whether a timed wait on a [`Condvar`] returned due to a time out or not.",
        WaitTimeoutResult
    }

    /* `std` enums */

    _reexport! { rust: std::sync,
        location: "work/sync" => enum TryLockError, tag: _tags!(concurrency error_composite),
        doc: "An enumeration of possible errors associated with a [`TryLockResult`].",
        TryLockError
    }

    /* `std` aliases */

    _reexport! { rust: std::sync,
        location: "work/sync" => type LockResult, tag: _tags!(concurrency result),
        doc: "A type alias for the result of a lock method which can be poisoned.",
        LockResult
    }
    _reexport! { rust: std::sync,
        location: "work/sync" => type TryLockResult, tag: _tags!(concurrency result),
        doc: "A type alias for the result of a nonblocking locking method.",
        TryLockResult
    }
}

// Apart from the Rust library items, it also re-exports from `portable-atomic`,
// and the [`Atomic`] type from the `atomic` crate.
pub(crate) mod sync_atomic {
    use super::*;
    #[cfg(feature = "dep_portable_atomic")]
    use crate::_DOC_ATOMIC_CORE_PORTABLE;

    /* enums */

    _reexport! { rust: core::sync::atomic,
        location: "work/sync/atomic" => enum AtomicOrdering, tag: _tags!(concurrency atomic ord),
        doc: "Atomic memory ordering.",
        @Ordering as AtomicOrdering
    }

    /* functions */

    _reexport! { rust: core::sync::atomic,
        location: "work/sync/atomic" => fn atomic_fence, tag: _tags!(concurrency atomic),
        doc: "An atomic fence.",
        @fence as atomic_fence
    }
    _reexport! { rust: core::sync::atomic,
        location: "work/sync/atomic" => fn atomic_compiler_fence, tag: _tags!(concurrency atomic),
        doc: "A compiler memory fence.",
        @compiler_fence as atomic_compiler_fence
    }

    /* from the `atomic` crate */

    _reexport! { "dep_atomic", "atomic", atomic,
        location: "work/sync/atomic" => struct Atomic,
        tag: _tags!(concurrency atomic),
        doc: "A generic atomic wrapper type.",
        Atomic
    }

    /* from `portable-atomic` */

    _reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
        location: "work/sync/atomic" => struct AtomicF32,
        tag: _tags!(concurrency atomic),
        doc: "A thread-safe floating-point type.",
        AtomicF32,
    }
    _reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
        location: "work/sync/atomic" => struct AtomicF64,
        tag: _tags!(concurrency atomic),
        doc: "A thread-safe floating-point type.",
        AtomicF64
    }
    _reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
        location: "work/sync/atomic" => struct AtomicI128,
        tag: _tags!(concurrency atomic),
        doc: "A thread-safe signed integer type.",
        AtomicI128
    }
    _reexport! { "dep_portable_atomic", "portable-atomic", portable_atomic,
        location: "work/sync/atomic" => struct AtomicU128,
        tag: _tags!(concurrency atomic),
        doc: "A thread-safe unsigned integer type.",
        AtomicU128
    }

    /* from either `portable-atomic` or `core` */

    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A thread-safe signed integer type.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic"), // multiple imports
    }]
    #[doc = _DOC_ATOMIC_CORE_PORTABLE!()]
    #[cfg(feature = "dep_portable_atomic")]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("dep_portable_atomic")))))]
    pub use crate::_dep::portable_atomic::{
        AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize,
    };

    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A thread-safe signed integer type.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic"), // multiple imports
    }]
    #[doc = _DOC_ATOMIC_CORE_PORTABLE!()]
    #[cfg(feature = "dep_portable_atomic")]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("dep_portable_atomic")))))]
    pub use crate::_dep::portable_atomic::{
        AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
    };

    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "8"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "8")))]
    pub use core::sync::atomic::{AtomicI8, AtomicU8};
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "16"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "16")))]
    pub use core::sync::atomic::{AtomicI16, AtomicU16};
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "32"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "32")))]
    pub use core::sync::atomic::{AtomicI32, AtomicU32};
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "64"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "64")))]
    pub use core::sync::atomic::{AtomicI64, AtomicU64};
    //
    // WAIT: [integer_atomics](https://github.com/rust-lang/rust/issues/99069)
    // #[doc = _tags!(concurrency atomic atomic_core_portable)]
    // #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "128"))]
    // #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "128")))]
    // pub use core::sync::atomic::{AtomicI128, AtomicU128};
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "ptr")))]
    pub use core::sync::atomic::{AtomicIsize, AtomicUsize};

    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A raw pointer type which can be safely shared between threads.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic", struct AtomicPtr),
    }]
    #[cfg(feature = "dep_portable_atomic")]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("dep_portable_atomic")))))]
    pub use crate::_dep::portable_atomic::AtomicPtr;
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A raw pointer type which can be safely shared between threads.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic", struct AtomicPtr),
    }]
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "ptr"))]
    #[cfg_attr(nightly_doc, doc(cfg(target_has_atomic = "ptr")))]
    pub use core::sync::atomic::AtomicPtr;

    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A boolean type which can be safely shared between threads.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic", struct AtomicBool),
    }]
    #[cfg(feature = "dep_portable_atomic")]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("dep_portable_atomic")))))]
    pub use crate::_dep::portable_atomic::AtomicBool;
    //
    #[doc = _tags!(concurrency atomic atomic_core_portable)]
    /// A boolean type which can be safely shared between threads.
    #[doc = crate::_doc_meta!{
        location("work/sync/atomic", struct AtomicBool),
    }]
    // #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic_load_store = "8"))] // WAIT
    #[cfg(all(not(feature = "dep_portable_atomic"), target_has_atomic = "8"))]
    #[cfg_attr(nightly_doc, doc(auto_cfg(hide(feature, values("dep_portable_atomic")))))]
    pub use core::sync::atomic::AtomicBool;
}
pub(crate) mod sync_mpsc {
    use super::*;

    /* structs */

    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscIntoIter,
        tag: _tags!(concurrency iterator),
        doc: "An owning iterator over messages on an [`MpscReceiver`].",
        @IntoIter as MpscIntoIter
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscIter,
        tag: _tags!(concurrency iterator),
        doc: "An iterator over messages on an [`MpscReceiver`]",
        @Iter as MpscIter
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscReceiver,
        tag: _tags!(concurrency),
        doc: "The receiving half of an [`Mpsc`][crate::Mpsc]
        [`channel`][crate::Mpsc::channel] or [`sync_channel`][crate::Mpsc::sync_channel].",
        @Receiver as MpscReceiver
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscReceiveError,
        tag: _tags!(concurrency error),
        doc: "An error returned from [`MpscReceiver::recv`].",
        @RecvError as MpscReceiveError
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscSendError,
        tag: _tags!(concurrency error),
        doc: "An error returned from [`MpscSender::send`] or [`MpscSyncSender::send`].",
        @SendError as MpscSendError
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscSender,
        tag: _tags!(concurrency),
        doc: "The sending half of an *async* `Mpsc` channel.",
        @Sender as MpscSender
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscSyncSender,
        tag: _tags!(concurrency),
        doc: "The sending half of a *sync* `Mpsc` channel.",
        @SyncSender as MpscSyncSender
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => struct MpscTryIter,
        tag: _tags!(concurrency iterator),
        doc: "Attempts to yield all pending values for an [`MpscReceiver`].",
        @TryIter as MpscTryIter
    }

    /* enums */

    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => enum MpscRecvTimeoutError,
        tag: _tags!(concurrency error_composite),
        doc: "Possible errors that made [`recv_timeout`][MpscReceiver::recv_timeout]
    unable to return data.",
        @RecvTimeoutError as MpscRecvTimeoutError
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => enum MpscTryRecvError,
        tag: _tags!(concurrency error_composite),
        doc: "Possible reasons that [`try_recv`][MpscReceiver::try_recv]
    could not return data when called.",
        @TryRecvError as MpscTryRecvError
    }
    _reexport! { rust: std::sync::mpsc,
        location: "work/sync/mpsc" => enum MpscTrySendError,
        tag: _tags!(concurrency error_composite),
        doc: "Possible error outcomes for the [`try_send`][MpscSyncSender::try_send] method.",
        @TrySendError as MpscTrySendError
    }
}

pub(crate) mod task_coro {
    use super::*;

    _reexport! { rust: core::ops,
        extra_flags:(nightly_coro),
        location: "work/task/coro" => trait Coroutine,
        tag: _tags!(concurrency runtime),
        doc: "The trait implemented by builtin coroutine types.",
        Coroutine
    }
    _reexport! { rust: core::ops,
        extra_flags:(nightly_coro),
        location: "work/task/coro" => struct CoroutineState,
        tag: _tags!(concurrency runtime result),
        doc: "The result of a coroutine resumption.",
        CoroutineState
    }
}
