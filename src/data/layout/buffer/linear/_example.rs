// devela/src/data/layout/buffer/linear/_example.rs
//
// TOC
// - BufferLinearStaticExample
// - BufferLinearViewExample
// - BufferLinearAllocExample
//
// NOTES
// - doclinks use #method notation to ensure they wont link to /all version.

use crate::buffer_linear;

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// A static linear buffer over contiguous storage, made with [`buffer_linear!`].
    #[doc = crate::_doc_meta!{
        location("data/layout/buffer"),
        test_size_of(__: BufferLinearStaticExample<(), [(); 8]> = 1|8),
        test_size_of(__: BufferLinearStaticExample<u8, [u8; 8]> = 9|72),
        test_size_of(__: BufferLinearStaticExample<i32, [i32; 8]> = 36|288),
        #[cfg(target_pointer_width = "32")]
        test_size_of(__: BufferLinearStaticExample<char, &[i32]> = 12|96),
        #[cfg(target_pointer_width = "64")]
        test_size_of(__: BufferLinearStaticExample<char, &[i32]> = 24|192),
    }]
    ///
    /// # Methods
    ///
    /// - [common methods](#impl-BufferLinearStaticExample<T,+S>)
    // %common_tracked:
    ///   - Size:
    ///     [len](#method.len) *([_prim](#method.len_prim))*,
    ///     [is_empty](#method.is_empty).
    //
    /// - [Fully initialized array](#impl-BufferLinearStaticExample<T,+[T;+CAP]>)
    /// (`array`)
    ///   - Constructors:
    ///     [new](#method.new) *([_init](#method.new_init))*,
    ///     from_array *([from_array_clamped](#method.from_array_clamped),
    ///       [_prim](#method.from_array_clamped_prim))*,
    ///     from_slice (*[_clone](#method.from_slice_clone),
    ///       [_copy](#method.from_slice_copy),
    ///       [_move_default](#method.from_slice_move_default))*.
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
    ///     [truncate](#method.truncate), *([_copy](#method.truncate_copy),
    ///       [_prim](#method.truncate_prim), [_prim_copy](#method.truncate_prim_copy))*.
    ///   - Push:
    ///     [push_back](#method.push_back) *([_copy](#method.push_back_copy))*,
    ///     [push_slice](#method.push_slice)
    ///       *([_copy](#method.push_slice_copy), [_copy_exact](#method.push_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_back_clone](#method.pop_back_clone),
    ///     [pop_back_copy](#method.pop_back_copy).
    ///   - Peek:
    ///     [peek_back](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [get](#method.get) *([_mut](#method.get_mut))*.
    ///   - Take:
    ///     [take_default](#method.take_default),
    ///     [take_init](#method.take_init),
    ///     [take_with](#method.take_with) *([_copy](#method.take_with_copy))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slice](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    //
    // --------------------------------------------------------------------------
    /// - [Partially initialized array](#impl-BufferLinearStaticExample<T,+[MaybeUninit<T>;+CAP]>)
    /// (`uninit`)<sup title="unsafe implementation">⚠</sup>
    ///   - Constructors:
    ///     [new](#method.new-1),
    ///     from_array *([_exact](#method.from_array_exact)
    ///       [_unchecked](#method.from_array_unchecked)<sup title="unsafe method">⚠</sup>)*,
    ///     from_slice (*[_clone](#method.from_slice_clone-1),
    ///       [_copy](#method.from_slice_copy-1),
    ///       [_move_default](#method.from_slice_move_default-1),
    ///       [_move_init](#method.from_slice_move_init-1))*.
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
    ///     [push_slice](#method.push_slice) *([_copy](#method.push_slice_copy),
    ///       [_copy_exact](#method.push_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_back](#method.pop_back).
    ///   - Peek:
    ///     [peek_back](#method.peek_back-1) *([_mut](#method.peek_mut_back-1))*.
    ///   - Get:
    ///     [get](#method.get-1) *([_mut](#method.get_mut-1))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice-2) *([_mut](#method.as_mut_slice-2))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter-2) *([_mut](#method.iter_mut-2))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each-2) *([_mut](#method.visit_each_mut-2))*,
    ///     [visit_slice](#method.visit_slice-2) *([_mut](#method.visit_mut_slice-2))*.
    //
    // --------------------------------------------------------------------------
    /// - [Fully initialized array of options](#impl-BufferLinearStaticExample<T,+[Option<T>;+CAP]>)
    /// (`option`)
    ///   - Constructors:
    ///     [new](#method.new-2),
    ///     from_array (*[_clone](#method.from_array_clone), [_copy](#method.from_array_copy),
    ///       [_unchecked](#method.from_array_unchecked-1)<sup title="unsafe method">⚠</sup>,
    ///       [_linear](#method.from_array_linear),
    ///       [_prefix](#method.from_array_prefix))*,
    ///     from_slice *([_clone](#method.from_slice_clone-2),
    ///       [_copy](#method.from_slice_copy-2),
    ///       [_move_default](#method.from_slice_move_default-2),
    ///       [_move_init](#method.from_slice_move_init-2))*.
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
    ///     [push_slice](#method.push_slice)
    ///       *([_copy](#method.push_slice_copy), [_copy_exact](#method.push_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_back](#method.pop_back-1).
    ///   - Peek:
    ///     [peek_back](#method.peek_back-2) *([_mut](#method.peek_mut_back-2))*.
    ///   - Get:
    ///     [get](#method.get-2) *([_mut](#method.get_mut-2))*.
    ///   - Swap:
    ///     [swap_remove](#method.swap_remove) *([_prim](#method.swap_remove_prim),
    ///       [_copy](#method.swap_remove_copy), [_copy_prim](#method.swap_remove_copy_prim))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slice](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    pub struct BufferLinearStaticExample: static (crate::NonValueU8<{u8::MAX}>);
    array,
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
    uninit,
    option,
);

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// A linear buffer view over contiguous storage, made with [buffer_linear!].
    #[doc = crate::_doc_meta!{location("data/layout/buffer")}]
    ///
    /// # Methods
    ///
    /// - [common methods](#impl-BufferLinearViewExample<'a,+T,+S>)
    // %common_tracked:
    ///   - Size:
    ///     [len](#method.len) *([_prim](#method.len_prim))*,
    ///     [is_empty](#method.is_empty).
    ///
    /// - [Exclusive slice](#impl-BufferLinearViewExample<'a,+T,+%26mut+[T]>) (slice_mut)
    ///   - Constructors:
    ///     [try_new](#method.try_new),
    ///     [new_truncated](#method.new_truncated)
    ///     [try_from_slice](#method.try_from_slice),
    ///     [from_slice_with](#method.from_slice_with),
    ///     [from_slice_truncated](#method.from_slice_truncated).
    // %common_view:
    ///   - Capacity:
    ///     [capacity](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [remaining_capacity](#method.remaining_capacity)
    ///       *([_prim](#method.remaining_capacity_prim))*,
    ///     [is_full](#method.is_full).
    //
    ///   - Logical range control:
    ///     [clear](#method.clear),
    ///     [truncate](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Push:
    ///     [push_back](#method.push_back) *([_copy](#method.push_back_copy))*,
    ///     [push_slice](#method.push_slice)
    ///       *([_copy](#method.push_slice_copy), [_copy_exact](#method.push_slice_copy_exact))*.
    ///   - Pop:
    ///     [pop_back_clone](#method.pop_back_clone),
    ///     [pop_back_copy](#method.pop_back_copy).
    ///   - Peek:
    ///     [peek_back](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [get](#method.get) *([_mut](#method.get_mut))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slice](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    //
    // --------------------------------------------------------------------------
    /// - [Shared slice](#impl-BufferLinearViewExample<'a,+T,+%26[T]>)
    /// (`slice`)
    ///   - Constructors:
    ///     [try_from_slice](#method.try_from_slice-1),
    ///     [from_slice_with](#method.from_slice_with-1),
    ///     [from_slice_truncated](#method.from_slice_truncated-1).
    // %common_view:
    ///   - Capacity:
    ///     [capacity](#method.capacity-1) *([_prim](#method.capacity_prim-1))*,
    ///     [remaining_capacity](#method.remaining_capacity-1)
    ///       *([_prim](#method.remaining_capacity_prim-1))*,
    ///     [is_full](#method.is_full-1).
    //
    ///   - Peek:
    ///     [peek_back](#method.peek_back-1).
    ///   - Get:
    ///     [get](#method.get-1).
    ///   - Views:
    ///     [as_slice](#method.as_slice-1).
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter-1).
    ///   - Visitation:
    ///     [visit_each](#method.visit_each-1),
    ///     [visit_slice](#method.visit_slice-1).
    pub struct BufferLinearViewExample: view (crate::NonValueU8<{u8::MAX}>);
    slice_mut,
    slice,
);

#[cfg(feature = "alloc")]
buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// An owned linear buffer over a [`Vec`], made with [`buffer_linear!`].
    #[doc = crate::_doc_meta!{location("data/layout/buffer")}]
    ///
    /// # Methods
    ///
    /// - [Dynamically sized array](#impl-BufferLinearAllocExample<T,+Vec<T>>)
    /// (`vec`)
    ///   - Constructors:
    ///     [new](#method.new) *([_init](#method.new_init))*,
    ///     [with_capacity](#method.with_capacity) *([_prim](#method.with_capacity_prim))*.
    ///   - Size:
    ///     [len](#method.len) *([_prim](#method.len_prim))*,
    ///     [is_empty](#method.is_empty),
    ///   - Capacity:
    ///     [capacity](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [remaining_capacity](#method.remaining_capacity)
    ///       *([_prim](#method.remaining_capacity_prim))*,
    ///     [is_full](#method.is_full).
    ///   - Logical range control:
    ///     [clear](#method.clear),
    ///     [truncate](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Push:
    ///     [push_back](#method.push_back),
    ///     [push_slice](#method.push_slice).
    ///   - Pop:
    ///     [pop_back](#method.pop_back).
    ///   - Peek:
    ///     [peek_back](#method.peek_back) *([_mut](#method.peek_mut_back))*.
    ///   - Get:
    ///     [get](#method.get) *([_mut](#method.get_mut))*.
    ///   - Take:
    ///     [take_default](#method.take_default),
    ///     [take_init](#method.take_init),
    ///     [take_with](#method.take_with), *([_copy](#method.take_with_copy))*.
    ///   - Views:
    ///     [as_slice](#method.as_slice) *([_mut](#method.as_mut_slice))*,
    // %common_iter_visit:
    ///   - Iteration:
    ///     [iter](#method.iter) *([_mut](#method.iter_mut))*.
    ///   - Visitation:
    ///     [visit_each](#method.visit_each) *([_mut](#method.visit_each_mut))*,
    ///     [visit_slice](#method.visit_slice) *([_mut](#method.visit_mut_slice))*.
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "_docs_examples")))]
    pub struct BufferLinearAllocExample: alloc (crate::NonValueU8<{u8::MAX}>);
    vec,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array() {
        let mut buf = BufferLinearStaticExample::<i32, [i32; 8]>::new_init();
        buf.push_back(10).unwrap();
        buf.push_back(20).unwrap();
        assert_eq!(buf.as_slice(), &[10, 20]);
    }
    #[test]
    fn option() {
        let mut buf = BufferLinearStaticExample::<i32, [Option<i32>; 8]>::new();
        buf.push_back(10).unwrap();
        buf.push_back(20).unwrap();
        assert_eq!(buf.as_slice(), &[Some(10), Some(20)]);
    }
    #[test]
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
    fn uninit() {
        let mut buf = BufferLinearStaticExample::<i32, [crate::MaybeUninit<i32>; 8]>::new();
        buf.push_back(10).unwrap();
        buf.push_back(20).unwrap();
        assert_eq!(buf.as_slice(), &[10, 20]);
    }
}
