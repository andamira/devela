// devela::task::coroutine
//
//! A minimal single-threaded coroutine implementation.
//!
//! This code demonstrates a basic cooperative multitasking system where tasks
//! can yield control back to the scheduler and be resumed later.
//!
//! This is the fundamental concept behind coroutines.
//!
//! This is based on:
//! - <https://blog.aloni.org/posts/a-stack-less-rust-coroutine-100-loc/>
//! - <https://www.reddit.com/r/rust/comments/etqwhx/a_stackless_rust_coroutine_library_under_100_loc/>
//

use super::{TaskContext, TaskPoll, TaskRawWaker, TaskRawWakerVTable, TaskWaker};
use _alloc::{boxed::Box, collections::VecDeque};
use core::{future::Future, pin::Pin};

/// Represents a single-threaded coroutine.
///
/// It has a private state that can be either running or halted.
pub struct LiteCoroutine {
    state: LiteCoroutineState,
}

impl LiteCoroutine {
    /// Returns a `LiteCoroutineWaiter` object, which is a future that can be awaited on.
    pub fn waiter(&mut self) -> LiteCoroutineWaiter<'_> {
        LiteCoroutineWaiter { coroutine: self }
    }
}

// Private LiteCoroutine state.
enum LiteCoroutineState {
    Halted,
    Running,
}

/// A future that alternates between [`Ready`][TaskPoll::Ready] and
/// [`Pending`][TaskPoll::Pending] states each time it's polled.
///
/// This allows the coroutine to yield control back to its `LiteCoroutineExecutor`
/// and be resumed later.
pub struct LiteCoroutineWaiter<'a> {
    coroutine: &'a mut LiteCoroutine,
}
impl<'a> Future for LiteCoroutineWaiter<'a> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut TaskContext) -> TaskPoll<Self::Output> {
        match self.coroutine.state {
            LiteCoroutineState::Halted => {
                self.coroutine.state = LiteCoroutineState::Running;
                TaskPoll::Ready(())
            }
            LiteCoroutineState::Running => {
                self.coroutine.state = LiteCoroutineState::Halted;
                TaskPoll::Pending
            }
        }
    }
}
/// An executor responsible for managing and executing the coroutines.
///
/// It maintains a queue of coroutines and runs them in a loop until they
/// are all complete. When a coroutine is polled and returns TaskPoll::Pending, it is put back
/// into the queue to be run again later. If it returns TaskPoll::Ready, it is
/// considered complete and is not put back into the queue.
pub struct LiteCoroutineExecutor {
    coroutines: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Default for LiteCoroutineExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl LiteCoroutineExecutor {
    /// Returns a new executor.
    pub fn new() -> Self {
        LiteCoroutineExecutor {
            coroutines: VecDeque::new(),
        }
    }

    /// Adds a task to the executor in the form of a closure.
    pub fn push<C, F>(&mut self, closure: C)
    where
        F: Future<Output = ()> + 'static,
        C: FnOnce(LiteCoroutine) -> F,
    {
        let coroutine = LiteCoroutine {
            state: LiteCoroutineState::Running,
        };
        self.coroutines.push_back(Box::pin(closure(coroutine)));
    }

    /// Runs the executor.
    pub fn run(&mut self) {
        let waker = LiteCoroutineWaker::new_waker();
        let mut context = TaskContext::from_waker(&waker);

        while let Some(mut coroutine) = self.coroutines.pop_front() {
            match coroutine.as_mut().poll(&mut context) {
                TaskPoll::Pending => {
                    self.coroutines.push_back(coroutine);
                }
                TaskPoll::Ready(()) => {}
            }
        }
    }
}

/// A dummy `TaskWaker` that does nothing when woken.
// This is used to create a context for polling the coroutines.
// The TaskContext is required by the `Future::poll` method, but since this is a
// single-threaded executor, the waker is not actually needed.
pub struct LiteCoroutineWaker;
impl LiteCoroutineWaker {
    fn new_waker() -> TaskWaker {
        // SAFETY: The waker points to a vtable with functions that do nothing.
        unsafe { TaskWaker::from_raw(Self::RAW_WAKER) }
    }
    const RAW_WAKER: TaskRawWaker = TaskRawWaker::new(core::ptr::null(), &Self::VTABLE);
    const VTABLE: TaskRawWakerVTable =
        TaskRawWakerVTable::new(Self::clone, Self::wake, Self::wake_by_ref, Self::drop);

    unsafe fn clone(_: *const ()) -> TaskRawWaker {
        Self::RAW_WAKER
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
}
