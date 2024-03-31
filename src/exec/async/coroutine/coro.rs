// devela::exec::async::coroutine
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

#[cfg(feature = "alloc")]
use crate::{
    _alloc::{boxed::Box, collections::VecDeque},
    exec::task_waker_noop,
};
use crate::{
    code::{serr, sok, OptRes},
    data::Pin,
    exec::{Future, TaskContext, TaskPoll},
};
use core::fmt::Debug;

/* coroutine */

/// Represents a single thread stackless coroutine.
///
/// It has a private status that can be either running or halted.
#[derive(Debug)]
pub struct Coro<T, E> {
    status: CoroStatus,
    result: OptRes<T, E>,
}

// Private coroutine status.
#[derive(Clone, Copy, Debug)]
enum CoroStatus {
    Halted,
    Running,
}

impl<T, E> Coro<T, E> {
    // Returns a new coroutine.
    #[inline]
    #[allow(unused)]
    fn new() -> Self {
        Coro {
            status: CoroStatus::Running,
            result: None,
        }
    }

    /// Yields an [`Ok`] `value` and returns an awaitable CoroYield.
    #[inline]
    pub fn yield_ok(&mut self, value: T) -> CoroYield<'_, T, E> {
        self.result = sok(value);
        CoroYield { cor: self }
    }

    /// Yields an [`Err`] and returns an awaitable future.
    #[inline]
    pub fn yield_err(&mut self, error: E) -> CoroYield<'_, T, E> {
        self.result = serr(error);
        CoroYield { cor: self }
    }
}

/* yielder */

/// A future that alternates between [`Ready`][TaskPoll::Ready] and
/// [`Pending`][TaskPoll::Pending] status each time it's polled.
///
/// This allows the coroutine to yield control back and be resumed later.
pub struct CoroYield<'a, T, E> {
    cor: &'a mut Coro<T, E>,
}

impl<'a, T, E> Future for CoroYield<'a, T, E> {
    type Output = OptRes<T, E>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut TaskContext) -> TaskPoll<OptRes<T, E>> {
        match self.cor.status {
            CoroStatus::Halted => {
                self.cor.status = CoroStatus::Running;
                if let Some(result) = self.cor.result.take() {
                    match result {
                        Err(error) => TaskPoll::Ready(serr(error)),
                        Ok(value) => TaskPoll::Ready(sok(value)),
                    }
                } else {
                    unreachable!();
                }
            }
            CoroStatus::Running => {
                self.cor.status = CoroStatus::Halted;
                TaskPoll::Pending
            }
        }
    }
}

/* runner */

/// A managed dynamic collection of single-thread [`Coro`]utines.
///
/// It maintains a queue of coroutines in the stack, and runs them in a loop until
/// they are all complete.
///
/// When a coroutine is polled and returns [`TaskPoll::Pending`], it is put back
/// into the queue to be run again later. If it returns [`Poll::Ready`]
/// it is considered complete and is not put back into the queue.
///
/// # Examples
/// ```
#[doc = include_str!("../../../../examples/coro_run.rs")]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct CoroRun<T, E> {
    #[allow(clippy::type_complexity)]
    coros: VecDeque<Pin<Box<dyn Future<Output = OptRes<T, E>>>>>,
}

#[cfg(feature = "alloc")]
impl<T, E: 'static + Debug> CoroRun<T, E> {
    /// Returns a new empty runner.
    pub fn new() -> Self {
        CoroRun {
            coros: VecDeque::new(),
        }
    }

    /// Adds a closure to the runner.
    pub fn push<C, F>(&mut self, closure: C)
    where
        F: Future<Output = OptRes<T, E>> + 'static,
        C: FnOnce(Coro<T, E>) -> F,
    {
        self.coros.push_back(Box::pin(closure(Coro::new())));
    }

    /// Runs all the coroutines to completion.
    pub fn run(&mut self) {
        let waker = task_waker_noop();
        let mut context = TaskContext::from_waker(&waker);

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
