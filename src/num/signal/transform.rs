// devela/src/num/signal/transform.rs
//
//! Signal mapping, modulation, and signal-to-signal transformation.
//
// TOC:
// - struct SignalClamp
// - struct SignalMap
// - struct SignalScale
// - struct SignalZip

use crate::{Mul, SignalAt, SignalNext};

#[doc = crate::_tags!(num signal)]
/// Clamps each sample to `[min, max]`.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalClamp<S, T> {
    /// The signal whose samples are clamped.
    pub signal: S,
    /// The minimum allowed sample value.
    pub min: T,
    /// The maximum allowed sample value.
    pub max: T,
}
#[rustfmt::skip]
impl<S, T> SignalClamp<S, T> {
    /// Creates a signal adapter that clamps samples to `[min, max]`.
    pub const fn new(signal: S, min: T, max: T) -> Self {
        Self { signal, min, max }
    }
    fn clamp_value(&self, v: T) -> T where T: Copy + PartialOrd {
        if v < self.min { self.min } else if v > self.max { self.max } else { v }
    }
}
impl<S, T> SignalNext for SignalClamp<S, T>
where
    S: SignalNext<Sample = T>,
    T: Copy + PartialOrd,
{
    type Sample = T;
    fn next(&mut self) -> T {
        let v = self.signal.next();
        self.clamp_value(v)
    }
}
impl<X, S, T> SignalAt<X> for SignalClamp<S, T>
where
    S: SignalAt<X, Sample = T>,
    T: Copy + PartialOrd,
{
    type Sample = T;
    fn at(&self, x: X) -> T {
        self.clamp_value(self.signal.at(x))
    }
}

#[doc = crate::_tags!(num signal)]
/// Maps each sample through a function.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalMap<S, F> {
    /// The signal whose samples are transformed.
    pub signal: S,
    /// The mapping function applied to each sample.
    pub f: F,
}
impl<S, F> SignalMap<S, F> {
    /// Creates a signal adapter that maps each sample through `f`.
    pub const fn new(signal: S, f: F) -> Self {
        Self { signal, f }
    }
}
impl<S, F, O> SignalNext for SignalMap<S, F>
where
    S: SignalNext,
    F: FnMut(S::Sample) -> O,
{
    type Sample = O;
    fn next(&mut self) -> O {
        (self.f)(self.signal.next())
    }
}
impl<X, S, F, O> SignalAt<X> for SignalMap<S, F>
where
    S: SignalAt<X>,
    F: Fn(S::Sample) -> O,
{
    type Sample = O;
    fn at(&self, x: X) -> O {
        (self.f)(self.signal.at(x))
    }
}

#[doc = crate::_tags!(num signal)]
/// Multiplies each sample by a fixed factor.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalScale<S, K> {
    /// The signal whose samples are scaled.
    pub signal: S,
    /// The factor multiplied with each sample.
    pub factor: K,
}
impl<S, K> SignalScale<S, K> {
    /// Creates a signal adapter that multiplies each sample by `factor`.
    pub const fn new(signal: S, factor: K) -> Self {
        Self { signal, factor }
    }
}
impl<S, K> SignalNext for SignalScale<S, K>
where
    S: SignalNext,
    S::Sample: Mul<K, Output = S::Sample>,
    K: Copy,
{
    type Sample = S::Sample;
    fn next(&mut self) -> Self::Sample {
        self.signal.next() * self.factor
    }
}
impl<X, S, K> SignalAt<X> for SignalScale<S, K>
where
    S: SignalAt<X>,
    S::Sample: Mul<K, Output = S::Sample>,
    K: Copy,
{
    type Sample = S::Sample;
    fn at(&self, x: X) -> Self::Sample {
        self.signal.at(x) * self.factor
    }
}

#[doc = crate::_tags!(num signal)]
/// Combines two signals sample-by-sample.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalZip<A, B, F> {
    /// The first input signal.
    pub a: A,
    /// The second input signal.
    pub b: B,
    /// The combining function applied to paired samples.
    pub f: F,
}
impl<A, B, F> SignalZip<A, B, F> {
    /// Creates a signal adapter that combines paired samples from two signals.
    pub const fn new(a: A, b: B, f: F) -> Self {
        Self { a, b, f }
    }
}
impl<A, B, F, O> SignalNext for SignalZip<A, B, F>
where
    A: SignalNext,
    B: SignalNext,
    F: FnMut(A::Sample, B::Sample) -> O,
{
    type Sample = O;
    fn next(&mut self) -> O {
        (self.f)(self.a.next(), self.b.next())
    }
}
impl<X, A, B, F, O> SignalAt<X> for SignalZip<A, B, F>
where
    X: Copy,
    A: SignalAt<X>,
    B: SignalAt<X>,
    F: Fn(A::Sample, B::Sample) -> O,
{
    type Sample = O;

    fn at(&self, x: X) -> O {
        (self.f)(self.a.at(x), self.b.at(x))
    }
}
