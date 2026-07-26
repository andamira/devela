// devela/src/data/layout/array/backing/vec.rs
//
//! Array implementations over vectors.
//

use super::validate_storage_len;
use crate::{Array, ArrayLayout, Box, MismatchedCapacity, Slice, Vec};

/// # Methods over a vector.
impl<T, const RANK: usize> Array<Vec<T>, RANK> {
    /* constructors */

    /// Creates an owning array over the vector `storage`.
    ///
    /// Extra initialized backing elements are permitted.
    ///
    /// The vector's capacity is not considered accessible storage;
    /// only its initialized length must cover `layout`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover
    /// every physical position addressed by `layout`.
    pub fn try_from_vec(
        storage: Vec<T>,
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        match validate_storage_len(storage.len(), layout) {
            Ok(()) => Ok(Self { data: storage, layout }),
            Err(error) => Err(error),
        }
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    ///
    /// This is not necessarily the array's logical ravel order.
    pub fn storage(&self) -> &[T] {
        self.data.as_slice()
    }
    /// Returns the complete exclusive physical backing slice.
    ///
    /// This permits mutation of initialized elements without permitting
    /// changes to the vector's length.
    pub fn storage_mut(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }
    /// Returns the backing-storage length.
    pub fn storage_len(&self) -> usize {
        self.data.len()
    }

    /* logical access */

    /// Returns a shared reference to the element at `coord`.
    ///
    /// Returns `None` if the coordinate is outside the logical shape.
    pub fn get(&self, coord: [usize; RANK]) -> Option<&T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get(self.data.as_slice(), index),
            None => None,
        }
    }
    /// Returns an exclusive reference to the element at `coord`.
    ///
    /// Returns `None` if the coordinate is outside the logical shape.
    pub fn get_mut(&mut self, coord: [usize; RANK]) -> Option<&mut T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get_mut(self.data.as_mut_slice(), index),
            None => None,
        }
    }

    /* reborrowing */

    /// Returns a shared slice-backed array reborrowed from this array.
    pub fn reborrow(&self) -> Array<&[T], RANK> {
        Array { data: self.data.as_slice(), layout: self.layout }
    }
    /// Returns an exclusive slice-backed array reborrowed from this array.
    pub fn reborrow_mut(&mut self) -> Array<&mut [T], RANK> {
        Array {
            data: self.data.as_mut_slice(),
            layout: self.layout,
        }
    }

    /* converting */

    /// Converts this vector-backed array into a boxed-slice-backed array.
    ///
    /// The logical layout is preserved.
    pub fn into_boxed(self) -> Array<Box<[T]>, RANK> {
        let (data, layout) = self.into_parts();
        Array { data: data.into_boxed_slice(), layout }
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{ArrayShape, String, vec_ as vec};

    const SHAPE: ArrayShape<2> = ArrayShape::new([2, 3]);
    const LAYOUT: ArrayLayout<2> = match ArrayLayout::dense_last(SHAPE) {
        Ok(layout) => layout,
        Err(_) => panic!("unexpected layout overflow"),
    };

    #[test]
    fn constructor_infers_vec_backing() {
        let array = Array::try_from_vec(vec![0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        let _: Array<Vec<u8>, 2> = array;
    }
    #[test]
    fn vec_array_queries() {
        let array = Array::try_from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7], LAYOUT).unwrap();
        assert_eq!(array.rank(), 2);
        assert_eq!(array.shape(), SHAPE);
        assert_eq!(array.layout(), LAYOUT);
        assert_eq!(array.element_count(), 6);
        assert!(!array.is_empty());
        assert_eq!(array.storage_len(), 8);
        assert_eq!(array.storage(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(array.data().as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(array.get([0, 0]).copied(), Some(0));
        assert_eq!(array.get([0, 2]).copied(), Some(2));
        assert_eq!(array.get([1, 0]).copied(), Some(3));
        assert_eq!(array.get([1, 2]).copied(), Some(5));
        assert_eq!(array.get([2, 0]), None);
        assert_eq!(array.get([0, 3]), None);
    }
    #[test]
    fn rejects_short_vec_storage() {
        let error = Array::try_from_vec(vec![0, 1, 2, 3, 4], LAYOUT).unwrap_err();
        assert_eq!(error, MismatchedCapacity::too_small(5, 6));
        assert_eq!(error.missing(), Some(1));
    }
    #[test]
    fn capacity_does_not_count_as_storage() {
        let mut storage = Vec::with_capacity(16);
        storage.extend([0, 1, 2, 3, 4]);
        assert!(storage.capacity() >= 6);
        assert_eq!(storage.len(), 5);
        let error = Array::try_from_vec(storage, LAYOUT).unwrap_err();
        assert_eq!(error, MismatchedCapacity::too_small(5, 6));
    }
    #[test]
    fn mutates_vec_elements_without_exposing_length() {
        let mut array = Array::try_from_vec(vec![0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        array.storage_mut()[0] = 8;
        *array.get_mut([1, 2]).unwrap() = 9;
        assert_eq!(array.into_data(), vec![8, 1, 2, 3, 4, 9]);
    }
    #[test]
    fn reborrows_vec_storage() {
        let mut array = Array::try_from_vec(vec![0, 1, 2, 3, 4, 5], LAYOUT).unwrap();
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
    fn ownership_recovery_preserves_values_and_capacity() {
        let mut storage = Vec::with_capacity(16);
        storage.extend([String::from("left"), String::from("right")]);
        let capacity = storage.capacity();
        let layout = ArrayLayout::dense_last(ArrayShape::new([2])).unwrap();
        let array = Array::try_from_vec(storage, layout).unwrap();
        let (storage, recovered_layout) = array.into_parts();
        assert_eq!(storage[0], "left");
        assert_eq!(storage[1], "right");
        assert_eq!(storage.capacity(), capacity);
        assert_eq!(recovered_layout, layout);
    }
    #[test]
    fn scalar_vec_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::<0>::new([])).unwrap();
        let array = Array::try_from_vec(vec![42], layout).unwrap();
        assert_eq!(array.element_count(), 1);
        assert_eq!(array.get([]).copied(), Some(42));
    }
    #[test]
    fn empty_vec_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2, 0, 3])).unwrap();
        let array = Array::<Vec<u8>, 3>::try_from_vec(vec![], layout).unwrap();
        assert!(array.is_empty());
        assert_eq!(array.storage_len(), 0);
        assert_eq!(array.get([0, 0, 0]), None);
    }
}
