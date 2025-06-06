// devela::data::list::stack::stack::definitions
//
//! Stacks ar linear lists for which all accesses are made from one end.
//!
//! <https://en.wikipedia.org/wiki/Stack_(abstract_data_type)>
//

use crate::{Array, Bare, Storage};
// #[cfg(feature = "dep_rkyv")] // DEP_DISABLED
// use rkyv::{Archive, Deserialize, Serialize};

/* types */

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A static stack backed by an [`Array`].
///
/// It is generic in respect to its
/// elements (`T`),
/// capacity (`CAP`),
/// index size (`IDX`)
/// and storage (`S`).
///
/// The index size will upper-bound the capacity to the maximum for that type,
/// e.g. `u8::MAX` for [`StackU8`].
///
/// The index size determines the maximum possible number of elements in the stack,
/// thereby upper-bounding the capacity to the maximum value representable by the
/// index type. For example, `u8::MAX` for [`StackU8`].
///
/// The total size in bytes of the stack may be influenced by the chosen index
/// size, depending on the size and alignment of the elements. This difference
/// could only be significant for small capacities, as only one index is stored.
///
/// See also the related aliases that specify `IDX`:
#[cfg_attr(not(feature = "_stack_all"), allow(rustdoc::broken_intra_doc_links))]
/// [`StackU8`], [`StackU16`], [`StackU32`], [`StackUsize`],
/// and the [`DataStack`][crate::DataStack] trait.
///
/// ## Methods
///
/// All the stack operations are done from the back.
///
/// The methods are the same for all `IDX` sizes:
/// - [Methods for `StackU8`][Self#methods-for-stacku8]
/// - [Methods for `StackU16`][Self#methods-for-stacku16]
/// - [Methods for `StackU32`][Self#methods-for-stacku32]
/// - [Methods for `StackUsize`][Self#methods-for-stackusize]
///
/// The following list of methods links to the ones implemented for `StackU8`:
///
/// - Construct:
///   [`new`][Self::new],
///   [`new_copied`][Self::new_copied],
///   [`from_array`][Self::from_array]*([`copy`](Self::from_array_copy))*.
/// - Deconstruct:
///   [`to_array`][Self::to_array],
///   [`to_vec`][Self::to_vec]*(`alloc`)*.
///   [`as_slice`][Self::as_slice],
///   [`as_mut_slice`][Self::as_mut_slice],
/// - Query:
///   [`len`][Self::len], [`is_empty`][Self::is_empty], [`is_full`][Self::is_full],
///   [`capacity`][Self::capacity], [`remaining_capacity`][Self::remaining_capacity],
///   [`contains`][Self::contains].
/// - Resize:
///   [`resize_default`][Self::resize_default]*([own][Self::own_resize_default])*,
///   [`resize_default_truncate`][Self::resize_default_truncate]
///     *([own][Self::own_resize_default_truncate])*.
/// - Convert:
///   [`to_idx_u8`][Self::to_idx_u8]*([own][Self::own_to_idx_u8])*,
///   [`to_idx_u16`][Self::to_idx_u16]*([own][Self::own_to_idx_u16])*,
///   [`to_idx_u32`][Self::to_idx_u32]*([own][Self::own_to_idx_u32])*,
///   [`to_idx_usize`][Self::to_idx_usize]*([own][Self::own_to_idx_usize])*.
/// - Iterate:
///   [`iter`][Self::iter],
///   [`extend`][Self::extend],
///
/// - Stack operations without bounds on `T`:
///
///   - clear: [`clear`][Self::clear].
///   - push: [`push`][Self::push].
///   - pop: [`pop`][Self::pop] *(unsafe)*.
///   - peek:
///     [`peek`][Self::peek]*([mut][Self::peek_mut])*,
///     [`peek_nth`][Self::peek_nth]*([mut][Self::peek_nth_mut])*,
///   - drop:
///     [`drop`][Self::drop], [`drop_n`][Self::drop_n],
///     [`drop_replace_default`][Self::drop_replace_default].
///   - nip: [`nip`][Self::nip], [`nip2`][Self::nip2].
///   - swap: [`swap`][Self::swap], [`swap2`][Self::swap2],
///   - rot:
///     [`rot`][Self::rot], [`rot_cc`][Self::rot_cc],
///     [`rot2`][Self::rot2], [`rot2_cc`][Self::rot2_cc].
///
/// - Stack [operations depending on `T: Clone`](#operations-depending-on-t-clone).
///
///   - pop: [`pop`][Self::pop] *(safe)*.
///   - dup: [`dup`][Self::dup], [`dup2`][Self::dup2].
///   - over: [`over`][Self::over], [`over2`][Self::over2].
///   - tuck: [`tuck`][Self::tuck], [`tuck2`][Self::tuck2].
///
/// - Stack [chainable *const* operations depending on `T:
///   Copy`](#chainable-const-operations-depending-on-t-copy).
///
///   - clear: [`own_clear`][Self::own_clear].
///   - push: [`own_push`][Self::own_push]*([uc][Self::own_push_unchecked])*,
///   - pop: [`own_pop`][Self::own_pop]*([uc][Self::own_pop_unchecked])*.
///   - drop:
///     [`own_drop`][Self::own_drop]*([uc][Self::own_drop_unchecked])*,
///     [`own_drop_n`][Self::own_drop_n]*([uc][Self::own_drop_n_unchecked])*.
///   - nip:
///     [`own_nip`][Self::own_nip]*([uc][Self::own_nip_unchecked])*,
///     [`own_nip2`][Self::own_nip2]*([uc][Self::own_nip2_unchecked])*.
///   - swap:
///     [`own_swap`][Self::own_swap]*([uc][Self::own_swap_unchecked])*,
///     [`own_swap2`][Self::own_swap2]*([uc][Self::own_swap2_unchecked])*.
///   - rot:
///     [`own_rot`][Self::own_rot]*([uc][Self::own_rot_unchecked])*,
///     [`own_rot_cc`][Self::own_rot_cc]*([uc][Self::own_rot_cc_unchecked])*,
///     [`own_rot2`][Self::own_rot2]*([uc][Self::own_rot2_unchecked])*,
///     [`own_rot2_cc`][Self::own_rot2_cc]*([uc][Self::own_rot2_cc_unchecked])*.
///   - dup:
///     [`own_dup`][Self::own_dup]*([uc][Self::own_dup_unchecked])*,
///     [`own_dup2`][Self::own_dup2]*([uc][Self::own_dup2_unchecked])*.
///   - over:
///     [`own_over`][Self::own_over]*([uc][Self::own_over_unchecked])*,
///     [`own_over2`][Self::own_over2]*([uc][Self::own_over2_unchecked])*.
///   - tuck:
///     [`own_tuck`][Self::own_tuck]*([uc][Self::own_tuck_unchecked])*,
///     [`own_tuck2`][Self::own_tuck2]*([uc][Self::own_tuck2_unchecked])*.
#[must_use]
//  #[cfg_attr(feature = "dep_rkyv", derive(Archive, Serialize, Deserialize))]
pub struct Stack<T, const CAP: usize, IDX, S: Storage = Bare> {
    pub(super) data: Array<T, CAP, S>,
    pub(super) len: IDX,
}

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A [`Stack`] with an 8-bit index size.
#[cfg(feature = "_stack_u8")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_stack_u8")))]
pub type StackU8<T, const CAP: usize, S = Bare> = Stack<T, CAP, u8, S>;

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A [`Stack`] with a 16-bit index size.
#[cfg(feature = "_stack_u16")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_stack_u16")))]
pub type StackU16<T, const CAP: usize, S = Bare> = Stack<T, CAP, u16, S>;

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A [`Stack`] with a 32-bit index size.
#[cfg(feature = "_stack_u32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_stack_u32")))]
pub type StackU32<T, const CAP: usize, S = Bare> = Stack<T, CAP, u32, S>;

#[doc = crate::TAG_DATA_STRUCTURE!()]
/// A [`Stack`] with a pointer-sized index size.
#[cfg(feature = "_stack_usize")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_stack_usize")))]
pub type StackUsize<T, const CAP: usize, S = Bare> = Stack<T, CAP, usize, S>;

/* iterators */

#[doc = crate::TAG_ITERATOR!()]
/// An iterator over [`Stack`] elements.
#[must_use]
pub struct StackIter<'s, T, const CAP: usize, IDX, S: Storage = Bare> {
    pub(super) stack: &'s Stack<T, CAP, IDX, S>,
    pub(super) idx: usize,
}
