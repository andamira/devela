// devela::data::dst::queue
//
//! Implementation of the FIFO queue structure.
//

use super::{DstArray, DstBuf};
use crate::PhantomData;

mod impl_traits;
mod methods;
mod private;

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A statically allocated FIFO queue of <abbr title="Dynamically sized type">DST</abbr>s.
#[doc = crate::_doc!(location: "data/dst")]
///
/// # Examples
/// ```
/// # use devela::data::{DstArray, DstQueue};
/// let mut queue = DstQueue::<str, DstArray<usize, 8>>::new();
/// queue.push_back_str("Hello");
/// queue.push_back_str("World");
/// assert_eq!(queue.pop_front().as_ref().map(|v| &v[..]), Some("Hello"));
/// ```
pub struct DstQueue<DST: ?Sized, BUF: DstBuf> {
    _pd: PhantomData<*const DST>,
    read_pos: usize,
    write_pos: usize,
    data: BUF,
}

#[doc = crate::_TAG_DATA_STRUCTURE!()]
/// A statically allocated FIFO queue of <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
#[doc = crate::_doc!(location: "data/dst")]
///
/// # Examples
/// ```
/// # use devela::data::DstQueueUsize;
/// let mut queue = DstQueueUsize::<[u8], 16>::new();
/// queue.push_copied(&[1]);
/// ```
// WAIT: [lazy_type_alias](https://github.com/rust-lang/rust/issues/112792) ↓DENIED
pub type DstQueueUsize<DST /*: ?Sized*/, const CAP: usize> = DstQueue<DST, DstArray<usize, CAP>>;

/// Handle returned by [`DstQueue::pop`][DstQueue#method.pop]
/// (does the actual pop on drop).
#[doc = crate::_doc!(location: "data/dst")]
#[derive(Debug)]
pub struct DstQueuePopHandle<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> {
    parent: &'a mut DstQueue<DST, BUF>,
}

#[doc = crate::_TAG_ITERATOR!()]
/// An iterator over the elements of a [`DstQueue`].
#[doc = crate::_doc!(location: "data/dst")]
#[derive(Debug)]
pub struct DstQueueIter<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(&'a DstQueue<DST, BUF>, usize);

#[doc = crate::_TAG_ITERATOR!()]
/// A mutable iterator over the elements of a [`DstQueue`].
#[doc = crate::_doc!(location: "data/dst")]
#[derive(Debug)]
pub struct DstQueueIterMut<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(
    &'a mut DstQueue<DST, BUF>,
    usize,
);
