// devela/src/geom/affine/facet.rs
//
//! Defines [`SimplexFacetView`], [`SimplexFacetIter`].
//

use crate::{Point, Sign, Simplex, is};

#[doc = crate::_tags!(geom)]
/// A borrowed facet of a [`Simplex`], represented by one omitted vertex.
#[doc = crate::_doc_meta!{location("geom/affine")}]
///
/// A facet is a codimension-one face of a simplex. This view borrows the
/// original simplex and exposes its vertices in their original order, except
/// for the vertex at [`omitted_index`](Self::omitted_index).
///
/// No vertices are copied and no storage is allocated.
///
/// # Orientation
///
/// In the oriented boundary of a simplex, the facet omitting vertex `i` has
/// coefficient `(-1)^i`. This coefficient is returned by
/// [`boundary_sign`](Self::boundary_sign).
///
/// # Empty facets
///
/// The empty facet of a point simplex is not represented. Consequently,
/// [`Simplex::facet`] returns `None` for every `Simplex<T, D, 1>`.
#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct SimplexFacetView<'a, T, const D: usize, const V: usize> {
    pub(super) simplex: &'a Simplex<T, D, V>,
    pub(super) omitted: usize,
}
#[rustfmt::skip]
impl<'a, T, const D: usize, const V: usize> SimplexFacetView<'a, T, D, V> {
    /// Returns the simplex from which this facet is projected.
    pub const fn simplex(&self) -> &'a Simplex<T, D, V> { self.simplex }

    /// Returns the index of the omitted vertex.
    #[must_use]
    pub const fn omitted_index(&self) -> usize { self.omitted }

    /// Returns the vertex opposite this facet.
    pub const fn opposite_vertex(&self) -> &Point<T, D> { &self.simplex.vertices[self.omitted] }

    /// Returns the number of vertices in this facet.
    #[must_use]
    pub const fn vertex_count(&self) -> usize { V - 1 }

    /// Returns the dimension implied by this facet's vertex count.
    #[must_use]
    pub const fn dimension(&self) -> usize { V - 2 }

    /// Returns the facet vertex at `index`.
    ///
    /// The returned vertices retain their relative order in the parent simplex.
    #[must_use]
    pub const fn vertex(&self, index: usize) -> Option<&Point<T, D>> {
        is! { index >= V - 1, return None }
        let source = if index < self.omitted { index } else { index + 1 };
        Some(&self.simplex.vertices[source])
    }
    /// Returns this facet's coefficient in the oriented simplex boundary.
    ///
    /// This is positive when the omitted index is even and negative when it
    /// is odd.
    pub const fn boundary_sign(&self) -> Sign {
        is! { self.omitted & 1 == 0, Sign::Positive, Sign::Negative }
    }
}

#[doc = crate::_tags!(geom iterator)]
/// An iterator over the facets of a [`Simplex`].
#[doc = crate::_doc_meta!{location("geom/affine")}]
///
/// Facets are yielded in omitted-vertex order.
#[must_use]
#[derive(Clone, Debug)]
pub struct SimplexFacetIter<'a, T, const D: usize, const V: usize> {
    pub(super) simplex: &'a Simplex<T, D, V>,
    pub(super) front: usize,
    pub(super) back: usize,
}
#[rustfmt::skip]
impl<'a, T, const D: usize, const V: usize> SimplexFacetIter<'a, T, D, V> {
    /// Returns the number of facets not yet yielded.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.back - self.front
    }
    /// Returns the next facet.
    #[must_use]
    pub const fn next(&mut self) -> Option<SimplexFacetView<'a, T, D, V>> {
        is! { Self::len(self) == 0, return None }
        let omitted = self.front;
        self.front += 1;
        Some(SimplexFacetView { simplex: self.simplex, omitted })
    }
    /// Returns the next facet from the back.
    #[must_use]
    pub const fn next_back(&mut self) -> Option<SimplexFacetView<'a, T, D, V>> {
        is! { Self::len(self) == 0, return None }
        self.back -= 1;
        Some(SimplexFacetView { simplex: self.simplex, omitted: self.back })
    }
}

impl<'a, T, const D: usize, const V: usize> Iterator for SimplexFacetIter<'a, T, D, V> {
    type Item = SimplexFacetView<'a, T, D, V>;

    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn count(self) -> usize {
        Self::len(&self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T, const D: usize, const V: usize> DoubleEndedIterator for SimplexFacetIter<'_, T, D, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Self::next_back(self)
    }
}
impl<T, const D: usize, const V: usize> ExactSizeIterator for SimplexFacetIter<'_, T, D, V> {
    fn len(&self) -> usize {
        Self::len(self)
    }
}
impl<T, const D: usize, const V: usize> core::iter::FusedIterator
    for SimplexFacetIter<'_, T, D, V>
{
}
