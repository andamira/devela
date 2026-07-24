// devela/src/data/layout/array/carrier/slice.rs

use crate::{ArrayLayout, ArrayShape, ArrayView, MismatchedCapacity, Slice};

/// # Methods over a shared slice.
impl<'a, T, const RANK: usize> ArrayView<&'a [T], RANK> {
    /// Creates a shared view over `storage`.
    ///
    /// Extra backing elements are permitted and remain visible through
    /// [`storage`][Self::storage].
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the storage is too short to contain
    /// every position addressed by `layout`.
    pub const fn try_from_slice(
        storage: &'a [T],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        let required = layout.required_storage_len();
        if storage.len() < required {
            Err(MismatchedCapacity::too_small(storage.len(), required))
        } else {
            Ok(Self { data: storage, layout })
        }
    }
    /// Creates a shared slice-backed view.
    ///
    /// This inference-friendly constructor is equivalent to
    /// [`try_from_slice`][Self::try_from_slice], but allows the data carrier
    /// and rank to be inferred from `storage` and `layout`.
    pub const fn try_from_slice_ref(
        storage: &'a [T],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        Self::try_from_slice(storage, layout)
    }
    /// Returns the complete physical backing slice.
    ///
    /// This is not necessarily the array's logical ravel order.
    #[inline(always)]
    pub const fn storage(&self) -> &'a [T] {
        self.data
    }
    /// Returns the backing-storage length.
    #[inline(always)]
    pub const fn storage_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the element at `coord`, or `None` if it is out of bounds.
    pub const fn get(&self, coord: [usize; RANK]) -> Option<&'a T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get(self.data, index),
            None => None,
        }
    }
    /// Returns a copy of the element at `coord`.
    ///
    /// Returns `None` if the coordinate is out of bounds.
    pub const fn get_copy(&self, coord: [usize; RANK]) -> Option<T>
    where
        T: Copy,
    {
        match self.get(coord) {
            Some(value) => Some(*value),
            None => None,
        }
    }
    /// Returns a shared view reborrowed for the lifetime of `self`.
    #[inline(always)]
    pub const fn reborrow(&self) -> ArrayView<&[T], RANK> {
        ArrayView { data: self.data, layout: self.layout }
    }
    /// Decomposes the view into its backing slice and layout.
    #[inline(always)]
    pub const fn into_parts(self) -> (&'a [T], ArrayLayout<RANK>) {
        (self.data, self.layout)
    }
}

/// # Methods over an exclusive slice.
impl<'a, T, const RANK: usize> ArrayView<&'a mut [T], RANK> {
    /// Creates an exclusive view over `storage`.
    ///
    /// Extra backing elements are permitted and remain accessible through
    /// [`storage_mut`][Self::storage_mut].
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if the storage is too short to contain
    /// every position addressed by `layout`.
    pub const fn try_from_slice(
        storage: &'a mut [T],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        let required = layout.required_storage_len();
        if storage.len() < required {
            Err(MismatchedCapacity::too_small(storage.len(), required))
        } else {
            Ok(Self { data: storage, layout })
        }
    }
    /// Creates an exclusive slice-backed view.
    ///
    /// This inference-friendly constructor is equivalent to
    /// [`try_from_slice`][Self::try_from_slice], but allows the data carrier
    /// and rank to be inferred from `storage` and `layout`.
    pub const fn try_from_slice_mut(
        storage: &'a mut [T],
        layout: ArrayLayout<RANK>,
    ) -> Result<Self, MismatchedCapacity> {
        Self::try_from_slice(storage, layout)
    }
    /// Returns the complete physical backing slice.
    #[inline(always)]
    pub const fn storage(&self) -> &[T] {
        &*self.data
    }
    /// Returns the complete exclusive physical backing slice.
    #[inline(always)]
    pub const fn storage_mut(&mut self) -> &mut [T] {
        &mut *self.data
    }
    /// Returns the backing-storage length.
    #[inline(always)]
    pub const fn storage_len(&self) -> usize {
        self.data.len()
    }
    /// Returns a shared reference to the element at `coord`.
    pub const fn get(&self, coord: [usize; RANK]) -> Option<&T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get(&*self.data, index),
            None => None,
        }
    }
    /// Returns an exclusive reference to the element at `coord`.
    pub const fn get_mut(&mut self, coord: [usize; RANK]) -> Option<&mut T> {
        match self.layout.storage_index(coord) {
            Some(index) => Slice::get_mut(&mut *self.data, index),
            None => None,
        }
    }
    /// Returns a copy of the element at `coord`.
    pub const fn get_copy(&self, coord: [usize; RANK]) -> Option<T>
    where
        T: Copy,
    {
        match self.get(coord) {
            Some(value) => Some(*value),
            None => None,
        }
    }
    /// Returns a shared view reborrowed for the lifetime of `self`.
    #[inline(always)]
    pub const fn reborrow(&self) -> ArrayView<&[T], RANK> {
        ArrayView { data: &*self.data, layout: self.layout }
    }
    /// Returns an exclusive view reborrowed for the lifetime of `self`.
    #[inline(always)]
    pub const fn reborrow_mut(&mut self) -> ArrayView<&mut [T], RANK> {
        ArrayView { data: &mut *self.data, layout: self.layout }
    }
    /// Consumes the exclusive view and returns a shared view.
    #[inline(always)]
    pub const fn into_shared(self) -> ArrayView<&'a [T], RANK> {
        ArrayView { data: self.data, layout: self.layout }
    }
    /// Decomposes the view into its backing slice and layout.
    #[inline(always)]
    pub const fn into_parts(self) -> (&'a mut [T], ArrayLayout<RANK>) {
        (self.data, self.layout)
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Order, const_assert};

    const SHAPE: ArrayShape<2> = ArrayShape::new([2, 3]);
    const LAYOUT: ArrayLayout<2> = match ArrayLayout::dense_last(SHAPE) {
        Ok(layout) => layout,
        Err(_) => panic!("unexpected layout overflow"),
    };
    const SHARED_DATA: &[u8] = &[0, 1, 2, 3, 4, 5];
    const SHARED_VIEW: ArrayView<&[u8], 2> =
        match ArrayView::<&[u8], 2>::try_from_slice(SHARED_DATA, LAYOUT) {
            Ok(view) => view,
            Err(_) => panic!("insufficient storage"),
        };
    const SHARED_VALUE: Option<u8> = SHARED_VIEW.get_copy([1, 2]);

    const MUTATED_DATA: [u8; 6] = const_mutated_data();
    const fn const_mutated_data() -> [u8; 6] {
        let mut data = [0, 1, 2, 3, 4, 5];
        {
            let mut view = match ArrayView::<&mut [u8], 2>::try_from_slice(&mut data, LAYOUT) {
                Ok(view) => view,
                Err(_) => panic!("insufficient storage"),
            };
            match view.get_mut([1, 2]) {
                Some(value) => *value = 9,
                None => panic!("valid coordinate rejected"),
            }
        }
        data
    }

    #[test]
    const fn const_shared_access() {
        const_assert!(eq SHARED_VALUE.unwrap(), 5);
    }
    #[test]
    const fn const_exclusive_access() {
        const_assert!(eq_buf & MUTATED_DATA, &[0u8, 1, 2, 3, 4, 9]);
    }
    #[test]
    fn constructors_infer_slice_view_types() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2, 3])).unwrap();
        let storage = [0, 1, 2, 3, 4, 5];
        let shared = ArrayView::try_from_slice_ref(&storage, layout).unwrap();

        // These assignments prove the fully inferred types.
        let _: ArrayView<&[u8], 2> = shared;
        let mut storage = [0, 1, 2, 3, 4, 5];
        let exclusive = ArrayView::try_from_slice_mut(&mut storage, layout).unwrap();
        let _: ArrayView<&mut [u8], 2> = exclusive;
    }
    #[test]
    fn shared_view_queries() {
        let storage = [0, 1, 2, 3, 4, 5, 6, 7];
        let view = ArrayView::<&[u8], 2>::try_from_slice(&storage, LAYOUT).unwrap();
        assert_eq!(view.rank(), 2);
        assert_eq!(view.shape(), SHAPE);
        assert_eq!(view.layout(), LAYOUT);
        assert_eq!(view.element_count(), 6);
        assert!(!view.is_empty());
        assert_eq!(view.storage_len(), 8);
        assert_eq!(view.storage(), &storage);
        assert_eq!(*view.data(), &storage);
        assert_eq!(view.get_copy([0, 0]), Some(0));
        assert_eq!(view.get_copy([0, 2]), Some(2));
        assert_eq!(view.get_copy([1, 0]), Some(3));
        assert_eq!(view.get_copy([1, 2]), Some(5));
        assert_eq!(view.get([2, 0]), None);
        assert_eq!(view.get([0, 3]), None);
    }
    #[test]
    fn shared_view_rejects_short_storage() {
        let storage = [0, 1, 2, 3, 4];
        let err = ArrayView::<&[u8], 2>::try_from_slice(&storage, LAYOUT).unwrap_err();
        assert_eq!(err, MismatchedCapacity::too_small(5, 6));
        assert_eq!(err.missing(), Some(1));
    }
    #[test]
    fn view_accepts_exact_storage() {
        let storage = [0, 1, 2, 3, 4, 5];
        let view = ArrayView::<&[u8], 2>::try_from_slice(&storage, LAYOUT).unwrap();
        assert_eq!(view.storage_len(), 6);
        assert_eq!(view.get_copy([1, 2]), Some(5));
    }
    #[test]
    fn scalar_view() {
        let layout = ArrayLayout::dense_last(ArrayShape::<0>::new([])).unwrap();
        let storage = [42];
        let view = ArrayView::<&[u8], 0>::try_from_slice(&storage, layout).unwrap();
        assert_eq!(view.rank(), 0);
        assert_eq!(view.element_count(), 1);
        assert!(!view.is_empty());
        assert_eq!(view.get_copy([]), Some(42));
        let empty: [u8; 0] = [];
        assert!(ArrayView::<&[u8], 0>::try_from_slice(&empty, layout).is_err());
    }
    #[test]
    fn empty_view_requires_no_storage() {
        let layout = ArrayLayout::dense_last(ArrayShape::new([2, 0, 3])).unwrap();
        let storage: [u8; 0] = [];
        let view = ArrayView::<&[u8], 3>::try_from_slice(&storage, layout).unwrap();
        assert!(view.is_empty());
        assert_eq!(view.element_count(), 0);
        assert_eq!(view.storage_len(), 0);
        assert_eq!(view.get([0, 0, 0]), None);
    }
    #[test]
    fn shared_access_does_not_require_copy() {
        #[derive(Debug, PartialEq, Eq)]
        struct Token(u8);
        let storage = [Token(7)];
        let layout = ArrayLayout::dense_last(ArrayShape::new([1])).unwrap();
        let view = ArrayView::<&[Token], 1>::try_from_slice(&storage, layout).unwrap();
        assert_eq!(view.get([0]), Some(&Token(7)));
    }
    #[test]
    fn exclusive_view_access_and_reborrowing() {
        let mut storage = [0, 1, 2, 3, 4, 5];
        {
            let mut view = ArrayView::<&mut [u8], 2>::try_from_slice(&mut storage, LAYOUT).unwrap();
            assert_eq!(view.get_copy([1, 2]), Some(5));
            *view.get_mut([1, 2]).unwrap() = 9;
            {
                let shared = view.reborrow();
                assert_eq!(shared.get_copy([1, 2]), Some(9));
            }
            {
                let mut exclusive = view.reborrow_mut();
                *exclusive.get_mut([0, 0]).unwrap() = 8;
            }
            let shared = view.into_shared();
            assert_eq!(shared.get_copy([0, 0]), Some(8));
            assert_eq!(shared.get_copy([1, 2]), Some(9));
        }
        assert_eq!(storage, [8, 1, 2, 3, 4, 9]);
    }
    #[test]
    fn shared_into_parts() {
        let storage = [0, 1, 2, 3, 4, 5];
        let view = ArrayView::<&[u8], 2>::try_from_slice(&storage, LAYOUT).unwrap();
        let (part_storage, part_layout) = view.into_parts();
        assert_eq!(part_storage, &storage);
        assert_eq!(part_layout, LAYOUT);
    }
    #[test]
    fn exclusive_into_parts() {
        let mut storage = [0, 1, 2, 3, 4, 5];
        {
            let view = ArrayView::<&mut [u8], 2>::try_from_slice(&mut storage, LAYOUT).unwrap();
            let (part_storage, part_layout) = view.into_parts();
            part_storage[0] = 9;
            assert_eq!(part_layout, LAYOUT);
        }
        assert_eq!(storage, [9, 1, 2, 3, 4, 5]);
    }
    #[test]
    fn equivalence_with_order() {
        let dims = [4, 3, 2];
        assert_eq!(ArrayLayout::dense_first(ArrayShape::new(dims)).unwrap().strides(), &[1, 4, 12]);
        assert_eq!(Order::row_major_strides(dims), Some([1, 4, 12]));
        assert_eq!(ArrayLayout::dense_last(ArrayShape::new(dims)).unwrap().strides(), &[6, 2, 1]);
        assert_eq!(Order::col_major_strides(dims), Some([6, 2, 1]));
    }
}
