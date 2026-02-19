// devela_base_core::data::layout::buffer::examples
//
// TOC
// - BufferStaticExample
// - BufferViewExample
//
// NOTES
// - doclinks use #method notation to ensure they wont link to /zall version.

use crate::buffer_linear;

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// A static linear buffer over contiguous storage, made with [`buffer_linear!`].
    #[doc = crate::_doc_location!("data/layout")]
    ///
    /// # Methods
    ///
    /// - [common methods](#impl-BufferStaticExample<T,+S>)
    // %common_tracked:
    ///   - Size:
    ///     [`len`](#method.len) *([_prim](#method.len_prim))*,
    ///     [`is_empty`](#method.is_empty).
    //
    ///
    /// - [Fully initialized array](#impl-BufferStaticExample<T,+[T;+CAP]>)
    /// (`array`)
    ///   - Constructors:
    ///     [`new`](#method.new) *([_init](#method.new_init))*,
    ///     [`from_array_clamped`](#method.from_array_clamped)
    ///       *([_prim](#method.from_array_clamped_prim))*,
    ///     [`from_slice_clone`](#method.from_slice_clone),
    ///     [`from_slice_copy`](#method.from_slice_copy),
    ///     [`from_slice_move_default`](#method.from_slice_move_default).
    // %common_static:
    ///   - Capacity:
    ///     [`CAP`](#associatedconstant.CAP) *([_PRIM](#associatedconstant.CAP_PRIM))*,
    ///     [`capacity`](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [`is_full`](#method.is_full).
    //
    ///   - Logical range control:
    ///     [`clear`](#method.clear),
    ///     [`truncate`](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Push:
    ///     [`push_back`](#method.push_back) *([_copy](#method.push_back_copy))*.
    ///   - Pop:
    ///     [`pop_back_clone`](#method.pop_back_clone),
    ///     [`pop_back_copy`](#method.pop_back_copy).
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [`get`](#method.get) *([_mut](#method.get_mut))*.
    ///   - Take:
    ///     [`take_default`](#method.take_default),
    ///     [`take_init`](#method.take_init),
    ///     [`take_with`](#method.take_with), *([_copy](#method.take_with_copy))*.
    ///   - Views:
    ///     [`as_slice`](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [`iter`](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [`visit_slice`](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    //
    // --------------------------------------------------------------------------
    /// - [Partially initialized array](#impl-BufferStaticExample<T,+[MaybeUninit<T>;+CAP]>)
    /// (`uninit`)<sup title="unsafe implementation">⚠</sup>
    ///   - Constructors:
    ///     [`new`](#method.new-1),
    ///     [`from_array_exact`](#method.from_array_exact)
    ///     [`from_array_unchecked`](#method.from_array_unchecked)<sup title="unsafe method">⚠</sup>,
    ///     [`from_slice_clone`](#method.from_slice_clone-1),
    ///     [`from_slice_copy`](#method.from_slice_copy-1),
    ///     [`from_slice_move_default`](#method.from_slice_move_default-1),
    ///     [`from_slice_move_init`](#method.from_slice_move_init-1).
    // %common_static:
    ///   - Capacity:
    ///     [`CAP`](#associatedconstant.CAP-1) *([_PRIM](#associatedconstant.CAP_PRIM-1))*,
    ///     [`capacity`](#method.capacity-1) *([_prim](#method.capacity_prim-1))*,
    ///     [`is_full`](#method.is_full-1).
    //
    ///   - Logical range control:
    ///     [`clear`](#method.clear-1),
    ///     [`drop_back`](#method.drop_back),
    ///     [`truncate`](#method.truncate-1), *([_prim](#method.truncate_prim-1))*.
    ///   - Push:
    ///     [`push_back`](#method.push_back-1).
    ///   - Pop:
    ///     [`pop_back`](#method.pop_back).
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back-1) *([_mut](#method.peek_mut_back-1))*.
    ///   - Get:
    ///     [`get`](#method.get-1) *([_mut](#method.get_mut-1))*.
    ///   - Swap:
    ///     [`swap_remove`](#method.swap_remove), *([_copy](#method.swap_remove_copy))*.
    ///   - Views:
    ///     [`as_slice`](#method.as_slice-2) *([_mut](#method.as_mut_slice-2))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [`iter`](#method.iter-2) *([_mut](#method.iter_mut-2))*.
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each-2) *([_mut](#method.visit_each_mut-2))*,
    ///     [`visit_slice`](#method.visit_slice-2) *([_mut](#method.visit_mut_slice-2))*.
    //
    // --------------------------------------------------------------------------
    /// - [Fully initialized array of options](#impl-BufferStaticExample<T,+[Option<T>;+CAP]>)
    /// (`option`)
    ///   - Constructors:
    ///     [`new`](#method.new-2),
    ///     [`from_array_clone`](#method.from_array_clone),
    ///     [`from_array_copy`](#method.from_array_copy),
    ///     [`from_array_unchecked`](#method.from_array_unchecked-1)<sup title="unsafe method">⚠</sup>,
    ///     [`from_array_linear`](#method.from_array_linear),
    ///     [`from_array_prefix`](#method.from_array_prefix),
    ///     [`from_slice_clone`](#method.from_slice_clone-2),
    ///     [`from_slice_copy`](#method.from_slice_copy-2),
    ///     [`from_slice_move_default`](#method.from_slice_move_default-2),
    ///     [`from_slice_move_init`](#method.from_slice_move_init-2).
    // %common_static:
    ///   - Capacity:
    ///     [`CAP`](#associatedconstant.CAP-2) *([_PRIM](#associatedconstant.CAP_PRIM-2))*,
    ///     [`capacity`](#method.capacity-2) *([_prim](#method.capacity_prim-2))*,
    ///     [`is_full`](#method.is_full-2).
    //
    ///   - Logical range control:
    ///     [`clear`](#method.clear-2),
    ///     [`truncate`](#method.truncate-2).
    ///   - Push:
    ///     [`push_back`](#method.push_back-2) *([_copy](#method.push_back_copy-1))*.
    ///   - Pop:
    ///     [`pop_back`](#method.pop_back-1).
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back-2) *([_mut](#method.peek_mut_back-2))*.
    ///   - Get:
    ///     [`get`](#method.get-2) *([_mut](#method.get_mut-2))*.
    ///   - Swap:
    ///     [`swap_remove`](#method.swap_remove) *([_copy](#method.swap_remove_copy))*.
    ///   - Views:
    ///     [`as_slice`](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    ///     [`iter`](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [`visit_slice`](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    pub struct BufferStaticExample: static (crate::NonValueU8<{u8::MAX}>);
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
    /// - [common methods](#impl-BufferViewExample<'a,+T,+S>)
    // %common_tracked:
    ///   - Size:
    ///     [`len`](#method.len) *([_prim](#method.len_prim))*,
    ///     [`is_empty`](#method.is_empty).
    ///
    /// - [Exclusive slice](#impl-BufferViewExample<'a,+T,+%26mut+[T]>) (`slice_mut`)
    ///   - Constructors:
    ///     [`try_new`](#method.try_new),
    ///     [`new_truncated`](#method.new_truncated)
    ///     [`try_from_slice`](#method.try_from_slice),
    ///     [`from_slice_with`](#method.from_slice_with),
    ///     [`from_slice_truncated`](#method.from_slice_truncated).
    // %common_view:
    ///   - Capacity:
    ///     [`capacity`](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [`is_full`](#method.is_full).
    //
    ///   - Logical range control:
    ///     [`clear`](#method.clear),
    ///     [`truncate`](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Push:
    ///     [`push_back`](#method.push_back) *([_copy](#method.push_back_copy))*.
    ///   - Pop:
    ///     [`pop_back_clone`](#method.pop_back_clone),
    ///     [`pop_back_copy`](#method.pop_back_copy).
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [`get`](#method.get) *([_mut](#method.get_mut))*.
    ///   - Views:
    ///     [`as_slice`](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [`iter`](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [`visit_slice`](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    //
    // --------------------------------------------------------------------------
    /// - [Shared slice](#impl-BufferViewExample<'a,+T,+%26[T]>) (`slice`)
    ///   - Constructors:
    ///     [`try_from_slice`](#method.try_from_slice-1),
    ///     [`from_slice_with`](#method.from_slice_with-1),
    ///     [`from_slice_truncated`](#method.from_slice_truncated-1).
    // %common_view:
    ///   - Capacity:
    ///     [`capacity`](#method.capacity-1) *([_prim](#method.capacity_prim-1))*,
    ///     [`is_full`](#method.is_full-1).
    //
    ///   - Peek:
    ///     [`peek_back`](#method.peek_back-1).
    ///   - Get:
    ///     [`get`](#method.get-1).
    ///   - Views:
    ///     [`as_slice`](#method.as_slice-1).
    // %common_iter_visit:
    ///   - Iteration:
    ///     [`iter`](#method.iter-1).
    ///   - Visitation:
    ///     [`visit_each`](#method.visit_each-1),
    ///     [`visit_slice`](#method.visit_slice-1).
    pub struct BufferViewExample: view (crate::NonValueU8<{u8::MAX}>);
    slice_mut,
    slice,
);
