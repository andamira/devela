// devela::work::future::block
//
//! Defines the private [`future_block`] standalone function.
//
// Original source code by Joshua Barretto, licensed as MIT OR Apache-2.0
// https://crates.io/crates/pollster/0.3.0
//
// MAYBE IMPROVE:
// - [Lock-free and alloc-free implementation](https://github.com/zesterer/pollster/pull/9)
// - [add benchmarks](https://github.com/zesterer/pollster/pull/20)

use crate::{Arc, Condvar, Future, Mutex, TaskContext, TaskPoll, TaskWake, TaskWaker, pin};

/// Blocks the thread until the `future` is ready.
///
/// See also the [`ExtFuture`][super::ExtFuture] trait.
///
#[doc = crate::doc_!(vendor: "pollster")]
pub(crate) fn future_block<F: Future>(mut future: F) -> F::Output {
    // Pin the future so that it can be polled.
    let mut future = pin!(future);

    // Signal used to wake up the thread for polling as the future moves to completion. We need to
    // use an `Arc` because, although the lifetime of `future` is limited to this function, the
    // underlying IO abstraction might keep the signal alive for far longer. `Arc` is a thread-safe
    // way to allow this to happen.
    // MAYBE: Investigate ways to reuse this `Arc<Signal>`... perhaps via a `static`?
    let signal = Arc::new(Signal::new());

    // Create a context that will be passed to the future.
    let waker = TaskWaker::from(Arc::clone(&signal));
    let mut context = TaskContext::from_waker(&waker);

    // Poll the future to completion
    loop {
        match future.as_mut().poll(&mut context) {
            TaskPoll::Pending => signal.wait(),
            TaskPoll::Ready(item) => break item,
        }
    }
}

struct Signal {
    state: Mutex<SignalState>,
    cond: Condvar,
}

enum SignalState {
    Empty,
    Waiting,
    Notified,
}

impl TaskWake for Signal {
    // WAIT: [arbitrary_self_types](https://github.com/rust-lang/rust/pull/135881)
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

impl Signal {
    fn new() -> Self {
        Self {
            state: Mutex::new(SignalState::Empty),
            cond: Condvar::new(),
        }
    }

    fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            SignalState::Notified => {
                // notify() was called before we got here,
                // consume it here without waiting and return immediately.
                *state = SignalState::Empty;
            }
            // This should not be possible because our signal is created within a function and
            // never handed out to any other threads. If this is the case, we have a serious
            // problem so we panic immediately to avoid anything more problematic happening.
            SignalState::Waiting => {
                unreachable!("Multiple threads waiting on the same signal: Open a bug report!");
            }
            SignalState::Empty => {
                // Nothing has happened yet, and we're the only thread waiting (as should be the
                // case!). Set the state accordingly and begin polling the condvar in a loop until
                // it's no longer telling us to wait. The loop prevents incorrect spurious wakeups.
                *state = SignalState::Waiting;
                while let SignalState::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }

    fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            // The signal was already notified, no need to do anything because the thread will be
            // waking up anyway
            SignalState::Notified => {}
            // The signal wasn't notified but a thread isnt waiting on it, so we can avoid doing
            // unnecessary work by skipping the condvar and leaving behind a message telling the
            // thread that a notification has already occurred should it come along in the future.
            SignalState::Empty => *state = SignalState::Notified,
            // The signal wasn't notified and there's a waiting thread. Reset the signal so it can
            // be waited on again and wake up the thread. Because there should only be a single
            // thread waiting, `notify_all` would also be valid.
            SignalState::Waiting => {
                *state = SignalState::Empty;
                self.cond.notify_one();
            }
        }
    }
}
