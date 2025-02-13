// devela::num::cycle
//
//! Defines [`Cycle`], [`CycleCount`].
//

/// A repeating cycle defined by a fundamental period.
///
/// A `Cycle` encapsulates the basic unit over which any phenomenon repeats,
/// whether in time, space, or any abstract domain. It is the foundation for
/// constructing more complex periodic behaviors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cycle<T> {
    /// The fundamental period of the cycle.
    pub period: T,
}

/// A cycle that repeats a fixed number of times.
///
/// `CycleCount` couples a fundamental `Cycle` with a discrete repetition count.
///
/// This is useful when the number of repetitions is significant.
/// For example, when an animation should loop a specified number of times.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CycleCount<T, N> {
    /// The underlying repeating cycle.
    pub cycle: Cycle<T>,
    /// The total number of repetitions.
    pub count: N,
}
