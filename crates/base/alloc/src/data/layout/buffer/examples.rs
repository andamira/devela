// devela_base_alloc::data::layout::buffer::examples
//
// TOC
// - BufferAllocExample
//
// NOTES
// - doclinks use #method notation to ensure they wont link to /zall version.

use crate::buffer_linear;

buffer_linear!(
    #[doc = crate::_tags!(example data_structure)]
    /// An owned linear buffer over a [`Vec`], made with [`buffer_linear!`].
    #[doc = crate::_doc_location!("data/layout")]
    ///
    /// # Methods
    ///
    /// - [Dynamically sized array](#impl-BufferAllocExample<T,+Vec<T>>)
    /// (`vec`)
    ///   - Constructors:
    ///     [`new`](#method.new) *([_init](#method.new_init))*,
    ///     [`with_capacity`](#method.with_capacity) *([_prim](#method.with_capacity_prim))*.
    ///   - Size:
    ///     [`len`](#method.len) *([_prim](#method.len_prim))*,
    ///     [`is_empty`](#method.is_empty),
    ///   - Capacity:
    ///     [`capacity`](#method.capacity) *([_prim](#method.capacity_prim))*,
    ///     [`is_full`](#method.is_full).
    ///   - Logical range control:
    ///     [`clear`](#method.clear),
    ///     [`truncate`](#method.truncate), *([_prim](#method.truncate_prim))*.
    ///   - Push:
    ///     [`push_back`](#method.push_back).
    ///   - Pop:
    ///     [`pop_back`](#method.pop_back).
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
    pub struct BufferAllocExample: alloc (crate::NonValueU8<{u8::MAX}>);
    vec,
);
