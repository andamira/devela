// devela::exec::async::waker
//
//!
//

/// Creates a new `Waker` that does nothing when `wake` is called.
// WAIT: [task::Waker::noop](https://github.com/rust-lang/rust/pull/98286)
#[inline]
#[must_use]
#[cfg(all(not(feature = "safe_exec"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
pub fn task_waker_noop() -> super::TaskWaker {
    const RAW_WAKER: super::TaskRawWaker = super::TaskRawWaker::new(core::ptr::null(), &VTABLE);
    const VTABLE: super::TaskRawWakerVTable = super::TaskRawWakerVTable::new(
        // Cloning just returns a new no-op raw waker
        |_| RAW_WAKER,
        // `wake` does nothing
        |_| {},
        // `wake_by_ref` does nothing
        |_| {},
        // Dropping does nothing as we don't allocate anything
        |_| {},
    );
    // SAFETY: The waker points to a vtable with functions that do nothing.
    unsafe { super::TaskWaker::from_raw(RAW_WAKER) }
}
