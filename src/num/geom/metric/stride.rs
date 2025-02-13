// devela::num::geom::metric::stride
//
//! Defines [`Stride`].
//

/// A step size for traversing dimensions or repetitions.
///
/// `Stride` defines the spacing between adjacent elements in a structured layout.
/// It does not define structure itself, but rather **how elements are accessed within it**.
///
/// - In **1D**, `Stride<T, 1>` represents uniform step spacing (e.g. sampling rate).
/// - In **nD**, `Stride<T, N>` defines the step sizes across `N` dimensions.
///
/// Common applications:
/// - **Numerical computing** (matrix row/column strides)
/// - **Memory layouts** (pixel buffers, structured arrays)
/// - **Geometric traversal** (lattices, grids, fractal stepping)
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stride<T, const D: usize> {
    /// Step sizes per dimension.
    pub step: [T; D],
}
