// devela_base_core::data::iter::lending
//
//! Defines [`IteratorLending`], [`IteratorLendingDoubleEnded`], [`IteratorLendingExactSize`],
//! [`IteratorLendingPeek`].
//

use crate::{_TAG_ITERATOR, is};

#[doc = _TAG_ITERATOR!()]
/// A lending iterator using a generic associated lifetime.
///
/// A lending iterator yields items borrowed from its own internal state.
/// Each call to [`next`][Self::next] creates a temporary borrow of `self`
/// and returns an item tied to that borrow. This enables iteration over
/// memory-backed structures without copying or owning the underlying data.
///
/// See also:
/// - [`IteratorLendingDoubleEnded`]
/// - [`IteratorLendingExactSize`]
/// - [`IteratorLendingPeek`]
#[rustfmt::skip]
pub trait IteratorLending {
    /// The item yielded by this iterator.
    ///
    /// The lifetime `'a` corresponds to the borrow of `self` required to produce the item.
    type Item<'a> where Self: 'a;

    /// Advances the iterator and returns the next item.
    ///
    /// The lifetime of the returned reference is scoped to the borrow
    /// of `self` established during this call.
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    /* non-required methods */

    /// Returns a size hint for the remaining items.
    ///
    /// The default implementation returns `(0, None)`, which is correct but
    /// conservative. Implementors may override this for improved accuracy.
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Consumes the iterator, returning the number of remaining items.
    fn count(&mut self) -> usize {
        let mut n = 0;
        while self.next().is_some() { n += 1; }
        n
    }

    // ✗ last

    /// Advances the iterator by `n` steps.
    ///
    /// Returns `Ok(())` if `n` items were successfully skipped.
    /// Returns `Err(remaining)` if the iterator ended early, where
    /// `remaining` is the number of items that were *not* skipped.
    fn advance_by(&mut self, mut n: usize) -> Result<(), usize> {
        while n > 0 {
            match self.next() {
                Some(_) => n -= 1,
                None => return Err(n),
            }
        }
        Ok(())
    }

    /// Advances the iterator by `n` items and returns the next one.
    fn nth<'a>(&'a mut self, mut n: usize) -> Option<Self::Item<'a>> {
        while n > 0 {
            is![self.next().is_none(); return None];
            n -= 1;
        }
        self.next()
    }

    // ✗ step_by
    // ✗ chain
    // ✗ zip
    // ✗ intersperse
    // ✗ intersperse_with
    // ✗ map

    /// Applies `f` to each remaining item.
    fn for_each<F>(&mut self, mut f: F)
    where F: FnMut(Self::Item<'_>) {
        while let Some(item) = self.next() { f(item); }
    }

    // ✗ filter
    // ✗ filter_map
    // ✗ enumerate
    // ✗ peekable
    // ✗ skip_while
    // ✗ take_while
    // ✗ map_while
    // ✓ skip
    // ✓ take
    // ✗ scan
    // ✗ flatmap
    // ✗ flatten
    // ✗ map_windows
    // ✗ fuse

    /// Calls `f` for each item before yielding it.
    ///
    /// This is useful for debugging or side effects. The iterator
    /// behavior is otherwise unchanged.
    fn inspect<F>(&mut self, mut f: F)
    where
        F: FnMut(&Self::Item<'_>),
    {
        while let Some(item) = self.next() {
            f(&item);
        }
    }

    // ✓ by_ref
    // ✗ collect
    // ✗ try_collect
    // ✗ collect_into
    // ✗ partition
    // ✗ partition_in_place

    /// Returns `true` if the iterator is partitioned according to `pred`.
    ///
    /// All items for which `pred` returns `true` must come before all
    /// items for which it returns `false`. Once the predicate becomes
    /// `false`, it must remain `false` for all subsequent items.
    fn is_partitioned<P>(&mut self, mut pred: P) -> bool
    where
        P: FnMut(&Self::Item<'_>) -> bool,
    {
        let mut seen_false = false;
        while let Some(item) = self.next() {
            if pred(&item) {
                if seen_false { return false; }
            } else {
                seen_false = true;
            }
        }
        true
    }

    /// Applies `f` to each item, accumulating results in `acc`.
    ///
    /// Iteration stops on the first `Err`. Returns `Ok(acc)` when all
    /// applications succeed.
    ///
    /// This mirrors `Iterator::try_fold` but works with borrowed items.
    fn try_fold<B, E, F>(&mut self, mut acc: B, mut f: F) -> Result<B, E>
    where
        F: FnMut(B, Self::Item<'_>) -> Result<B, E>,
    {
        while let Some(item) = self.next() {
            acc = f(acc, item)?;
        }
        Ok(acc)
    }

    /// Applies `f` to each remaining item until an error occurs.
    fn try_for_each<F, E>(&mut self, mut f: F) -> Result<(), E>
    where F: FnMut(Self::Item<'_>) -> Result<(), E> {
        while let Some(item) = self.next() { f(item)?; }
        Ok(())
    }

    // ✗ fold
    // ✓ reduce
    // ✓ try_reduce

    /// Returns `true` if all items satisfy the predicate.
    fn all<P>(&mut self, mut pred: P) -> bool
    where
        P: FnMut(&Self::Item<'_>) -> bool,
    {
        while let Some(item) = self.next() { is![!pred(&item); return false]; }
        true
    }
    /// Returns `true` if any item satisfies the predicate.
    fn any<P>(&mut self, mut pred: P) -> bool
    where
        P: FnMut(&Self::Item<'_>) -> bool,
    {
        while let Some(item) = self.next() { is![pred(&item); return true]; }
        false
    }

    // ✓ find
    // ✓ find_map
    // ✓ try_find

    /// Returns the index of the first item for which `pred` returns `true`,
    /// or `None` if no matching item is found.
    fn position<P>(&mut self, mut pred: P) -> Option<usize>
    where
        P: FnMut(&Self::Item<'_>) -> bool,
    {
        let mut index = 0;
        while let Some(item) = self.next() {
            is![pred(&item); return Some(index)];
            index += 1;
        }
        None
    }

    // ✓ rposition
    // ✓ max
    // ✓ min
    // ✓ max_by_key
    // ✓ max_by
    // ✓ min_by_key
    // ✓ min_by
    // ✓ rev (in double-ended)
    // ✗ unzip
    // ✗ copied
    // ✗ cloned
    // ✗ cycle
    // ✗ array_chunks

    // ~ sum
    // ~ product
    // ✓ cmp
    // ✓ cmp_by
    // ✓ partial_cmp
    // ✓ partial_cmp_by
    // ✓ eq
    // ✓ eq_by
    // ✓ ne
    // ✓ lt
    // ✓ le
    // ✓ gt
    // ✓ ge
    // ✓ is_sorted
    // ✓ is_sorted_by
    // ✓ is_sorted_by_key
}

#[doc = _TAG_ITERATOR!()]
/// A lending iterator that can yield items from the back.
///
/// This is the borrowing analogue of [`DoubleEndedIterator`].
pub trait IteratorLendingDoubleEnded: IteratorLending {
    /// Returns the next item from the back of the iterator.
    fn next_back<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    /* non-required methods*/

    /// Skips `n` items from the back and returns the next one.
    ///
    /// Repeatedly calls [`next_back`][Self::next_back] `n + 1` times.
    /// Returns the final yielded item, or `None` if the iterator ends before advancing `n` steps.
    ///
    /// This is the reverse analogue of [`IteratorLending::nth`].
    fn nth_back<'a>(&'a mut self, mut n: usize) -> Option<Self::Item<'a>> {
        while n > 0 {
            is![self.next_back().is_none(); return None];
            n -= 1;
        }
        self.next_back()
    }

    // ✓ rposition

    /// Reverse-direction equivalent of [`IteratorLending::try_fold`].
    ///
    /// Applies `f` starting from the back of the iterator, folding
    /// items in reverse order. Stops on first `Err`.
    fn try_rfold<B, E, F>(&mut self, mut acc: B, mut f: F) -> Result<B, E>
    where
        F: FnMut(B, Self::Item<'_>) -> Result<B, E>,
    {
        while let Some(item) = self.next_back() {
            acc = f(acc, item)?;
        }
        Ok(acc)
    }
}

#[doc = _TAG_ITERATOR!()]
/// A lending iterator with a known remaining length.
///
/// Types implementing this trait can report how many items are left
/// without consuming the iterator.
pub trait IteratorLendingExactSize: IteratorLending {
    /* non-required methods */

    /// Returns the exact number of items remaining.
    ///
    /// The default implementation derives this from `size_hint`, as per the
    /// standard library's `ExactSizeIterator`.
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        debug_assert!(upper == Some(lower));
        lower
    }

    /// Returns `true` when no more items can be produced.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[doc = _TAG_ITERATOR!()]
/// A lending iterator that can inspect the next item without advancing.
///
/// The returned reference is tied to the temporary mutable borrow of `self`
/// created by this call. Implementations must not modify the iteration state
/// (i.e., the next call to [`IteratorLending::next`] will still yield the same item, if any).
pub trait IteratorLendingPeek: IteratorLending {
    /// Returns the next item without advancing the iterator.
    ///
    /// Borrows `self` mutably so that both shared and exclusive lending
    /// iterators can expose a peek. The reference remains valid until
    /// `self` is borrowed again.
    fn peek<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    /* non-required */

    /// Returns the next item if it matches the predicate.
    ///
    /// The iterator is advanced only when `pred` returns `true` for the
    /// next item. Otherwise, the iterator remains unchanged.
    fn next_if<F>(&mut self, pred: F) -> Option<Self::Item<'_>>
    where
        F: FnOnce(&Self::Item<'_>) -> bool,
    {
        is![self.peek().is_some_and(|i| pred(&i)); self.next(); None]
    }

    /// Returns the next item if it is equal to `expected`.
    ///
    /// This is a convenience wrapper around [`next_if`][Self::next_if] using `PartialEq`.
    fn next_if_eq<Q>(&mut self, expected: &Q) -> Option<Self::Item<'_>>
    where
        for<'a> Self::Item<'a>: PartialEq<Q>,
        Q: ?Sized,
    {
        self.next_if(|item| item == expected)
    }

    /// Applies `f` to the next item without advancing the iterator unless `f` returns `Ok`.
    ///
    /// If `f` returns:
    /// - `Ok(r)` — the iterator is advanced and `Ok(Some(r))` is returned.
    /// - `Err(e)` — the iterator is left unchanged and `Err(e)` is returned.
    /// If the iterator is exhausted, `Ok(None)` is returned.
    ///
    /// `f` receives a temporary borrow of the next item. The borrow ends before
    /// this method calls [`IteratorLending::next`], so advancing the iterator is
    /// always sound.
    fn next_if_map<R, E, F>(&mut self, f: F) -> Result<Option<R>, E>
    where
        F: FnOnce(&Self::Item<'_>) -> Result<R, E>,
    {
        match self.peek().map(|item| f(&item)) {
            None => Ok(None),
            Some(Ok(r)) => {
                self.next();
                Ok(Some(r))
            }
            Some(Err(e)) => Err(e),
        }
    }
}
