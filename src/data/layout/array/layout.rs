// devela/src/data/layout/array/layout.rs
//
//! Defines [`ArrayLayout`], [`ArrayShape`].
//

use crate::{Overflow, is, unwrap, whilst};

#[doc = crate::_tags!(data_structure mem)]
/// An affine mapping from array coordinates to linear storage positions.
#[doc = crate::_doc_meta!{location("data/layout/array")}]
///
/// A physical storage position is calculated as:
///
/// ```text
/// offset + coord[0] × stride[0] + ... + coord[RANK - 1] × stride[RANK - 1]
/// ```
///
/// Offsets and strides are measured in elements, not bytes.
///
/// `ArrayLayout` does not own or borrow storage. An [`Array`][crate::Array]
/// binds a layout to accessible data and verifies that the storage is large enough.
///
/// # Invariants
///
/// Every safely constructed non-empty layout:
///
/// - has a representable element count and address range;
/// - maps every valid coordinate to a non-negative storage position;
/// - maps different logical coordinates to different storage positions.
///
/// The initial public constructors create only dense layouts.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrayLayout<const RANK: usize> {
    shape: ArrayShape<RANK>,
    offset: usize,
    strides: [isize; RANK],
}
impl<const RANK: usize> ArrayLayout<RANK> {
    /// Creates a dense layout whose last axis varies fastest.
    ///
    /// For shape `[2, 3]`, the resulting strides are `[3, 1]`.
    ///
    /// Empty layouts use zero strides because they address no elements.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the element count, address range,
    /// or an axis stride is not representable.
    pub const fn dense_last(shape: ArrayShape<RANK>) -> Result<Self, Overflow> {
        let element_count = unwrap![ok? shape.element_count()];
        is! { element_count > isize::MAX as usize + 1, return Err(Overflow(None)) }
        is! { shape.is_empty(), return Ok(Self { shape, offset: 0, strides: [0; RANK] }) }
        let (mut stride, mut strides) = (1usize, [0; RANK]);
        whilst! { axis in rev 0..RANK; {
            is! { stride > isize::MAX as usize, return Err(Overflow(None)) }
            strides[axis] = stride as isize;
            stride = unwrap![some_ok_or? stride.checked_mul(shape.lengths[axis]), Overflow(None)];
        }}
        Ok(Self { shape, offset: 0, strides })
    }
    /// Creates a dense layout whose first axis varies fastest.
    ///
    /// For shape `[2, 3]`, the resulting strides are `[1, 2]`.
    ///
    /// Empty layouts use zero strides because they address no elements.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the element count, address range,
    /// or an axis stride is not representable.
    pub const fn dense_first(shape: ArrayShape<RANK>) -> Result<Self, Overflow> {
        let element_count = unwrap![ok? shape.element_count()];
        is! { element_count > isize::MAX as usize + 1, return Err(Overflow(None)) }
        is! { shape.is_empty(), return Ok(Self { shape, offset: 0, strides: [0; RANK] }) }
        let (mut stride, mut strides) = (1usize, [0; RANK]);
        whilst! { axis in 0..RANK; {
            is! { stride > isize::MAX as usize, return Err(Overflow(None)) }
            strides[axis] = stride as isize;
            stride = unwrap![some_ok_or? stride.checked_mul(shape.lengths[axis]), Overflow(None)];
        }}
        Ok(Self { shape, offset: 0, strides })
    }
}
#[rustfmt::skip]
impl<const RANK: usize> ArrayLayout<RANK> {
    /// Returns the number of axes.
    pub const fn rank(&self) -> usize { RANK }

    /// Returns the logical shape.
    pub const fn shape(&self) -> ArrayShape<RANK> { self.shape }

    /// Returns the physical origin in the backing storage.
    pub const fn offset(&self) -> usize { self.offset }

    /// Returns the physical stride of every axis.
    pub const fn strides(&self) -> &[isize; RANK] { &self.strides }

    /// Returns whether the logical array has no elements.
    pub const fn is_empty(&self) -> bool { self.shape.is_empty() }

    /// Returns the number of logical elements.
    ///
    /// Every safely constructed layout has a representable element count.
    pub const fn element_count(&self) -> usize {
        is! { self.shape.is_empty(), return 0 }
        let mut count = 1;
        whilst! { axis in 0..RANK; {
            count *= self.shape.lengths[axis]; // validated by the layout constructors
        }}
        count
    }

    /// Returns the minimum backing-storage length required by this layout.
    ///
    /// Empty layouts require no storage.
    pub const fn required_storage_len(&self) -> usize {
        is! { self.is_empty(), return 0 }
        let mut highest = self.offset as isize;
        whilst! { axis in 0..RANK; {
            let last = (self.shape.lengths[axis] - 1) as isize;
            let delta = self.strides[axis] * last;
            is! { delta > 0, highest += delta }
        }}
        highest as usize + 1
    }
    /// Returns the storage position corresponding to `coord`.
    ///
    /// Returns `None` if the coordinate is outside the logical shape.
    pub const fn storage_index(&self, coord: [usize; RANK]) -> Option<usize> {
        is! { !self.shape.contains_coord(coord), return None }
        let mut index = self.offset as isize;
        whilst! { axis in 0..RANK; {
            is! { coord[axis] > isize::MAX as usize, return None }
            let delta = unwrap![some? self.strides[axis].checked_mul(coord[axis] as isize)];
            index = unwrap![some? index.checked_add(delta)];
        }}
        is! { index < 0, None, Some(index as usize) }
    }
    /// Decomposes the layout into shape, offset, and strides.
    pub const fn into_parts(self) -> (ArrayShape<RANK>, usize, [isize; RANK]) {
        (self.shape, self.offset, self.strides)
    }
}

#[doc = crate::_tags!(data_structure)]
/// The ordered lengths of an array's logical axes.
#[doc = crate::_doc_meta!{location("data/layout/array")}]
///
/// `RANK` is the number of axes and is known at compile time.
///
/// A rank-zero shape, `ArrayShape<0>`, represents one scalar element.
/// A shape with one or more zero-length axes represents an empty array.
///
/// A shape does not describe storage order, strides, ownership, or access.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArrayShape<const RANK: usize> {
    lengths: [usize; RANK],
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
    /// Returns whether `coord` belongs to this shape.
    ///
    /// The sole rank-zero coordinate, `[]`, belongs to a scalar shape.
    pub const fn contains_coord(&self, coord: [usize; RANK]) -> bool {
        whilst! { axis in 0..RANK; {
            if coord[axis] >= self.lengths[axis] { return false; }
        }}
        true
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
    fn dense_last_layout() {
        let shape = ArrayShape::new([2, 3, 4]);
        let layout = ArrayLayout::dense_last(shape).unwrap();
        assert_eq!(layout.rank(), 3);
        assert_eq!(layout.shape(), shape);
        assert_eq!(layout.offset(), 0);
        assert_eq!(layout.strides(), &[12, 4, 1]);
        assert_eq!(layout.element_count(), 24);
        assert_eq!(layout.required_storage_len(), 24);
        assert_eq!(layout.storage_index([0, 0, 0]), Some(0));
        assert_eq!(layout.storage_index([0, 0, 3]), Some(3));
        assert_eq!(layout.storage_index([0, 2, 0]), Some(8));
        assert_eq!(layout.storage_index([1, 0, 0]), Some(12));
        assert_eq!(layout.storage_index([1, 2, 3]), Some(23));
        assert_eq!(layout.storage_index([2, 0, 0]), None);
        assert_eq!(layout.storage_index([0, 3, 0]), None);
        assert_eq!(layout.storage_index([0, 0, 4]), None);
    }
    #[test]
    fn dense_first_layout() {
        let shape = ArrayShape::new([2, 3, 4]);
        let layout = ArrayLayout::dense_first(shape).unwrap();
        assert_eq!(layout.strides(), &[1, 2, 6]);
        assert_eq!(layout.element_count(), 24);
        assert_eq!(layout.required_storage_len(), 24);
        assert_eq!(layout.storage_index([0, 0, 0]), Some(0));
        assert_eq!(layout.storage_index([1, 0, 0]), Some(1));
        assert_eq!(layout.storage_index([0, 1, 0]), Some(2));
        assert_eq!(layout.storage_index([0, 0, 1]), Some(6));
        assert_eq!(layout.storage_index([1, 2, 3]), Some(23));
    }
    #[test]
    fn empty_layout() {
        let shape = ArrayShape::new([2, 0, usize::MAX]);
        let last = ArrayLayout::dense_last(shape).unwrap();
        let first = ArrayLayout::dense_first(shape).unwrap();
        for layout in [last, first] {
            assert!(layout.is_empty());
            assert_eq!(layout.element_count(), 0);
            assert_eq!(layout.strides(), &[0, 0, 0]);
            assert_eq!(layout.required_storage_len(), 0);
            assert_eq!(layout.storage_index([0, 0, 0]), None);
        }
    }
    #[test]
    fn scalar_layout() {
        let shape = ArrayShape::<0>::new([]);
        let last = ArrayLayout::dense_last(shape).unwrap();
        let first = ArrayLayout::dense_first(shape).unwrap();
        for layout in [last, first] {
            assert!(!layout.is_empty());
            assert_eq!(layout.strides(), &[]);
            assert_eq!(layout.element_count(), 1);
            assert_eq!(layout.required_storage_len(), 1);
            assert_eq!(layout.storage_index([]), Some(0));
        }
    }
    #[test]
    fn signed_address_boundary() {
        let maximum_len = isize::MAX as usize + 1;
        let maximum_shape = ArrayShape::new([maximum_len]);
        let last = ArrayLayout::dense_last(maximum_shape).unwrap();
        let first = ArrayLayout::dense_first(maximum_shape).unwrap();
        for layout in [last, first] {
            assert_eq!(layout.required_storage_len(), maximum_len);
            assert_eq!(layout.storage_index([isize::MAX as usize]), Some(isize::MAX as usize));
        }
        let too_large = ArrayShape::new([maximum_len + 1]);
        assert!(ArrayLayout::dense_last(too_large).is_err());
        assert!(ArrayLayout::dense_first(too_large).is_err());
    }
    #[test]
    fn layout_into_parts() {
        let shape = ArrayShape::new([2, 3]);
        let layout = ArrayLayout::dense_last(shape).unwrap();
        let (part_shape, offset, strides) = layout.into_parts();
        assert_eq!(part_shape, shape);
        assert_eq!(offset, 0);
        assert_eq!(strides, [3, 1]);
    }
}
