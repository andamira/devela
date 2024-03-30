// devela::exec::async::noop
//
//!
//

use crate::exec::{TaskRawWaker, TaskRawWakerVTable, TaskWaker};

/// A dummy `TaskWaker` that does nothing when woken.
//
// WAIT: [task::Waker::noop](https://github.com/rust-lang/rust/pull/98286)
// (it will be const and returned as a &'static)
pub struct TaskWakerNoop;

impl TaskWakerNoop {
    /// Creates a new `Waker` that does nothing when `wake` is called.
    #[inline]
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> TaskWaker {
        const RAW_WAKER: TaskRawWaker = TaskRawWaker::new(core::ptr::null(), &VTABLE);
        const VTABLE: TaskRawWakerVTable = TaskRawWakerVTable::new(
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
        unsafe { TaskWaker::from_raw(RAW_WAKER) }
    }
}
