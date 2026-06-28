// devela/src/num/signal/emit.rs
//
//! Signal emitters, generators, and produced value streams.
//

use crate::SignalAt;

/* traits */

#[doc = crate::_tags!(num signal)]
/// A stateful signal that emits one sample per step.
#[doc = crate::_doc_meta!{location("num/signal")}]
pub trait SignalNext {
    /// The emitted sample type.
    type Sample;
    /// Emits the next sample and advances internal state.
    fn next(&mut self) -> Self::Sample;
}

/* types */

#[doc = crate::_tags!(num signal)]
/// Emits the same sample every time.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalConst<T> {
    /// The sample value emitted or returned every time.
    pub sample: T,
}
impl<T> SignalConst<T> {
    /// Creates a signal that always yields `sample`.
    pub const fn new(sample: T) -> Self {
        Self { sample }
    }
}
impl<T: Copy> SignalNext for SignalConst<T> {
    type Sample = T;
    fn next(&mut self) -> T {
        self.sample
    }
}
impl<X, T: Copy> SignalAt<X> for SignalConst<T> {
    type Sample = T;
    fn at(&self, _: X) -> T {
        self.sample
    }
}

#[doc = crate::_tags!(num signal)]
/// Wraps a function or closure as a signal.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalFn<F> {
    /// The function or closure used to produce signal samples.
    pub f: F,
}
impl<F> SignalFn<F> {
    /// Creates a signal from a function or closure.
    pub const fn new(f: F) -> Self {
        Self { f }
    }
}
impl<F: FnMut() -> T, T> SignalNext for SignalFn<F> {
    type Sample = T;
    fn next(&mut self) -> T {
        (self.f)()
    }
}
impl<X, F: Fn(X) -> T, T> SignalAt<X> for SignalFn<F> {
    type Sample = T;
    fn at(&self, x: X) -> T {
        (self.f)(x)
    }
}
