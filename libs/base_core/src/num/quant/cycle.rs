// devela_base_core::num::quant::cycle
//
//! Defines [`Cycle`], [`CycleCount`].
//

use crate::ConstInitCore;

#[doc = crate::_tags!(quant)]
/// A repeating cycle defined by a fundamental period.
#[doc = crate::_doc_location!("num/quant")]
///
/// A `Cycle` encapsulates the basic unit over which any phenomenon repeats,
/// whether in time, space, or any abstract domain. It is the foundation for
/// constructing more complex periodic behaviors.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cycle<T> {
    /// The fundamental period of the cycle.
    pub period: T,
}
impl<T: ConstInitCore> ConstInitCore for Cycle<T> {
    const INIT: Self = Self { period: T::INIT };
}

#[doc = crate::_tags!(quant)]
/// A cycle that repeats a fixed number of times.
#[doc = crate::_doc_location!("num/quant")]
///
/// `CycleCount` couples a fundamental `Cycle` with a discrete repetition count.
///
/// This is useful when the number of repetitions is significant.
/// For example, when an animation should loop a specified number of times.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CycleCount<T, N> {
    /// The underlying repeating cycle.
    pub cycle: Cycle<T>,
    /// The total number of repetitions.
    pub count: N,
}
impl<T: ConstInitCore, N: ConstInitCore> ConstInitCore for CycleCount<T, N> {
    const INIT: Self = Self { cycle: Cycle::<T>::INIT, count: N::INIT };
}

// WIPZONE

// #[doc = crate::_tags!(quant)]
// /// Cyclic behavior.
// ///
// /// Defines operations common to periodic structures, such as retrieving
// /// the period, normalizing values within the cycle, applying offsets,
// /// and handling bounded or repeated cycles.
// pub trait Cycled<T> {
//     /// Returns the fundamental period of the cycle.
//     fn cycle_period(&self) -> T;
//
//     /// Normalizes a value within the cycle's periodic range.
//     ///
//     /// Ensures that the input `value` is wrapped within `[0, period)`.
//     fn cycle_normalize(&self, value: T) -> T;
//
//     ///
//     fn cycle_count(&self) -> T;
//
//     ///
//     fn cycle_offset(&self) -> T;
//
//     /// Advances the cycle by a given offset.
//     ///
//     /// This may modify internal state or return a new cycle with the offset applied.
//     fn with_offset(&self, offset: T) -> Self;
//
//     /// Determines how many complete cycles fit within a given range.
//     ///
//     /// This method is useful for bounded or counted cycles.
//     fn cycles_in_range(&self, range: T) -> T;
// }
