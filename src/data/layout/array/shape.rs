// devela/src/data/layout/array/shape.rs
//
//! Defines [`ArrayShape`].
//

use crate::{ArrayCoordIter, Overflow, is, unwrap, whilst};

#[doc = crate::_tags!(data_structure)]
/// The ordered lengths of an array's logical axes.
#[doc = crate::_doc_meta!{
    location("data/layout/array", struct ArrayShape),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ArrayShape<2> = 8|64; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ArrayShape<2> = 16|128; niche !Option),
}]
/// `RANK` is the number of axes and is known at compile time.
///
/// A rank-zero shape, `ArrayShape<0>`, represents one scalar element.
/// A shape with one or more zero-length axes represents an empty array.
///
/// A shape does not describe storage order, strides, ownership, or access.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrayShape<const RANK: usize> {
    pub(super) lengths: [usize; RANK],
}
#[rustfmt::skip]
impl<const RANK: usize> ArrayShape<RANK> {
    /// The number of axes.
    pub const RANK: usize = RANK;

    /// Creates a shape from its ordered axis lengths.
    pub const fn new(lengths: [usize; RANK]) -> Self { Self { lengths } }

    /// Returns the number of axes.
    pub const fn rank(&self) -> usize { RANK }

    /// Returns the ordered axis lengths.
    pub const fn lengths(&self) -> &[usize; RANK] { &self.lengths }

    /// Returns the ordered axis lengths by value.
    pub const fn into_lengths(self) -> [usize; RANK] { self.lengths }

    /// Returns the length of `axis`, or `None` if it is out of bounds.
    pub const fn axis_len(&self, axis: usize) -> Option<usize> {
        if axis < RANK { Some(self.lengths[axis]) } else { None }
    }
    /// Returns whether no logical element exists.
    ///
    /// Rank zero is not empty. Any zero-length axis makes a shape empty.
    pub const fn is_empty(&self) -> bool {
        whilst! { axis in 0..RANK; { if self.lengths[axis] == 0 { return true; } }}
        false
    }
    /// Returns the total number of logical elements.
    ///
    /// Rank zero contains one element. A shape containing a zero-length
    /// axis contains zero elements.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the product of the non-zero axis lengths
    /// is not representable as a `usize`.
    pub const fn element_count(&self) -> Result<usize, Overflow> {
        if self.is_empty() { return Ok(0); }
        let mut count = 1usize;
        whilst! { axis in 0..RANK; {
            count = match count.checked_mul(self.lengths[axis]) {
                Some(count) => count,
                None => return Err(Overflow(None)),
            };
        }}
        Ok(count)
    }

    /* coords */

    /// Returns whether `coord` belongs to this shape.
    ///
    /// The sole rank-zero coordinate, `[]`, belongs to a scalar shape.
    pub const fn contains_coord(&self, coord: [usize; RANK]) -> bool {
        whilst! { axis in 0..RANK; {
            if coord[axis] >= self.lengths[axis] { return false; }
        }}
        true
    }

    /// Returns an iterator over every coordinate belonging to this shape.
    ///
    /// Axis `0` changes fastest.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the total number of coordinates is not
    /// representable as a `usize`.
    pub const fn try_coords(&self) -> Result<ArrayCoordIter<RANK>, Overflow> {
        match self.element_count() {
            Ok(count) => Ok(ArrayCoordIter::new(*self, count)),
            Err(error) => Err(error),
        }
    }
    /// Returns the coordinate at `ordinal` in canonical logical order.
    ///
    /// Axis `0` changes fastest. This is the same order yielded by
    /// [`try_coords`][Self::try_coords].
    ///
    /// Returns `None` if `ordinal` is outside the shape or if the shape's
    /// element count is not representable as a `usize`.
    pub const fn coord_at(&self, mut ordinal: usize) -> Option<[usize; RANK]> {
        let count = unwrap![ok_some? self.element_count()];
        is! { ordinal >= count, return None }
        let mut coord = [0; RANK];
        whilst! { axis in 0..RANK; {
            let len = self.lengths[axis];
            coord[axis] = ordinal % len;
            ordinal /= len;
        }}
        Some(coord)
    }
    /// Returns `coord`'s ordinal in canonical logical order.
    ///
    /// Axis `0` changes fastest. For every representable valid ordinal `i`,
    /// `coord_ordinal(coord_at(i)?) == Some(i)`.
    ///
    /// Returns `None` if `coord` is outside the shape or if the shape's
    /// element count is not representable as a `usize`.
    pub const fn coord_ordinal(&self, coord: [usize; RANK]) -> Option<usize> {
        is! { !self.contains_coord(coord), return None }
        let (mut ordinal, mut stride) = (0usize, 1usize);
        whilst! { axis in 0..RANK; {
            ordinal = match ordinal.checked_add(unwrap![some? coord[axis].checked_mul(stride)]) {
                Some(value) => value,
                None => return None,
            };
            stride = match stride.checked_mul(self.lengths[axis]) {
                Some(value) => value,
                None => return None,
            };
        }}
        Some(ordinal)
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn shape_scalar() {
        let shape = ArrayShape::<0>::new([]);
        assert_eq!(shape.rank(), 0);
        assert_eq!(ArrayShape::<0>::RANK, 0);
        assert_eq!(shape.axis_len(0), None);
        assert!(!shape.is_empty());
        assert_eq!(shape.element_count().unwrap(), 1);
        assert!(shape.contains_coord([]));
        assert_eq!(shape.into_lengths(), []);
    }
    #[test]
    fn shape_empty() {
        let shape = ArrayShape::new([4, 0, 8]);
        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.lengths(), &[4, 0, 8]);
        assert_eq!(shape.axis_len(0), Some(4));
        assert_eq!(shape.axis_len(1), Some(0));
        assert_eq!(shape.axis_len(2), Some(8));
        assert_eq!(shape.axis_len(3), None);
        assert!(shape.is_empty());
        assert_eq!(shape.element_count().unwrap(), 0);
        assert!(!shape.contains_coord([0, 0, 0]));
    }
    #[test]
    fn empty_shape_short_circuits_overflow() {
        let shape = ArrayShape::new([usize::MAX, 0, usize::MAX]);
        assert!(shape.is_empty());
        assert_eq!(shape.element_count().unwrap(), 0);
    }
    #[test]
    fn shape_element_count_and_overflow() {
        assert_eq!(ArrayShape::new([2, 3, 4]).element_count().unwrap(), 24);
        assert!(ArrayShape::new([usize::MAX, 2]).element_count().is_err());
    }
    #[test]
    fn shape_coordinate_bounds() {
        let shape = ArrayShape::new([2, 3, 4]);
        assert!(shape.contains_coord([0, 0, 0]));
        assert!(shape.contains_coord([1, 2, 3]));
        assert!(!shape.contains_coord([2, 0, 0]));
        assert!(!shape.contains_coord([0, 3, 0]));
        assert!(!shape.contains_coord([0, 0, 4]));
    }
    #[test]
    fn bidirectional_coordinate_iteration() {
        let mut iter = ArrayShape::new([2, 3]).try_coords().unwrap();
        assert_eq!(iter.peek(), Some([0, 0]));
        assert_eq!(iter.peek_back(), Some([1, 2]));
        assert_eq!(iter.len(), 6);
        assert_eq!(iter.next(), Some([0, 0]));
        assert_eq!(iter.next_back(), Some([1, 2]));
        assert_eq!(iter.next_back(), Some([0, 2]));
        assert_eq!(iter.next(), Some([1, 0]));
        assert_eq!(iter.peek(), Some([0, 1]));
        assert_eq!(iter.peek_back(), Some([1, 1]));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some([0, 1]));
        assert_eq!(iter.next_back(), Some([1, 1]));
        assert!(iter.is_empty());
        assert_eq!(iter.peek(), None);
        assert_eq!(iter.peek_back(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn implements_double_ended_iterator() {
        let mut iter = ArrayShape::new([2, 3]).try_coords().unwrap();
        assert_eq!(DoubleEndedIterator::next_back(&mut iter), Some([1, 2]),);
        assert_eq!(iter.len(), 5);
    }
    #[test]
    fn three_dimensional_carry_and_borrow() {
        let mut iter = ArrayShape::new([2, 2, 2]).try_coords().unwrap();
        assert_eq!(iter.next(), Some([0, 0, 0]));
        assert_eq!(iter.next(), Some([1, 0, 0]));
        assert_eq!(iter.next(), Some([0, 1, 0]));
        assert_eq!(iter.next_back(), Some([1, 1, 1]));
        assert_eq!(iter.next_back(), Some([0, 1, 1]));
        assert_eq!(iter.next_back(), Some([1, 0, 1]));
    }
    #[test]
    fn scalar_and_empty_coordinate_ordinals() {
        let scalar = ArrayShape::<0>::new([]);
        assert_eq!(scalar.coord_at(0), Some([]));
        assert_eq!(scalar.coord_at(1), None);
        assert_eq!(scalar.coord_ordinal([]), Some(0));
        let empty = ArrayShape::new([2, 0, 3]);
        assert_eq!(empty.coord_at(0), None);
        assert_eq!(empty.coord_ordinal([0, 0, 0]), None);
    }
    #[test]
    fn overflowing_shape_has_no_coordinate_ordinals() {
        let shape = ArrayShape::new([usize::MAX, 2]);
        assert_eq!(shape.coord_at(0), None);
        assert_eq!(shape.coord_ordinal([0, 0]), None);
    }
}
