// devela::data::layout::array::coord
//
//! Logical array-coordinate traversal.
//
// > A finite exhaustive traversal of a rectangular n-dimensional coordinate domain.

use crate::{ArrayShape, IteratorFused, is, whilst};

#[doc = crate::_tags!(data_structure iterator)]
/// An iterator over the coordinates of an array shape.
#[doc = crate::_doc_meta!{
    location("data/layout/array"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__: ArrayCoordIter<2> = 28|224),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__: ArrayCoordIter<2> = 56|448),
}]
/// Coordinates are yielded in canonical logical order,
/// with axis `0` changing fastest.
///
/// This matches physical storage order for a dense-first layout. For other
/// layouts, logical coordinate order and physical storage order may differ.
///
/// For a shape with lengths `[2, 3]`, the sequence is:
///
/// ```text
/// [0, 0]
/// [1, 0]
/// [0, 1]
/// [1, 1]
/// [0, 2]
/// [1, 2]
/// ```
///
/// This iterator describes only the logical coordinate domain.
/// It does not inspect an [`ArrayLayout`][crate::ArrayLayout]'s
/// offset or strides and does not access any physical storage.
///
/// A rank-zero shape yields its sole coordinate, `[]`, once.
/// A shape with any zero-length axis yields no coordinates.
///
/// The inherent [`next`][Self::next] method is `const`. The [`Iterator`]
/// implementation delegates to it for ordinary runtime iteration.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayCoordIter<const RANK: usize> {
    shape: ArrayShape<RANK>,
    front: [usize; RANK],
    back: [usize; RANK],
    remaining: usize,
}

impl<const RANK: usize> ArrayCoordIter<RANK> {
    /// Creates an iterator with a previously validated element count.
    pub(crate) const fn new(shape: ArrayShape<RANK>, remaining: usize) -> Self {
        let mut back = [0; RANK];
        if remaining != 0 {
            let lengths = shape.lengths();
            whilst! { axis in 0..RANK; {
                back[axis] = lengths[axis] - 1;
            }}
        }
        Self { shape, front: [0; RANK], back, remaining }
    }

    /// Returns the number of coordinates not yet yielded.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.remaining
    }
    /// Returns whether no coordinates remain.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.remaining == 0
    }

    /// Returns the complete logical shape being traversed.
    pub const fn shape(&self) -> ArrayShape<RANK> {
        self.shape
    }

    /// Advances the iterator and returns the next coordinate from the front.
    #[must_use]
    pub const fn next(&mut self) -> Option<[usize; RANK]> {
        is! { self.remaining == 0, return None }
        let coord = self.front;
        self.remaining -= 1;
        is! { self.remaining != 0, self.advance_front() }
        Some(coord)
    }

    /// Advances the iterator and returns the next coordinate from the back.
    #[must_use]
    pub const fn next_back(&mut self) -> Option<[usize; RANK]> {
        is! { self.remaining == 0, return None }
        let coord = self.back;
        self.remaining -= 1;
        is! { self.remaining != 0, self.advance_back() }
        Some(coord)
    }

    /// Returns the next coordinate from the front without advancing the iterator.
    pub const fn peek(&self) -> Option<[usize; RANK]> {
        is! { self.remaining == 0, None, Some(self.front) }
    }
    /// Returns the next coordinate from the back without advancing the iterator.
    pub const fn peek_back(&self) -> Option<[usize; RANK]> {
        is! { self.remaining == 0, None, Some(self.back) }
    }

    const fn advance_front(&mut self) {
        let lengths = self.shape.lengths();
        whilst! { axis in 0..RANK; {
            self.front[axis] += 1;
            if self.front[axis] < lengths[axis] { return; }
            self.front[axis] = 0;
        }}
    }
    const fn advance_back(&mut self) {
        let lengths = self.shape.lengths();
        whilst! { axis in 0..RANK; {
            if self.back[axis] != 0 {
                self.back[axis] -= 1;
                return;
            }
            self.back[axis] = lengths[axis] - 1;
        }}
    }
}

/* impl traits */

impl<const RANK: usize> Iterator for ArrayCoordIter<RANK> {
    type Item = [usize; RANK];

    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn count(self) -> usize {
        self.remaining
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining;
        (len, Some(len))
    }
}
impl<const RANK: usize> DoubleEndedIterator for ArrayCoordIter<RANK> {
    fn next_back(&mut self) -> Option<Self::Item> {
        Self::next_back(self)
    }
}
impl<const RANK: usize> ExactSizeIterator for ArrayCoordIter<RANK> {
    fn len(&self) -> usize {
        Self::len(self)
    }
}
impl<const RANK: usize> IteratorFused for ArrayCoordIter<RANK> {}

#[cfg(test)]
mod _test {
    use super::*;
    #[cfg(feature = "alloc")]
    use crate::Vec;
    use crate::{Array, ArrayLayout, const_assert};

    const COORDS_2_3: [[usize; 2]; 6] = {
        let shape = ArrayShape::new([2, 3]);
        let layout = match ArrayLayout::dense_first(shape) {
            Ok(layout) => layout,
            Err(_) => panic!("unexpected layout overflow"),
        };
        let mut iter = layout.coords();
        let mut output = [[0; 2]; 6];
        let mut index = 0;
        while let Some(coord) = iter.next() {
            output[index] = coord;
            index += 1;
        }
        assert!(index == 6);
        assert!(iter.is_empty());
        output
    };
    const LAST_2_3: [usize; 2] = {
        let shape = ArrayShape::new([2, 3]);
        let layout = match ArrayLayout::dense_first(shape) {
            Ok(layout) => layout,
            Err(_) => panic!("unexpected layout overflow"),
        };
        let mut iter = layout.coords();
        match iter.next_back() {
            Some(coord) => coord,
            None => panic!("missing coordinate"),
        }
    };
    #[test]
    const fn const_coordinate_iteration() {
        const_assert!(eq COORDS_2_3[0][0], 0);
        const_assert!(eq COORDS_2_3[0][1], 0);
        const_assert!(eq COORDS_2_3[1][0], 1);
        const_assert!(eq COORDS_2_3[1][1], 0);
        const_assert!(eq COORDS_2_3[2][0], 0);
        const_assert!(eq COORDS_2_3[2][1], 1);
        const_assert!(eq COORDS_2_3[5][0], 1);
        const_assert!(eq COORDS_2_3[5][1], 2);
    }
    #[test]
    const fn const_back_coordinate_iteration() {
        const_assert!(eq LAST_2_3[0], 1);
        const_assert!(eq LAST_2_3[1], 2);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn shape_coordinate_sequence() {
        let coords: Vec<_> = ArrayShape::new([2, 3]).try_coords().unwrap().collect();
        assert_eq!(coords, [[0, 0], [1, 0], [0, 1], [1, 1], [0, 2], [1, 2],]);
    }
    #[test]
    #[cfg(feature = "alloc")]
    fn coordinate_order_is_layout_independent() {
        let shape = ArrayShape::new([2, 3]);
        let first: Vec<_> = ArrayLayout::dense_first(shape).unwrap().coords().collect();
        let last: Vec<_> = ArrayLayout::dense_last(shape).unwrap().coords().collect();
        assert_eq!(first, last);
    }
    #[test]
    fn scalar_coordinate() {
        let mut iter = ArrayShape::<0>::new([]).try_coords().unwrap();
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some([]));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn empty_shape_has_no_coordinates() {
        let mut iter = ArrayShape::new([4, 0, 8]).try_coords().unwrap();
        assert!(iter.is_empty());
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn overflowing_shape_rejects_exact_iteration() {
        let shape = ArrayShape::new([usize::MAX, 2]);
        assert!(shape.try_coords().is_err());
    }
    #[test]
    fn exact_size_tracks_remaining_coordinates() {
        let mut iter = ArrayShape::new([2, 2]).try_coords().unwrap();
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some([0, 0]));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some([1, 0]));
        assert_eq!(iter.len(), 2);
    }
    #[test]
    fn array_coords_do_not_depend_on_backing() {
        let layout = ArrayLayout::dense_first(ArrayShape::new([2, 2])).unwrap();
        let array = Array::try_from_array([0, 1, 2, 3], layout).unwrap();
        let mut coords = array.coords();

        assert_eq!(coords.next(), Some([0, 0]));
        assert_eq!(coords.next(), Some([1, 0]));
        assert_eq!(coords.next(), Some([0, 1]));
        assert_eq!(coords.next(), Some([1, 1]));
        assert_eq!(coords.next(), None);
    }
}
