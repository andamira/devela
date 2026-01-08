// devela::work::sync::spin_lock
//
//! Defines the [`SpinLock`] and [`SpinLockGuard`] structs for simple mutual exclusion.
//

use crate::{
    AtomicBool, AtomicOrdering, Debug, Deref, DerefMut, FmtResult, Formatter, UnsafeCell,
    any_type_name, spin_loop,
};
// use crate::SleepSpin; // WIP
#[cfg(feature = "std")]
use crate::{Thread, ThreadExt};

#[doc = crate::_tags!(concurrency)]
/// A spinlock providing mutual exclusion without blocking.
#[doc = crate::_doc_location!("work/sync")]
///
/// Uses an atomic flag for synchronization, with a configurable backoff strategy.
///
/// # Examples
/// ```
/// # use devela::SpinLock;
/// let lock = SpinLock::<i32, 5, 10, 100>::new(42);
///
/// let mut guard = lock.lock(); // Acquire the lock
/// *guard += 1; // Modify the locked value
///
/// if let Some(mut guard) = lock.try_lock() {
///     *guard *= 2; // Modify only if lock was acquired
/// }
/// // Lock is automatically released when `guard` goes out of scope
/// ```
#[derive(Default)]
pub struct SpinLock<T, const SPIN: usize = 5, const YIELD: usize = 10, const SLEEP: u64 = 100> {
    /// The protected value, wrapped in an `UnsafeCell` for interior mutability.
    value: UnsafeCell<T>,
    /// Atomic flag indicating whether the lock is held (`true`).
    lock: AtomicBool,
}

// SAFETY: `SpinLock` can be safely sent across threads,
// because it does not impose any additional restrictions beyond `T: Send`.
unsafe impl<T> Send for SpinLock<T> where T: Send {}
// SAFETY: `SpinLock` ensures exclusive access via atomic locking,
// making it safe for shared references across threads as long as `T: Send`.
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> Debug
    for SpinLock<T, SPIN, YIELD, SLEEP>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        let locked = self.lock.load(AtomicOrdering::Acquire);
        f.debug_struct("SpinLock")
            .field("type", &any_type_name::<T>())
            .field("locked", &locked)
            .finish()
    }
}

#[rustfmt::skip]
impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> SpinLock<T, SPIN, YIELD, SLEEP> {
    /// Creates a new spinlock with the given value.
    pub const fn new(value: T) -> Self {
        SpinLock { value: UnsafeCell::new(value), lock: AtomicBool::new(false) }
    }

    /// Acquires the lock, spinning until it is available.
    ///
    /// This method employs an **adaptive backoff strategy** to minimize CPU contention:
    /// - Spins ([`spin_loop`]`()`) for `SPIN` iterations to avoid unnecessary thread switching.
    /// - Yields ([`ThreadExt::yield_now`]`()`) for the next `YIELD - SPIN` iterations,
    ///   allowing other threads to progress.
    /// - Sleeps ([`ThreadExt::sleep_ns`]`(SLEEP)`) if `SLEEP > 0`,
    ///   reducing CPU load under high contention.
    ///
    /// ## Environment
    /// - In `std` environments, the full backoff strategy is used.
    /// - In `no_std` environments, only `spin_loop()` is available.
    ///
    /// ## Compile-Time Optimization
    /// Unnecessary branches are removed at compile time if their respective constant is set to 0.
    pub fn lock(&self) -> SpinLockGuard<'_, T, SPIN, YIELD, SLEEP> {
        #[cfg(feature = "std")]
        let mut spin = 0usize;
        while self.lock.compare_exchange_weak(false, true,
            AtomicOrdering::Acquire, AtomicOrdering::Acquire).is_err() {
            #[cfg(feature = "std")]
            {
                if spin < SPIN { spin_loop(); }
                else if spin < YIELD { Thread::yield_now(); }
                else if SLEEP > 0 { Thread::sleep_ns(SLEEP); }
                spin += 1;
            }
            #[cfg(not(feature = "std"))]
            { spin_loop(); }
        }
        // let mut sleep_spin = SleepSpin::<SPIN, YIELD, SLEEP>::new();
        // while self
        //     .lock
        //     .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Acquire)
        //     .is_err()
        // {
        //     sleep_spin.spin();
        // }
        SpinLockGuard(self)
    }

    /// Attempts to acquire the lock without blocking.
    ///
    /// Returns `Some(SpinLockGuard<T>)` if successful, otherwise `None`.
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T, SPIN, YIELD, SLEEP>> {
        self.lock.compare_exchange(false, true, AtomicOrdering::Acquire, AtomicOrdering::Acquire)
            .is_ok().then(|| SpinLockGuard(self))
    }

    /// Checks if the lock is currently held.
    pub fn is_locked(&self) -> bool { self.lock.load(AtomicOrdering::Acquire) }

    /// Consumes the lock and returns the inner value.
    pub fn into_inner(self) -> T { self.value.into_inner() }

    /// Tries to consume the lock and return the inner value.
    ///
    /// If the lock is currently **unlocked**, this method returns `Some(T)`.
    /// Otherwise, it returns `None` without blocking or modifying the lock.
    pub fn try_into_inner(&self) -> Option<T> {
        // SAFETY: Safe because we check the lock
        (!self.is_locked()).then(|| unsafe { self.value.get().read() })
    }

    // /// Provides an unsafe reference to the value without locking.
    // ///
    // /// # Safety
    // /// - The caller must ensure no other thread is modifying the value simultaneously.
    // /// - This breaks Rust's aliasing rules if misused.
    // pub unsafe fn unsafe_as_ref(&self) -> &T { unsafe { &*self.value.get() } }

    /// Forces the lock to be released without dropping the guard.
    ///
    /// # Safety
    /// - This is a **manual unlocking mechanism**. If used incorrectly,
    ///   it may allow multiple threads to access `T` simultaneously.
    #[cfg(debug_assertions)]
    #[cfg_attr(nightly_doc, doc(cfg(debug_assertions)))]
    pub unsafe fn debug_force_unlock(&self) { self.lock.store(false, AtomicOrdering::SeqCst); }
}

/// A guard that grants exclusive access to a [`SpinLock`] value.
///
/// ## Guarantees
/// - If a `SpinLockGuard` exists, the lock **is held**.
/// - The protected value can be safely accessed through `Deref` and `DerefMut`.
/// - Dropping `SpinLockGuard` **releases the lock**, allowing other threads to acquire it.
pub struct SpinLockGuard<'a, T, const SPIN: usize, const YIELD: usize, const SLEEP: u64>(
    &'a SpinLock<T, SPIN, YIELD, SLEEP>,
);

// impl<'a, T, const SPIN: usize, const YIELD: usize, const SLEEP: u64>
//     SpinLockGuard<'a, *mut T, SPIN, YIELD, SLEEP>
// {
//     /// Converts a `SpinLockGuard<*mut T>` into `SpinLockGuard<&T>`.
//     ///
//     /// # Safety
//     /// - The caller must guarantee that `T` is not aliased elsewhere.
//     /// - `transmute` is used to reinterpret the pointer type, which can cause UB if misused.
//     pub const unsafe fn as_ref(self) -> SpinLockGuard<'a, &'a T, SPIN, YIELD, SLEEP> {
//         unsafe { transmute(self) }
//     }
// }

impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> Debug
    for SpinLockGuard<'_, T, SPIN, YIELD, SLEEP>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> {
        f.debug_tuple("SpinLockGuard").field(&any_type_name::<T>()).finish()
    }
}
impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> Drop
    for SpinLockGuard<'_, T, SPIN, YIELD, SLEEP>
{
    /// Releases the lock when the guard is dropped.
    fn drop(&mut self) {
        self.0.lock.store(false, AtomicOrdering::Release);
    }
}
impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> Deref
    for SpinLockGuard<'_, T, SPIN, YIELD, SLEEP>
{
    type Target = T;
    /// Allows access to the locked value.
    fn deref(&self) -> &Self::Target {
        // SAFETY: The lock is held, ensuring exclusive access to `T`.
        unsafe { &*self.0.value.get() }
    }
}
impl<T, const SPIN: usize, const YIELD: usize, const SLEEP: u64> DerefMut
    for SpinLockGuard<'_, T, SPIN, YIELD, SLEEP>
{
    /// Allows exclusive access to the locked value.
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The lock is held, ensuring exclusive access to `T`.
        unsafe { &mut *self.0.value.get() }
    }
}
