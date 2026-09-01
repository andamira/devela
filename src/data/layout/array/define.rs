// devela/src/data/layout/array/define.rs
//
//! Array views over generic backing storage.
//

use crate::{ArrayCoordIter, ArrayLayout, ArrayShape};

#[doc = crate::_tags!(data_structure mem)]
/// A logical array over backing storage.
#[doc = crate::_doc_meta!{
    location("data/layout/array", struct Array),
    #[cfg(target_pointer_width = "32")]
    test_size_of(Array<[u8; 6], 1> = 20|160; niche !Option),
    #[cfg(target_pointer_width = "32")]
    test_size_of(Array<[u8; 6], 2> = 28|224; niche !Option),
    #[cfg(target_pointer_width = "32")]
    test_size_of(Array<[u8; 6], 3> = 36|288; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(Array<[u8; 6], 1> = 32|256; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(Array<[u8; 6], 2> = 48|384; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(Array<[u8; 6], 3> = 64|512; niche !Option),
}]
/// An array joins:
/// - backing data of type `D`;
/// - an [`ArrayLayout`] mapping logical coordinates into that data.
///
/// `D` determines whether the array borrows or owns its storage,
/// whether access is shared or exclusive, and whether the
/// storage length is fixed or dynamically determined.
///
/// # Invariant
///
/// Every physical storage position addressed by `layout`
/// must be accessible through `data`.
///
/// The provided constructors preserve this relationship.
///
/// # Examples
/// º
/// ```
/// # use devela::{ArrayLayout, ArrayShape, Array};
/// let storage = [0, 1, 2, 3, 4, 5];
/// let shape = ArrayShape::new([2, 3]);
/// let layout = ArrayLayout::dense_last(shape)?;
/// let view = Array::try_from_slice_ref(&storage, layout)?;
///
/// assert_eq!(view.get([1, 2]).copied(), Some(5));
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct Array<D, const RANK: usize> {
    /// The backing data providing physical element storage.
    ///
    /// Its accessible storage must cover every position addressed by `layout`.
    pub(super) data: D,

    /// The logical shape and coordinate-to-storage mapping.
    ///
    /// Every physical position addressed by this layout must be valid for `data`.
    pub(super) layout: ArrayLayout<RANK>,
}
#[rustfmt::skip]
impl<D, const RANK: usize> Array<D, RANK> {

    /// Returns a shared reference to the backing data.
    pub const fn data(&self) -> &D { &self.data }

    /// Returns the array layout.
    pub const fn layout(&self) -> ArrayLayout<RANK> { self.layout }

    /// Consumes the array and returns its backing data.
    pub fn into_data(self) -> D {
        self.data
    }
    /// Decomposes the array into its backing data and layout.
    pub fn into_parts(self) -> (D, ArrayLayout<RANK>) {
        (self.data, self.layout)
    }

    /// Returns the logical shape.
    pub const fn shape(&self) -> ArrayShape<RANK> { self.layout.shape() }

    /// Returns the number of logical axes.
    pub const fn rank(&self) -> usize { RANK }

    /// Returns the number of logical elements.
    pub const fn element_count(&self) -> usize { self.layout.element_count() }

    /// Returns whether the logical array has no elements.
    pub const fn is_empty(&self) -> bool { self.layout.is_empty() }

    /// Returns an iterator over every logical coordinate of this array.
    ///
    /// Coordinates are independent of physical offset, strides,
    /// backing-storage order, and ownership.
    pub const fn coords(&self) -> ArrayCoordIter<RANK> {
        self.layout.coords()
    }
}
