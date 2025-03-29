// devela::work::future::coroutine::coro
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

use crate::{serr, sok, Debug, Future, OptRes, Pin, TaskContext, TaskPoll};
#[cfg(feature = "alloc")]
use crate::{Box, TaskWaker, VecDeque};

/* coroutine */

/// Represents a single-thread stackless coroutine worker.
///
/// It has a private status that can be either running or halted.
#[derive(Clone, Copy, Debug)]
pub struct CoroWorker<T, E> {
    status: CoroWorkerStatus,
    result: OptRes<T, E>,
}

/// Private coroutine worker status.
#[derive(Clone, Copy, Debug)]
enum CoroWorkerStatus {
    Halted,
    Running,
}

impl<T, E> CoroWorker<T, E> {
    /// Returns a new coroutine worker.
    #[allow(unused)]
    const fn new() -> Self {
        CoroWorker { status: CoroWorkerStatus::Running, result: None }
    }

    /// Yields an [`Ok`] `value` and returns an awaitable `CoroWork`.
    pub fn yield_ok(&mut self, value: T) -> CoroWork<'_, T, E> {
        self.result = sok(value);
        CoroWork { cor: self }
    }

    /// Yields an [`Err`] and returns an awaitable `CoroWork`.
    pub fn yield_err(&mut self, error: E) -> CoroWork<'_, T, E> {
        self.result = serr(error);
        CoroWork { cor: self }
    }
}

/* yielder */

/// A future that alternates between [`Ready`][TaskPoll::Ready] and
/// [`Pending`][TaskPoll::Pending] status each time it's polled.
///
/// This allows the coroutine to yield control back and be resumed later.
pub struct CoroWork<'a, T, E> {
    cor: &'a mut CoroWorker<T, E>,
}

impl<T, E> Future for CoroWork<'_, T, E> {
    type Output = OptRes<T, E>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut TaskContext) -> TaskPoll<OptRes<T, E>> {
        match self.cor.status {
            CoroWorkerStatus::Halted => {
                self.cor.status = CoroWorkerStatus::Running;
                if let Some(result) = self.cor.result.take() {
                    match result {
                        Err(error) => TaskPoll::Ready(serr(error)),
                        Ok(value) => TaskPoll::Ready(sok(value)),
                    }
                } else {
                    unreachable!();
                }
            }
            CoroWorkerStatus::Running => {
                self.cor.status = CoroWorkerStatus::Halted;
                TaskPoll::Pending
            }
        }
    }
}

/* manager */

/// A managed dynamic collection of single-thread [`CoroWorker`] coroutines.
///
/// It maintains a queue of coroutines in the stack, and runs them in a loop
/// until they are all complete.
///
/// When a coroutine is polled and returns [`TaskPoll::Pending`], it is put back
/// into the queue to be run again later. If it returns [`TaskPoll::Ready`]
/// it is considered complete and is not put back into the queue.
///
/// # Examples
/// ```
#[doc = include_str!("../../../../examples/work/coro_manager.rs")]
/// ```
/// It outputs:
/// ```text
/// Running
/// > instance 1 NEW
/// > instance 2 NEW
/// > instance 3 NEW
/// > instance 4 NEW
///   instance 1 A.0 Ok('a'))
///   instance 2 A.0 Ok('a'))
///   instance 3 A.0 Ok('a'))
///   instance 1 B Ok('b')
///   instance 2 B Ok('b')
///   instance 3 B Ok('b')
///   instance 1 A.1 Ok('a'))
///   instance 2 A.1 Ok('a'))
///   instance 3 A.1 Ok('a'))
///   instance 4 BYE!
///   instance 1 B Ok('b')
///   instance 2 B Ok('b')
///   instance 3 B Ok('b')
///   instance 1 A.2 Ok('a'))
///   instance 2 A.2 Ok('a'))
///   instance 3 A.2 Ok('a'))
///   instance 1 B Ok('b')
///   instance 2 B Ok('b')
///   instance 3 B Ok('b')
///   instance 1 A.3 Ok('a'))
///   instance 2 A.3 Ok('a'))
///   instance 3 A.3 Ok('a'))
///   instance 1 B Ok('b')
///   instance 2 B Ok('b')
///   instance 3 B Ok('b')
/// Done
/// ```
#[derive(Default)]
#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
pub struct CoroManager<T, E> {
    #[allow(clippy::type_complexity)]
    coros: VecDeque<Pin<Box<dyn Future<Output = OptRes<T, E>>>>>,
}

#[cfg(feature = "alloc")]
impl<T, E: 'static + Debug> CoroManager<T, E> {
    /// Returns a new empty manager.
    pub fn new() -> Self {
        CoroManager { coros: VecDeque::new() }
    }

    /// Adds a closure to the manager.
    pub fn push<C, F>(&mut self, closure: C)
    where
        C: FnOnce(CoroWorker<T, E>) -> F,
        F: Future<Output = OptRes<T, E>> + 'static,
    {
        self.coros.push_back(Box::pin(closure(CoroWorker::new())));
    }

    /// Runs all the coroutines to completion.
    pub fn run(&mut self) {
        let waker = TaskWaker::noop();
        let mut context = TaskContext::from_waker(waker);

        while let Some(mut cor) = self.coros.pop_front() {
            let polled = cor.as_mut().poll(&mut context);
            // println!("  coroutine polled:");

            match polled {
                TaskPoll::Pending => {
                    // println!("  - pending, push back");
                    self.coros.push_back(cor);
                }
                TaskPoll::Ready(_result) => {
                    // println!("  - READY");
                    // if let Some(Err(err)) = result {
                    //     // eprintln!("    Error in coroutine: {:?}", err);
                    // }
                }
            }
        }
    }
}
