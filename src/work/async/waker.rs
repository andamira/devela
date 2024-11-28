// devela::work::async::waker

/// Creates a new `Waker` that does nothing when `wake` is called.
// WAIT: [noop_waker](https://github.com/rust-lang/rust/pull/98286)
#[must_use]
#[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
pub const fn task_waker_noop() -> crate::TaskWaker {
    use crate::{Ptr, TaskRawWaker, TaskRawWakerVTable, TaskWaker};
    const RAW_WAKER: TaskRawWaker = TaskRawWaker::new(Ptr::null(), &VTABLE);
    const VTABLE: TaskRawWakerVTable = TaskRawWakerVTable::new(
        |_| RAW_WAKER, // cloning returns a new no-op raw waker
        |_| {},        // `wake` does nothing
        |_| {},        // `wake_by_ref` does nothing
        |_| {},        // dropping does nothing
    );
    // SAFETY: The waker points to a vtable with functions that do nothing.
    unsafe { TaskWaker::from_raw(RAW_WAKER) }
}
