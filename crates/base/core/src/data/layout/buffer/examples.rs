// devela_base_core::data::layout::buffer::examples
//
// TOC
// - BufferExample
// - BufferViewExample
//
// NOTES
// - doclinks use #method notation to ensure they wont link to /zall version.

use crate::buffer_linear;

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// An owned linear buffer over contiguous storage, made with [`buffer_linear!`].
    #[doc = crate::_doc_location!("data/layout")]
    ///
    /// # Methods
    ///
    /// - [common](#impl-BufferExample<T,+S>)
    ///   - Constants:
    ///     [`CAP`](#associatedconstant.CAP) *([_PRIM](#associatedconstant.CAP_PRIM))*.
    ///   - Queries:
    ///     [`len`](#method.len) *([_prim](#method.len_prim))*,
    ///     [`is_empty`](#method.is_empty).
    ///
    /// - [fully initialized array](#impl-BufferOwnedExample<'_,+T,+[T;+CAP]>)
    /// (`array`)
    ///   - ...
    ///
    /// - [partially initialized array](#impl-BufferOwnedExample<'_,+T,+[MaybeUninit<T>;+CAP]>)
    /// (`uninit`)<sup title="unsafe implementation">⚠</sup>
    ///   - ...
    ///
    /// - [fully initialized array of options](#impl-BufferOwnedExample<'_,+T,+[Option<T>;+CAP]>)
    /// (`option`)
    ///   - Deconstructors:
    ///     [`clear`](#method.clear),
    ///     [`truncate`](#method.truncate).
    ///   - Push:
    ///     [`push_back`](#method.push_back) *([_copy](#method.push_back_copy))*.
    ///   - Pop:
    ///     [`pop_back`](#method.pop_back).
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [`get`](#method.get) *([_mut](#method.get_mut))*.
    ///   - Swap:
    ///     [`swap_remove`](#method.swap_remove) *([_copy](#method.swap_remove_copy))*.
    ///   - Views:
    ///     [`as_slice`](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    ///     [`iter`](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [`visit_slice`](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    pub struct BufferExample: owned (crate::NonValueU8<{u8::MAX}>);
    array,
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
    uninit,
    option,
);

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// A linear buffer view over contiguous storage, made with [`buffer_linear!`].
    #[doc = crate::_doc_location!("data/layout")]
    ///
    /// # Methods
    ///
    /// - [common](#impl-BufferViewExample<'a,+T,+S>)
    ///   - Queries:
    ///     [`len`](#method.len) *([_prim](#method.len_prim))*,
    ///     [`is_empty`](#method.is_empty).
    ///
    /// - [exclusive slice](#impl-BufferViewExample<'a,+T,+%26mut+[T]>) (`slice`)
    ///   - Constructors:
    ///     [`try_new`](#method.try_new),
    ///     [`new_truncated`](#method.new_truncated),
    ///     [`from_slice_with`](#method.from_slice_with),
    ///     [`try_from_slice`](#method.try_from_slice),
    ///     [`from_slice_truncated`](#method.from_slice_truncated).
    ///   - Queries:
    ///     [`capacity`](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [`is_full`](#method.is_full).
    ///
    /// - [shared slice](#impl-BufferViewExample<'a,+T,+%26[T]>) (`slice_mut`)
    ///   - ...
    pub struct BufferViewExample: view (crate::NonValueU8<{u8::MAX}>);
    slice_mut,
    slice,
);
