// devela/src/geom/metric/stride.rs
//
//! Defines [`Stride`].
//
// Boundary: Stride describes per-axis stepping.
// It does not select iteration order or define a memory layout.
//

#[doc = crate::_tags!(geom)]
/// A step size for traversing dimensions or repetitions.
#[doc = crate::_doc_meta!{
    location("geom/metric", struct Stride),
    test_size_of(Stride<u32, 2> = 8|64; niche !Option),
}]
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
#[repr(transparent)]
pub struct Stride<T, const D: usize> {
    /// The step sizes per dimension.
    pub dim: [T; D],
}

#[doc = crate::_tags!(geom)]
/// A 1-dimensional [`Stride`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Stride1),
    test_size_of(Stride1<u32> = 4|32; niche !Option),
}]
pub type Stride1<T> = Stride<T, 1>;

#[doc = crate::_tags!(geom)]
/// A 2-dimensional [`Stride`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Stride2),
    test_size_of(Stride2<u32> = 8|64; niche !Option),
}]
pub type Stride2<T> = Stride<T, 2>;

#[doc = crate::_tags!(geom)]
/// A 3-dimensional [`Stride`].
#[doc = crate::_doc_meta!{
    location("geom/metric", type Stride3),
    test_size_of(Stride3<u32> = 12|96; niche !Option),
}]
pub type Stride3<T> = Stride<T, 3>;

crate::_geom_dim_impl_common![common_methods: Stride];
crate::_geom_dim_impl_common![common_traits: Stride];
