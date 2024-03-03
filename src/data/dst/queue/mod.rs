// devela::data::dst::queue
//
//! Implementation of the FIFO queue structure.
//

use super::{DstArray, DstBuf};
use crate::code::PhantomData;

mod impl_traits;
mod methods;
mod private;

/// A statically allocated FIFO queue of <abbr title="Dynamically sized type">DST</abbr>s.
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

/// A statically allocated FIFO queue of <abbr title="Dynamically sized
/// type">DST</abbr>s with pointer alignment.
///
/// # Examples
/// ```
/// # use devela::data::DstQueueUsize;
/// let mut queue = DstQueueUsize::<[u8], 16>::new();
/// queue.push_copied(&[1]);
/// ```
// WAIT: [lazy_type_alias](https://github.com/rust-lang/rust/issues/112792) ↓DENIED
pub type DstQueueUsize<DST /*: ?Sized*/, const N: usize> = DstQueue<DST, DstArray<usize, N>>;

/// Handle returned by [`DstQueue::pop`][DstQueue#method.pop]
/// (does the actual pop on drop).
pub struct DstQueuePopHandle<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> {
    parent: &'a mut DstQueue<DST, BUF>,
}

/// An iterator over the elements of a [`DstQueue`].
pub struct DstQueueIter<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(&'a DstQueue<DST, BUF>, usize);

/// A mutable iterator over the elements of a [`DstQueue`].
pub struct DstQueueIterMut<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf>(
    &'a mut DstQueue<DST, BUF>,
    usize,
);
