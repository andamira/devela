// devela/src/num/signal/curve.rs
//
//!
//

use crate::{Add, Mul, SignalAt, Sub};

#[doc = crate::_tags!(num signal)]
/// A linear ramp from `start` to `end`.
#[doc = crate::_doc_meta!{location("num/signal")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurveRamp<T> {
    /// The value returned at the beginning of the ramp.
    pub start: T,
    /// The value returned at the end of the ramp.
    pub end: T,
}
impl<T> CurveRamp<T> {
    /// Creates a linear ramp from `start` to `end`.
    pub const fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}
impl<T> SignalAt<T> for CurveRamp<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Sample = T;
    fn at(&self, x: T) -> T {
        self.start + (self.end - self.start) * x
    }
}
