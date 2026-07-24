// devela/src/data/layout/array/define.rs
//
//! Array views over generic data carriers.
//

use crate::{ArrayLayout, ArrayShape};

#[doc = crate::_tags!(data_structure lifetime mem)]
/// A logical array interpretation over a data carrier.
#[doc = crate::_doc_meta!{location("data/layout/array")}]
///
/// An array view joins:
/// - accessible data of type `D`;
/// - an [`ArrayLayout`] describing how logical coordinates address it.
///
/// The view does not imply ownership. `D` may be a shared slice,
/// an exclusive slice, or another supported carrier.
///
/// # Example
/// ```
/// # use devela::{ArrayLayout, ArrayShape, Array};
/// let storage = [0, 1, 2, 3, 4, 5];
/// let shape = ArrayShape::new([2, 3]);
/// let layout = ArrayLayout::dense_last(shape)?;
/// let view = Array::try_from_slice_ref(&storage, layout)?;
///
/// assert_eq!(view.get_copy([1, 2]), Some(5));
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct Array<D, const RANK: usize> {
    pub(super) data: D,
    pub(super) layout: ArrayLayout<RANK>,
}
#[rustfmt::skip]
impl<D, const RANK: usize> Array<D, RANK> {
    /// Returns the underlying data carrier.
    pub const fn data(&self) -> &D { &self.data }

    /// Returns the array layout.
    pub const fn layout(&self) -> ArrayLayout<RANK> { self.layout }

    /// Returns the logical shape.
    pub const fn shape(&self) -> ArrayShape<RANK> { self.layout.shape() }

    /// Returns the number of logical axes.
    pub const fn rank(&self) -> usize { RANK }

    /// Returns the number of logical elements.
    pub const fn element_count(&self) -> usize { self.layout.element_count() }

    /// Returns whether the logical array has no elements.
    pub const fn is_empty(&self) -> bool { self.layout.is_empty() }
}
