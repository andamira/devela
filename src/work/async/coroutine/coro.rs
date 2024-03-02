// devela::work::async::coroutine
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

use crate::{
    _deps::alloc::{boxed::Box, collections::VecDeque},
    work::{TaskContext, TaskPoll, TaskWakerNoop},
};
use core::{future::Future, pin::Pin};

/* coroutine */

/// Represents a single-threaded coroutine.
///
/// It has a private state that can be either running or halted.
pub struct Coro {
    state: CoroState,
}

impl Coro {
    /// Returns a `CoroYield` object, which is a future that can be awaited on.
    pub fn waiter(&mut self) -> CoroYield<'_> {
        CoroYield { coroutine: self }
    }
}

// Private Coro state.
enum CoroState {
    Halted,
    Running,
}

/// A future that alternates between [`Ready`][TaskPoll::Ready] and
/// [`Pending`][TaskPoll::Pending] states each time it's polled.
///
/// This allows the coroutine to yield control back to its `CoroRunner`
/// and be resumed later.
pub struct CoroYield<'a> {
    coroutine: &'a mut Coro,
}
impl<'a> Future for CoroYield<'a> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut TaskContext) -> TaskPoll<Self::Output> {
        match self.coroutine.state {
            CoroState::Halted => {
                self.coroutine.state = CoroState::Running;
                TaskPoll::Ready(())
            }
            CoroState::Running => {
                self.coroutine.state = CoroState::Halted;
                TaskPoll::Pending
            }
        }
    }
}

/// A [`Coro`] runner responsible for managing and executing the coroutines.
///
/// It maintains a queue of coroutines and runs them in a loop until they
/// are all complete. When a coroutine is polled and returns TaskPoll::Pending, it is put back
/// into the queue to be run again later. If it returns TaskPoll::Ready, it is
/// considered complete and is not put back into the queue.
pub struct CoroRunner {
    coroutines: VecDeque<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Default for CoroRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl CoroRunner {
    /// Returns a new executor.
    pub fn new() -> Self {
        CoroRunner {
            coroutines: VecDeque::new(),
        }
    }

    /// Adds a task to the executor in the form of a closure.
    pub fn push<C, F>(&mut self, closure: C)
    where
        F: Future<Output = ()> + 'static,
        C: FnOnce(Coro) -> F,
    {
        let coroutine = Coro {
            state: CoroState::Running,
        };
        self.coroutines.push_back(Box::pin(closure(coroutine)));
    }

    /// Runs the executor.
    pub fn run(&mut self) {
        let waker = TaskWakerNoop::new();
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
