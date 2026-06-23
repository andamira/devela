// devela/src/data/layout/buffer/ring/examples.rs
//
// TOC
// - BufferRingStaticExample

use crate::buffer_ring;

buffer_ring!(
    #[doc = crate::_tags!(example data_structure)]
    /// A static ring buffer over contiguous storage, made with [`buffer_ring!`].
    #[doc = crate::_doc_meta!{location("data/layout/buffer")}]
    ///
    /// # Methods
    ///
    /// - [common methods](#impl-BufferRingStaticExample<T,+S>)
    // %common_tracked:
    ///   - Size:
    ///     [len](#method.len) *([_prim](#method.len_prim))*,
    ///     [is_empty](#method.is_empty).
    //
    // WIP
    ///
    /// - [Fully initialized array](#impl-BufferRingStaticExample<T,+[T;+CAP]>)
    /// (`array`)
    ///   - Constructors:
    ///     [new_init](#method.new_init),
    ///     from_array *([_empty](#method.from_array_empty), [_full](#method.from_array_full),
    ///       [_ring](#method.from_array_ring), [_ring_prim](#method.from_array_ring_prim))*,
    ///     from_slice *([_clone](#method.from_slice_clone), [_copy](#method.from_slice_copy))*.
    // %common_static:
    ///   - Capacity:
    ///     [CAP](#associatedconstant.CAP) *([_PRIM](#associatedconstant.CAP_PRIM))*,
    ///     [capacity](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [remaining_capacity](#method.remaining_capacity)
    ///       *([_prim](#method.remaining_capacity_prim))*,
    ///     [is_full](#method.is_full).
    //
    ///   - Logical range control:
    ///     [clear](#method.clear) *([_copy](#method.clear_copy))*,
    ///     [truncate](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Spare:
    ///     [spare_back_slice_mut](#method.spare_back_slice_mut),
    ///     [commit_back_slice](#method.commit_back) *([_prim](#method.commit_back_slice_prim))*.
    ///   - Push:
    ///     [push_back](#method.push_back) *([_copy](#method.push_back_copy))*,
    ///     [push_back_slice](#method.push_back_slice) *([_copy](#method.push_back_slice_copy),
    ///       [_copy_exact](#method.push_back_slice_copy_exact))*,
    ///     [push_front](#method.push_front) *([_copy](#method.push_front_copy))*,
    ///     [push_front_slice](#method.push_front_slice) *([_copy](#method.push_front_slice_copy),
    ///       [_copy_exact](#method.push_front_slice_copy_exact))*.
    ///   - Pop:
    ///     pop_front *([_clone](#method.pop_front_clone), [_copy](#method.pop_front_copy),
    ///       [_with](#method.pop_front_with), [_default](#method.pop_front_default),
    ///       [_copy_with](#method.pop_front_copy_with) [_init](#method.pop_front_init))*,
    ///     pop_back *([_clone](#method.pop_back_clone), [_copy](#method.pop_back_copy),
    ///       [_with](#method.pop_back_with), [_default](#method.pop_back_default),
    ///       [_copy_with](#method.pop_back_copy_with) [_init](#method.pop_back_init))*.
    ///   - Peek:
    ///     [peek_front](#method.peek_front) *([_mut](#method.peek_mut_front))*,
    ///     [peek_back](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [get](#method.get) *([_prim](#method.get_prim),
    ///       [_mut](#method.get_mut), [_mut_prim](#method.get_mut_prim))*.
    ///   - Take:
    ///     [take_with](#method.take_with), *([_prim](#method.take_with_prim))*,
    ///     [take_default](#method.take_default), *([_prim](#method.take_default_prim))*,
    ///     [take_copy_with](#method.take_copy_with), *([_prim](#method.take_copy_with_prim))*,
    ///     [take_init](#method.take_init), *([_prim](#method.take_init_prim))*.
    ///   - Swap:
    ///     [swap_remove_with](#method.swap_remove_with) *([_prim](#method.swap_remove_with_prim))*,
    ///     [swap_remove_default](#method.swap_remove_default) *([_prim](#method.swap_remove_default_prim))*,
    ///     [swap_remove_copy](#method.swap_remove_copy) *([_prim](#method.swap_remove_copy_prim))*,
    ///     [swap_remove_copy_with](#method.swap_remove_copy_with) *([_prim](#method.swap_remove_copy_with_prim))*,
    ///     [swap_remove_init](#method.swap_remove_init) *([_prim](#method.swap_remove_init_prim))*.
    ///   - Views:
    ///     [as_slices](#method.as_slices) *([_mut](#method.as_mut_slices))*.
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slices](#method.visit_slices) *([_mut](#method.visit_mut_slices))*.
    //
    // --------------------------------------------------------------------------
    /*
    /// - [Partially initialized array](#impl-BufferRingStaticExample<T,+[MaybeUninit<T>;+CAP]>)
    /// (`uninit`)<sup title="unsafe implementation">⚠</sup>
    ///   - Constructors:
    ///     [new](#method.new-1),
    ///     [from_array_exact](#method.from_array_exact)
    ///     [from_array_unchecked](#method.from_array_unchecked)<sup title="unsafe method">⚠</sup>,
    ///     [from_slice_clone](#method.from_slice_clone-1),
    ///     [from_slice_copy](#method.from_slice_copy-1),
    ///     [from_slice_move_default](#method.from_slice_move_default-1),
    ///     [from_slice_move_init](#method.from_slice_move_init-1).
    // %common_static:
    ///   - Capacity:
    ///     [CAP](#associatedconstant.CAP-1) *([_PRIM](#associatedconstant.CAP_PRIM-1))*,
    ///     [capacity](#method.capacity-1) *([_prim](#method.capacity_prim-1))*,
    ///     [remaining_capacity](#method.remaining_capacity-1)
    ///       *([_prim](#method.remaining_capacity_prim-1))*,
    ///     [is_full](#method.is_full-1).
    //
    ///   - Logical range control:
    ///     [clear](#method.clear-1),
    ///     [drop_back](#method.drop_back),
    ///     [truncate](#method.truncate-1), *([_prim](#method.truncate_prim-1))*.
    ///   - Push:
    ///     [push_back](#method.push_back-1),
    ///     [push_slice](#method.push_slice)
    ///       *([_copy](#method.push_slice_copy), [_copy_exact](#method.push_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_back](#method.pop_back).
    ///   - Peek:
    ///     [peek_back](#method.peek_back-1) *([_mut](#method.peek_mut_back-1))*.
    ///   - Get:
    ///     [get](#method.get-1) *([_mut](#method.get_mut-1))*.
    ///   - Swap:
    ///     [swap_remove](#method.swap_remove), *([_copy](#method.swap_remove_copy))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice-2) *([_mut](#method.as_mut_slice-2))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter-2) *([_mut](#method.iter_mut-2))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each-2) *([_mut](#method.visit_each_mut-2))*,
    ///     [visit_slice](#method.visit_slice-2) *([_mut](#method.visit_mut_slice-2))*.
    */
    //
    // --------------------------------------------------------------------------
    /// - [Fully initialized array of options](#impl-BufferRingStaticExample<T,+[Option<T>;+CAP]>)
    /// (`option`)
    ///   - Constructors:
    ///     [new_empty](#method.new_empty),
    ///     from_array *([_ring](#method.from_array_ring),
    ///       [_ring_prim](#method.from_array_ring_prim), [_linear](#method.from_array_linear),
    ///       [_clone](#method.from_array_clone), [_copy](#method.from_array_copy))*,
    ///     from_slice *([_clone](#method.from_slice_clone), [_copy](#method.from_slice_copy))*.
    // %common_static:
    ///   - Capacity:
    ///     [CAP](#associatedconstant.CAP-2) *([_PRIM](#associatedconstant.CAP_PRIM-2))*,
    ///     [capacity](#method.capacity-2) *([_prim](#method.capacity_prim-2))*,
    ///     [remaining_capacity](#method.remaining_capacity-2)
    ///       *([_prim](#method.remaining_capacity_prim-2))*,
    ///     [is_full](#method.is_full-2).
    //
    ///   - Logical range control:
    ///     [clear](#method.clear-2),
    ///     [truncate](#method.truncate-2).
    ///   - Push:
    ///     [push_back](#method.push_back-2) *([_copy](#method.push_back_copy-1))*,
    ///     [push_back_slice](#method.push_back_slice) *([_copy](#method.push_back_slice_copy),
    ///       [_copy_exact](#method.push_back_slice_copy_exact))*.
    ///     [push_front](#method.push_front-2) *([_copy](#method.push_front_copy-1))*,
    ///     [push_front_slice](#method.push_front_slice) *([_copy](#method.push_front_slice_copy),
    ///       [_copy_exact](#method.push_front_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_front](#method.pop_front-1) *([_copy](#method.pop_front_copy-1))*,
    ///     [pop_back](#method.pop_back-1) *([_copy](#method.pop_back_copy-1))*.
    ///   - Peek:
    ///     [peek_front](#method.peek_front-2) *([_mut](#method.peek_mut_front-2))*,
    ///     [peek_back](#method.peek_back-2) *([_mut](#method.peek_mut_back-2))*.
    ///   - Get:
    ///     [get](#method.get-2) *([_prim](#method.get_prim-2),
    ///       [_mut](#method.get_mut-2), [_mut_prim](#method.get_mut_prim-2))*.
    ///   - Swap:
    ///     [swap_remove](#method.swap_remove) *([_prim](#method.swap_remove_prim),
    ///       [_copy](#method.swap_remove_copy), [_copy_prim](#method.swap_remove_copy_prim))*.
    ///   - Views:
    ///     [as_slices](#method.as_slices-1) *([_mut](#method.as_mut_slices-1))*.
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slices](#method.visit_slices) *([_mut](#method.visit_mut_slices))*.
    pub struct BufferRingStaticExample: static (crate::NonValueU8<{u8::MAX}>);
    array,
    // #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
    // uninit, // TODO
    option,
);
