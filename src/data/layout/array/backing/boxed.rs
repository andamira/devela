// devela/data/layout/array/backing/boxed.rs
//
//! Array implementations over boxed slices.
//

use super::validate_storage_len;
use crate::{Array, ArrayLayout, Box, MismatchedCapacity, Slice, Vec};

/// # Methods over a boxed slice.
impl<T, const RANK: usize> Array<Box<[T]>, RANK> {
    /* constructors */

    /// Creates an owning array over the boxed slice `storage`.
    ///
    /// Extra initialized backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover
    /// every physical position addressed by `layout`.
    pub fn try_from_boxed_slice(
        storage: Box<[T]>,
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
        self.data.as_ref()
    }
    /// Returns the complete exclusive physical backing slice.
    pub fn storage_mut(&mut self) -> &mut [T] {
        self.data.as_mut()
    }
    /// Returns the backing-storage length.
    pub fn storage_len(&self) -> usize {
        self.data.len()
    }

    /* logical access */

    /// Returns a shared reference to the element at `coord`.
    pub fn get(&self, coord: [usize; RANK]) -> Option<&T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get(self.data.as_ref(), index),
            None => None,
        }
    }
    /// Returns an exclusive reference to the element at `coord`.
    pub fn get_mut(&mut self, coord: [usize; RANK]) -> Option<&mut T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get_mut(self.data.as_mut(), index),
            None => None,
        }
    }

    /* reborrowing */

    /// Returns a shared slice-backed array reborrowed from this array.
    pub fn reborrow(&self) -> Array<&[T], RANK> {
        Array { data: self.data.as_ref(), layout: self.layout }
    }
    /// Returns an exclusive slice-backed array reborrowed from this array.
    pub fn reborrow_mut(&mut self) -> Array<&mut [T], RANK> {
        Array { data: self.data.as_mut(), layout: self.layout }
    }

    /* converting */

    /// Converts this boxed-slice-backed array into a vector-backed array.
    ///
    /// The logical layout is preserved.
    pub fn into_vec(self) -> Array<Vec<T>, RANK> {
        let (data, layout) = self.into_parts();
        Array { data: Vec::from(data), layout }
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{ArrayShape, Vec, vec_ as vec};

    const SHAPE: ArrayShape<2> = ArrayShape::new([2, 3]);
    const LAYOUT: ArrayLayout<2> = match ArrayLayout::dense_last(SHAPE) {
        Ok(layout) => layout,
        Err(_) => panic!("unexpected layout overflow"),
    };

    #[test]
    fn constructor_infers_boxed_backing() {
        let storage = vec![0_u8, 1, 2, 3, 4, 5].into_boxed_slice();
        let array = Array::try_from_boxed_slice(storage, LAYOUT).unwrap();
        let _: Array<Box<[u8]>, 2> = array;
    }
    #[test]
    fn boxed_array_queries() {
        let storage = vec![0_u8, 1, 2, 3, 4, 5, 6, 7].into_boxed_slice();
        let array = Array::try_from_boxed_slice(storage, LAYOUT).unwrap();
        assert_eq!(array.storage_len(), 8);
        assert_eq!(array.storage(), &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(array.get([0, 0]).copied(), Some(0));
        assert_eq!(array.get([0, 2]).copied(), Some(2));
        assert_eq!(array.get([1, 0]).copied(), Some(3));
        assert_eq!(array.get([1, 2]).copied(), Some(5));
        assert_eq!(array.get([2, 0]), None);
        assert_eq!(array.get([0, 3]), None);
    }
    #[test]
    fn rejects_short_boxed_storage() {
        let storage = vec![0_u8, 1, 2, 3, 4].into_boxed_slice();
        let error = Array::try_from_boxed_slice(storage, LAYOUT).unwrap_err();
        assert_eq!(error, MismatchedCapacity::too_small(5, 6));
    }
    #[test]
    fn mutates_boxed_elements() {
        let storage = vec![0_u8, 1, 2, 3, 4, 5].into_boxed_slice();
        let mut array = Array::try_from_boxed_slice(storage, LAYOUT).unwrap();
        array.storage_mut()[0] = 8;
        *array.get_mut([1, 2]).unwrap() = 9;
        let storage = array.into_data();
        assert_eq!(storage.as_ref(), &[8, 1, 2, 3, 4, 9]);
    }
    #[test]
    fn reborrows_boxed_storage() {
        let storage = vec![0_u8, 1, 2, 3, 4, 5].into_boxed_slice();
        let mut array = Array::try_from_boxed_slice(storage, LAYOUT).unwrap();
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
    fn vec_boxed_roundtrip_preserves_layout() {
        let array = Array::try_from_vec(vec![0_u8, 1, 2, 3, 4, 5], LAYOUT).unwrap();
        let boxed = array.into_boxed();
        assert_eq!(boxed.layout(), LAYOUT);
        assert_eq!(boxed.storage(), &[0, 1, 2, 3, 4, 5]);
        let vector = boxed.into_vec();
        assert_eq!(vector.layout(), LAYOUT);
        assert_eq!(vector.storage(), &[0, 1, 2, 3, 4, 5]);
    }
    #[test]
    fn boxed_scalar_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::<0>::new([])).unwrap();
        let array = Array::try_from_boxed_slice(vec![42].into_boxed_slice(), layout).unwrap();
        assert_eq!(array.get([]).copied(), Some(42));
    }
    #[test]
    fn empty_boxed_array() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2, 0, 3])).unwrap();
        let array =
            Array::<Box<[u8]>, 3>::try_from_boxed_slice(Vec::new().into_boxed_slice(), layout)
                .unwrap();
        assert!(array.is_empty());
        assert_eq!(array.storage_len(), 0);
    }
}
