// devela/src/data/layout/array/backing/fixed.rs
//
//! Array implementations over fixed native arrays.
//

use super::validate_storage_len;
use crate::{Array, ArrayLayout, MismatchedCapacity, Slice};

/// # Methods over a fixed native array.
#[rustfmt::skip]
impl<T, const LEN: usize, const RANK: usize> Array<[T; LEN], RANK> {
    /* constructors */

    /// Creates an owning array over the fixed native `storage`.
    ///
    /// Extra backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover
    /// every physical position addressed by `layout`.
    pub fn try_from_array(
        storage: [T; LEN],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        match validate_storage_len(LEN, layout) {
            Ok(()) => Ok(Self { data: storage, layout }),
            Err(error) => Err(error),
        }
    }
    /// Creates an owning array over a fixed native backing array in const contexts.
    ///
    /// This is the `T: Copy` counterpart of [`try_from_array`][Self::try_from_array].
    /// Extra backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover every
    /// physical position addressed by `layout`.
    pub const fn try_from_array_copy(
        storage: [T; LEN],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> where T: Copy {
        match validate_storage_len(LEN, layout) {
            Ok(()) => Ok(Self { data: storage, layout }),
            Err(error) => Err(error),
        }
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    ///
    /// This is not necessarily the array's logical ravel order.
    pub const fn storage(&self) -> &[T] {
        &self.data
    }
    /// Returns the complete exclusive physical backing slice.
    pub const fn storage_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    /// Returns exclusive access to the fixed native backing array.
    ///
    /// Unlike dynamically sized backing storage,
    /// exposing this value cannot change the physical storage length.
    pub const fn data_mut(&mut self) -> &mut [T; LEN] {
        &mut self.data
    }
    /// Returns the backing-storage length.
    pub const fn storage_len(&self) -> usize {
        LEN
    }

    /* logical access */

    /// Returns a shared reference to the element at `coord`.
    ///
    /// Returns `None` if the coordinate is outside the logical shape.
    pub const fn get(&self, coord: [usize; RANK]) -> Option<&T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get(&self.data, index),
            None => None,
        }
    }
    /// Returns an exclusive reference to the element at `coord`.
    ///
    /// Returns `None` if the coordinate is outside the logical shape.
    pub const fn get_mut(&mut self, coord: [usize; RANK]) -> Option<&mut T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get_mut(&mut self.data, index),
            None => None,
        }
    }

    /* reborrowing */

    /// Returns a shared slice-backed array reborrowed from this array.
    pub const fn reborrow(&self) -> Array<&[T], RANK> {
        Array {
            data: &self.data,
            layout: self.layout,
        }
    }
    /// Returns an exclusive slice-backed array reborrowed from this array.
    pub const fn reborrow_mut(&mut self) -> Array<&mut [T], RANK> {
        Array {
            data: &mut self.data,
            layout: self.layout,
        }
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{ArrayShape, StringU8, const_assert};

    const SHAPE: ArrayShape<2> = ArrayShape::new([2, 3]);
    const LAYOUT: ArrayLayout<2> = match ArrayLayout::dense_last(SHAPE) {
        Ok(layout) => layout,
        Err(_) => panic!("unexpected layout overflow"),
    };
    const FIXED: Array<[u8; 6], 2> = match Array::try_from_array_copy([0, 1, 2, 3, 4, 5], LAYOUT) {
        Ok(array) => array,
        Err(_) => panic!("insufficient storage"),
    };
    const FIXED_VALUE: Option<u8> = FIXED.get([1, 2]).copied();
    const MUTATED_DATA: [u8; 6] = const_mutated_data();
    const fn const_mutated_data() -> [u8; 6] {
        let mut array = match Array::try_from_array_copy([0, 1, 2, 3, 4, 5], LAYOUT) {
            Ok(array) => array,
            Err(_) => panic!("insufficient storage"),
        };
        match array.get_mut([1, 2]) {
            Some(value) => *value = 9,
            None => panic!("valid coordinate rejected"),
        }
        *array.data()
    }

    #[test]
    const fn const_shared_access() {
        const_assert!(eq FIXED_VALUE.unwrap(), 5);
    }
    #[test]
    const fn const_exclusive_access() {
        const_assert!(eq_buf & MUTATED_DATA, &[0u8, 1, 2, 3, 4, 9]);
    }
    #[test]
    fn constructor_infers_fixed_backing() {
        let array = Array::try_from_array([0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        let _: Array<[u8; 6], 2> = array;
    }
    #[test]
    fn fixed_array_queries() {
        let array = Array::try_from_array([0, 1, 2, 3, 4, 5, 6, 7], LAYOUT).unwrap();
        assert_eq!(array.rank(), 2);
        assert_eq!(array.shape(), SHAPE);
        assert_eq!(array.layout(), LAYOUT);
        assert_eq!(array.element_count(), 6);
        assert!(!array.is_empty());
        assert_eq!(array.storage_len(), 8);
        assert_eq!(array.storage(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(array.data(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(array.get([0, 0]).copied(), Some(0));
        assert_eq!(array.get([0, 2]).copied(), Some(2));
        assert_eq!(array.get([1, 0]).copied(), Some(3));
        assert_eq!(array.get([1, 2]).copied(), Some(5));
        assert_eq!(array.get([2, 0]), None);
        assert_eq!(array.get([0, 3]), None);
    }
    #[test]
    fn rejects_short_fixed_storage() {
        let error = Array::try_from_array([0, 1, 2, 3, 4], LAYOUT).unwrap_err();
        assert_eq!(error, MismatchedCapacity::too_small(5, 6));
        assert_eq!(error.missing(), Some(1));
    }
    #[test]
    fn exposes_safe_mutable_fixed_backing() {
        let mut array = Array::try_from_array([0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        array.data_mut()[0] = 8;
        array.storage_mut()[1] = 7;
        *array.get_mut([1, 2]).unwrap() = 9;
        assert_eq!(array.into_data(), [8, 7, 2, 3, 4, 9]);
    }
    #[test]
    fn reborrows_fixed_storage() {
        let mut array = Array::try_from_array([0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        {
            let shared = array.reborrow();
            let _: Array<&[u8], 2> = shared;
            assert_eq!(shared.get([1, 2]).copied(), Some(5));
        }
        {
            let mut exclusive = array.reborrow_mut();
            let _: &mut Array<&mut [u8], 2> = &mut exclusive;
            *exclusive.get_mut([0, 0]).unwrap() = 9;
        }
        assert_eq!(array.get([0, 0]).copied(), Some(9));
    }
    #[test]
    fn shared_access_does_not_require_copy() {
        #[derive(Debug, PartialEq, Eq)]
        struct Token(u8);
        let layout = ArrayLayout::dense_last(ArrayShape::new([2])).unwrap();
        let array = Array::try_from_array([Token(7), Token(8)], layout).unwrap();
        assert_eq!(array.get([0]), Some(&Token(7)));
        assert_eq!(array.get([1]), Some(&Token(8)));
    }
    #[test]
    fn ownership_recovery_preserves_non_copy_values() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2])).unwrap();
        let array = Array::try_from_array(
            [StringU8::<5>::from_str("left").unwrap(), StringU8::from_str("right").unwrap()],
            layout,
        )
        .unwrap();
        let (storage, recovered_layout) = array.into_parts();
        assert_eq!(storage[0], "left");
        assert_eq!(storage[1], "right");
        assert_eq!(recovered_layout, layout);
    }
    #[test]
    fn scalar_fixed_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::<0>::new([])).unwrap();
        let array = Array::try_from_array([42], layout).unwrap();
        assert_eq!(array.element_count(), 1);
        assert_eq!(array.get([]).copied(), Some(42));
    }
    #[test]
    fn empty_fixed_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2, 0, 3])).unwrap();
        let array = Array::<[u8; 0], 3>::try_from_array([], layout).unwrap();
        assert!(array.is_empty());
        assert_eq!(array.storage_len(), 0);
        assert_eq!(array.get([0, 0, 0]), None);
    }
}
