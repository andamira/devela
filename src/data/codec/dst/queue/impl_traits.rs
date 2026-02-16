// devela::data::codec::dst::queue_impl_traits
//
//!
//

use super::{DstBuf, DstQueue, DstQueueIter, DstQueueIterMut, DstQueuePopHandle};
use core::{fmt, iter, mem, ops};

/* queue */

impl<DST: ?Sized, BUF: DstBuf> ops::Drop for DstQueue<DST, BUF> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
impl<DST: ?Sized, BUF: DstBuf + Default> Default for DstQueue<DST, BUF> {
    fn default() -> Self {
        DstQueue::new()
    }
}

impl<BUF: DstBuf, DST: ?Sized + fmt::Debug> fmt::Debug for DstQueue<DST, BUF> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("[")?;
        for v in self.iter() {
            v.fmt(f)?;
            f.write_str(",")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

/* pop handle */

impl<DST: ?Sized, BUF: DstBuf> ops::Deref for DstQueuePopHandle<'_, DST, BUF> {
    type Target = DST;
    fn deref(&self) -> &DST {
        unsafe { &*self.parent.front_raw() }
    }
}
impl<DST: ?Sized, BUF: DstBuf> ops::DerefMut for DstQueuePopHandle<'_, DST, BUF> {
    fn deref_mut(&mut self) -> &mut DST {
        unsafe { &mut *self.parent.front_raw_mut() }
    }
}
impl<DST: ?Sized, BUF: DstBuf> ops::Drop for DstQueuePopHandle<'_, DST, BUF> {
    fn drop(&mut self) {
        self.parent.pop_front_inner();
    }
}

/* iter */

impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstQueueIter<'a, DST, BUF> {
    type Item = &'a DST;
    fn next(&mut self) -> Option<&'a DST> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFETY: Bounds checked, aliasing enforced by API.
            let rv = unsafe { &*self.0.raw_at(self.1) };
            self.1 +=
                DstQueue::<DST, BUF>::meta_words() + BUF::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}
impl<'a, DST: 'a + ?Sized, BUF: 'a + DstBuf> iter::Iterator for DstQueueIterMut<'a, DST, BUF> {
    type Item = &'a mut DST;
    fn next(&mut self) -> Option<&'a mut DST> {
        if self.1 == self.0.write_pos {
            None
        } else {
            // SAFETY: Bounds checked, aliasing enforced by API
            let rv = unsafe { &mut *self.0.raw_at_mut(self.1) };
            self.1 +=
                DstQueue::<DST, BUF>::meta_words() + BUF::round_to_words(mem::size_of_val(rv));
            Some(rv)
        }
    }
}
