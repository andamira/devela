// devela::work::thread::ext
//
//! Defines the [`ExtThread`] trait.
//

use crate::{Duration, Thread, ThreadJoinHandle, ThreadScope};
use std::thread::{
    available_parallelism, current, panicking, park, park_timeout, scope, sleep, spawn, yield_now,
};

/// Marker trait to prevent downstream implementations of the [`ExtThread`] trait.
trait Sealed {}
impl Sealed for Thread {}

#[doc = crate::_TAG_NAMESPACE!()]
/// Extension trait providing additional methods for [`Thread`]s.
///
/// It offers the standalone functions in `std::thread` as associated methods.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtThread: Sealed {
    /// Gets a handle to the thread that invokes it.
    ///
    /// See `std::thread::`[current].
    #[must_use]
    fn current() -> Thread { current() }

    /// Determines whether the current thread is unwinding because of panic.
    ///
    /// See `std::thread::`[panicking].
    #[must_use]
    fn panicking() -> bool { panicking() }

    /// Returns an estimate of the default amount of parallelism a program should use.
    ///
    /// See `std::thread::`[available_parallelism].
    fn available_parallelism() ->  Result<usize, crate::IoError> {
        available_parallelism().map(|n|n.get())
    }

    /// Blocks unless or until the current thread’s token is made available.
    ///
    /// See `std::thread::`[park].
    fn park() { park() }

    /// Blocks unless or until the current thread’s token is made available
    /// or the specified duration has been reached (may wake spuriously).
    ///
    /// See `std::thread::`[park_timeout].
    fn park_timeout(duration: Duration) { park_timeout(duration) }

    /// Create a scope for spawning scoped threads.
    ///
    /// See `std::thread::`[scope].
    fn scope<'env, F, T>(f: F) -> T
    where F: for<'scope> FnOnce(&'scope ThreadScope<'scope, 'env>) -> T { scope(f) }

    /// Puts the current thread to sleep for at least the specified amount of time.
    ///
    /// See `std::thread::`[sleep].
    fn sleep(duration: Duration) { sleep(duration) }

    /// Puts the current thread to sleep for at least the specified amount of `milliseconds`.
    fn sleep_ms(milliseconds: u64) { sleep(Duration::from_millis(milliseconds)) }

    /// Puts the current thread to sleep for at least the specified amount of `nanoseconds`.
    fn sleep_us(microseconds: u64) { sleep(Duration::from_micros(microseconds)) }

    /// Puts the current thread to sleep for at least the specified amount of `nanoseconds`.
    fn sleep_ns(nanoseconds: u64) { sleep(Duration::from_nanos(nanoseconds)) }

    /// Spawns a new thread, returning a [`ThreadJoinHandle`] for it.
    ///
    /// See `std::thread::`[spawn].
    fn spawn<F, T>(f: F) -> ThreadJoinHandle<T>
    where F: FnOnce() -> T + Send + 'static, T: Send + 'static { spawn(f) }

    /// Cooperatively gives up a timeslice to the OS scheduler.
    ///
    /// See `std::thread::`[yield_now].
    fn yield_now() { yield_now() }
}
impl ExtThread for Thread {}
