// devela::data::list::queue::destaque::definitions
//
//! Double-ended queues are linear lists for which any accesses are made from
//! either end.
//

use crate::{Array, Bare, Storage};
// #[cfg(feature = "dep_rkyv")] // DEP_DISABLED
// use rkyv::{Archive, Deserialize, Serialize};

/* types */

#[doc = crate::_tags!(data_structure)]
/// A static double-ended queue and stack backed by an [`Array`].
#[doc = crate::_doc_location!("data/list/queue")]
///
/// It is generic in respect to its
/// elements (`T`),
/// capacity (`CAP`),
/// index size (`IDX`)
/// and storage (`S`).
///
/// The index size will upper-bound the capacity to the maximum for that type,
/// e.g. `u8::MAX` for [`DestaqueU8`].
///
/// The index size determines the maximum possible number of elements in the destaque,
/// thereby upper-bounding the capacity to the maximum value representable by the
/// index type. For example, `u8::MAX` for [`DestaqueU8`].
///
/// The total size in bytes of the stack may be influenced by the chosen index
/// size, depending on the size and alignment of the elements. This difference
/// could only be significant for small capacities, as only one index is stored.
///
/// See also the related aliases that specify `IDX`:
#[cfg_attr(not(feature = "_destaque_all"), allow(rustdoc::broken_intra_doc_links))]
/// [`DestaqueU8`], [`DestaqueU16`], [`DestaqueU32`], [`DestaqueUsize`],
/// and the related traits:
/// [`DataQueue`][crate::DataQueue], [`DataDeque`][crate::DataDeque],
/// [`DataStack`][crate::DataStack], [`DataDesta`][crate::DataDesta].<br/>
///
/// ## Methods
///
/// It implements methods that operate from both the front and the back.
/// Rememeber that a single-ended *stack* operates only from the back, while a
/// single-ended *queue* pushes to the back and pops from the front.
///
/// - General methods:
///   - [`new`][Self::new],
/// [`len`][Self::len], [`is_empty`][Self::is_empty], [`is_full`][Self::is_full],
/// [`clear`][Self::clear], [`contains`][Self::contains],
/// [`capacity`][Self::capacity], [`remaining_capacity`][Self::remaining_capacity].
///   - [`iter`][Self::iter],
/// [`extend_back`][Self::extend_back], [`extend_front`][Self::extend_front],
/// [`from_array`][Self::from_array]*([`copy`][Self::from_array_copy])*,
/// [`to_array`][Self::to_array],
/// [`to_vec`][Self::to_vec].
///
/// - Queue and stack methods:
///   - push:
/// [`push_back`][Self::push_back]*([uc][Self::push_back_unchecked])*
///   **=** [`enqueue`][Self::enqueue],
/// [`push_front`][Self::push_front]*([uc][Self::push_front_unchecked])*.
///   - pop:
/// [`pop_front`][Self::pop_front]
///   **=** [`dequeue`][Self::dequeue],
/// [`pop_back`][Self::pop_back].
///   - peek:
/// [`peek_back`][Self::peek_back]*([mut][Self::peek_back_mut])*
/// [`peek_nth_back`][Self::peek_nth_back]*([mut][Self::peek_nth_back_mut])*,
/// [`peek_front`][Self::peek_front]*([mut][Self::peek_front_mut])*,
/// [`peek_nth_front`][Self::peek_nth_front]*([mut][Self::peek_nth_front_mut])*.
///   - drop:
/// [`drop_back`][Self::drop_back],
/// [`drop_front`][Self::drop_front],
/// [`drop_n_back`][Self::drop_n_back],
/// [`drop_n_front`][Self::drop_n_front].
///   - swap:
/// [`swap_back`][Self::swap_back]*([uc][Self::swap_back_unchecked])*,
/// [`swap_front`][Self::swap_front]*([uc][Self::swap_front_unchecked])*,
/// [`swap2_back`][Self::swap2_back]*([uc][Self::swap2_back_unchecked])*,
/// [`swap2_front`][Self::swap2_front]*([uc][Self::swap2_front_unchecked])*,
/// [`swap_ends`][Self::swap_ends], [`swap2_ends`][Self::swap2_ends].
///   - rot:
/// [`rot_right`][Self::rot_right],
/// [`rot_right_n`][Self::rot_right_n],
/// [`rot_left`][Self::rot_left],
/// [`rot_left_n`][Self::rot_left_n].
///   - dup:
/// [`dup_back`][Self::dup_back],
/// [`dup_front`][Self::dup_front],
/// [`dup2_back`][Self::dup2_back],
/// [`dup2_front`][Self::dup2_front].
///   - over:
/// [`over_back`][Self::over_back],
/// [`over_front`][Self::over_front],
/// [`over2_back`][Self::over2_back],
/// [`over2_front`][Self::over2_front].
///   - tuck:
/// [`tuck_back`][Self::tuck_back],
/// [`tuck_front`][Self::tuck_front],
/// [`tuck2_back`][Self::tuck2_back],
/// [`tuck2_front`][Self::tuck2_front].
// #[cfg_attr(feature = "dep_rkyv", derive(Archive, Serialize, Deserialize))]
pub struct Destaque<T, const CAP: usize, IDX, S: Storage = Bare> {
    pub(super) data: Array<T, CAP, S>,
    pub(super) len: IDX,
    pub(super) front: IDX,
    pub(super) back: IDX,
}

#[doc = crate::_tags!(data_structure)]
/// A [`Destaque`] with an 8-bit index size.
#[doc = crate::_doc_location!("data/list/queue")]
#[cfg(feature = "_destaque_u8")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u8")))]
pub type DestaqueU8<T, const CAP: usize, S = Bare> = Destaque<T, CAP, u8, S>;

#[doc = crate::_tags!(data_structure)]
/// A [`Destaque`] with a 16-bit index size.
#[doc = crate::_doc_location!("data/list/queue")]
#[cfg(feature = "_destaque_u16")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u16")))]
pub type DestaqueU16<T, const CAP: usize, S = Bare> = Destaque<T, CAP, u16, S>;

#[doc = crate::_tags!(data_structure)]
/// A [`Destaque`] with a 32-bit index size.
#[doc = crate::_doc_location!("data/list/queue")]
#[cfg(feature = "_destaque_u32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_u32")))]
pub type DestaqueU32<T, const CAP: usize, S = Bare> = Destaque<T, CAP, u32, S>;

#[doc = crate::_tags!(data_structure)]
/// A [`Destaque`] with a pointer-sized index size.
#[doc = crate::_doc_location!("data/list/queue")]
#[cfg(feature = "_destaque_usize")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_destaque_usize")))]
pub type DestaqueUsize<T, const CAP: usize, S = Bare> = Destaque<T, CAP, usize, S>;

/* iterators */

#[doc = crate::_tags!(iterator)]
/// An iterator over [`Destaque`] elements.
#[doc = crate::_doc_location!("data/list/queue")]
#[allow(missing_debug_implementations, reason = "unsatisfied trait bounds")]
pub struct DestaqueIter<'s, T, const CAP: usize, IDX, S: Storage = Bare> {
    pub(super) destaque: &'s Destaque<T, CAP, IDX, S>,
    pub(super) idx: usize,
}
