// devela/src/data/store/pool/seq/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __pool_seq_impl_array {
    (
        [cell: $cprim:ident]
        [private:
            meta: $Meta:ident;
            free_span: $FreeSpan:ident;
            meta_pool: $MetaPool:ident;
        ]
        $(#[$pool_attr:meta])*
        $vis:vis $Pool:ident;
        $hvis:vis $Handle:ident;
    ) => {
        /* private representation */

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct $Meta {
            start: $cprim,
            len: $cprim,
            capacity: $cprim,
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        struct $FreeSpan {
            start: $cprim,
            len: $cprim,
        }
        impl $FreeSpan {
            const EMPTY: Self = Self { start: 0, len: 0 };
        }

        /* sequence pool */

        $(#[$pool_attr])*
        #[derive(Clone, Debug)]
        $vis struct $Pool<T, const SEQS: usize, const CELLS: usize> {
            seqs: $MetaPool<$Meta, SEQS>,
            cells: [T; CELLS],

            // Sorted by `start`, immediately coalesced.
            free: [$FreeSpan; SEQS],
            free_len: usize,

            // One-past the physically used prefix.
            frontier: $cprim,

            // Logical cells and physically reserved cells.
            cell_len: usize,
            allocated_len: usize,
        }

        impl<T: $crate::ConstInit, const SEQS: usize, const CELLS: usize>
            $crate::ConstInit for $Pool<T, SEQS, CELLS> {
            const INIT: Self = Self::new_init();
        }
        impl<T: $crate::ConstInit, const SEQS: usize, const CELLS: usize>
            Default for $Pool<T, SEQS, CELLS> {
            fn default() -> Self { Self::new_init() }
        }

        #[allow(dead_code)]
        impl<T, const SEQS: usize, const CELLS: usize> $Pool<T, SEQS, CELLS> {
            const _VALID_CONFIG: () = {
                const fn require_uint<P: $crate::PrimUint>() {}
                require_uint::<$cprim>();
                assert!(CELLS <= $cprim::MAX as usize,
                    "the cell capacity exceeds its representation");
            };

            /// Returns a new empty pool using `T::INIT` as backing storage.
            #[must_use]
            $vis const fn new_init() -> Self where T: $crate::ConstInit {
                let () = Self::_VALID_CONFIG;
                Self {
                    seqs: $MetaPool::new(),
                    cells: [const { T::INIT }; CELLS],
                    free: [$FreeSpan::EMPTY; SEQS],
                    free_len: 0,
                    frontier: 0,
                    cell_len: 0,
                    allocated_len: 0,
                }
            }

            /* capacity */

            /// Returns the maximum number of simultaneously live sequences.
            #[must_use]
            $vis const fn capacity(&self) -> usize { SEQS }

            /// Returns the number of live sequences.
            #[must_use]
            $vis const fn len(&self) -> usize { self.seqs.len() }

            /// Returns whether there are no live sequences.
            #[must_use]
            $vis const fn is_empty(&self) -> bool { self.seqs.is_empty() }

            /// Returns whether no additional sequence identity is available.
            #[must_use]
            $vis const fn is_full(&self) -> bool { self.seqs.is_full() }

            /// Returns the remaining sequence capacity.
            #[must_use]
            $vis const fn remaining(&self) -> usize { self.seqs.remaining() }

            /// Returns the total cell capacity.
            #[must_use]
            $vis const fn cell_capacity(&self) -> usize { CELLS }

            /// Returns the number of logical cells in live sequences.
            #[must_use]
            $vis const fn cell_len(&self) -> usize { self.cell_len }

            /// Returns the total number of cells reserved by live sequences.
            #[must_use]
            $vis const fn allocated_cell_len(&self) -> usize { self.allocated_len }

            /// Returns the number of cells not currently reserved by any sequence.
            #[must_use]
            $vis const fn cell_remaining(&self) -> usize { CELLS - self.allocated_len }

            /// Returns the largest currently available contiguous cell span.
            #[must_use]
            $vis const fn largest_free_span(&self) -> usize {
                let mut largest = CELLS - self.frontier as usize;
                $crate::whilst! { i in 0..self.free_len; {
                    let len = self.free[i].len as usize;
                    if len > largest { largest = len; }
                }}
                largest
            }
            /// Returns whether a sequence of `len` cells can currently be inserted.
            #[must_use]
            $vis const fn can_insert(&self, len: usize) -> bool {
                !self.is_full() && len <= self.largest_free_span()
            }
            /// Returns whether total free cells suffice but no contiguous span does.
            #[must_use]
            $vis const fn is_fragmented_for(&self, len: usize) -> bool {
                len <= self.cell_remaining() && len > self.largest_free_span()
            }
            /// Returns the unused capacity currently reserved by `handle`.
            #[must_use]
            $hvis const fn seq_remaining(&self, handle: $Handle) -> Option<usize> {
                $crate::unwrap![some_map self.seqs.get(handle),
                    |meta| meta.capacity as usize - meta.len as usize]
            }

            /* access */

            /// Returns whether `handle` currently identifies a sequence.
            #[must_use]
            $hvis const fn contains(&self, handle: $Handle) -> bool {
                self.seqs.contains(handle)
            }
            /// Returns the sequence length.
            #[must_use]
            $hvis const fn seq_len(&self, handle: $Handle) -> Option<usize> {
                $crate::unwrap![some_map self.seqs.get(handle), |meta| meta.len as usize]
            }
            /// Returns the sequence's currently reserved cell capacity.
            #[must_use]
            $hvis const fn seq_capacity(&self, handle: $Handle) -> Option<usize> {
                $crate::unwrap![some_map self.seqs.get(handle), |meta| meta.capacity as usize]
            }
            /// Returns the logical cells of the sequence identified by `handle`.
            #[must_use]
            $hvis fn get(&self, handle: $Handle) -> Option<&[T]> {
                let meta = *self.seqs.get(handle)?;
                let start = meta.start as usize;
                let end = start + meta.len as usize;
                Some(&self.cells[start..end])
            }
            /// Returns the logical cells of the sequence identified by `handle` exclusively.
            #[must_use]
            $hvis fn get_mut(&mut self, handle: $Handle) -> Option<&mut [T]> {
                let meta = *self.seqs.get(handle)?;
                let start = meta.start as usize;
                let end = start + meta.len as usize;
                Some(&mut self.cells[start..end])
            }

            /* mutation */

            /// Removes every sequence and invalidates every live handle.
            $vis const fn clear(&mut self) {
                self.seqs.clear_copy();
                self.free_len = 0;
                self.frontier = 0;
                self.cell_len = 0;
                self.allocated_len = 0;
            }

            /// Inserts a copy of `values`, returning its sequence handle.
            ///
            /// Returns `None` when no sequence slot or contiguous cell span fits.
            $hvis const fn insert(&mut self, values: &[T]) -> Option<$Handle> where T: Copy {
                if self.seqs.is_full() { return None; }
                let len = values.len();
                let start = $crate::unwrap![some? self._alloc_span(len)];
                let meta = $Meta {
                    start: start as $cprim,
                    len: len as $cprim,
                    capacity: len as $cprim,
                };
                let handle = match self.seqs.insert_copy(meta) {
                    Ok(handle) => handle,
                    Err(_) => { self._release_span(start, len); return None; }
                };
                $crate::whilst! { i in 0..len; {
                    self.cells[start + i] = values[i];
                }}
                self.cell_len += len;
                self.allocated_len += len;
                Some(handle)
            }
            /// Removes a sequence and reclaims its reserved cell span.
            $hvis const fn remove(&mut self, handle: $Handle) -> bool {
                let meta = $crate::unwrap![some_or? self.seqs.remove(handle), false];
                self.cell_len -= meta.len as usize;
                self.allocated_len -= meta.capacity as usize;
                self._release_span(meta.start as usize, meta.capacity as usize);
                true
            }
            /// Shortens a sequence without releasing its reserved capacity.
            ///
            /// Returns `false` if `handle` is invalid.
            $hvis const fn truncate(&mut self, handle: $Handle, len: usize) -> bool {
                let meta = $crate::unwrap![some_or? self.seqs.get_mut(handle), false];
                let old_len = meta.len as usize;
                if len < old_len {
                    meta.len = len as $cprim;
                    self.cell_len -= old_len - len;
                }
                true
            }
            /// Releases unused capacity from a sequence.
            ///
            /// Returns `false` if `handle` is invalid.
            $hvis const fn shrink_to_fit(&mut self, handle: $Handle) -> bool {
                let (tail_start, tail_len) = {
                let meta = $crate::unwrap![some_or? self.seqs.get_mut(handle), false];
                    let len = meta.len as usize;
                    let capacity = meta.capacity as usize;
                    let tail_len = capacity - len;
                    if tail_len == 0 { return true; }
                    meta.capacity = meta.len;
                    (meta.start as usize + len, tail_len)
                };
                self.allocated_len -= tail_len;
                self._release_span(tail_start, tail_len);
                true
            }
            /// Removes the logical contents while preserving reserved capacity.
            $hvis const fn clear_seq(&mut self, handle: $Handle) -> bool {
                self.truncate(handle, 0)
            }

            /// Reserves capacity for at least `additional` cells beyond the current length.
            ///
            /// Existing reserved capacity is reused first. The sequence may extend in place
            /// or relocate while its handle remains valid.
            ///
            /// Returns `false` if `handle` is invalid or no contiguous target span fits.
            /// The pool is not implicitly compacted.
            $hvis const fn reserve_exact(&mut self, handle: $Handle, additional: usize) -> bool
            where T: Copy {
                let old = *$crate::unwrap![some_or? self.seqs.get(handle), false];
                let len = old.len as usize;
                let capacity = old.capacity as usize;
                let target = $crate::unwrap![some_or? len.checked_add(additional), false];
                if target > CELLS { return false; }
                if target <= capacity { return true; }
                let growth = target - capacity;
                let old_start = old.start as usize;
                let old_end = old_start + capacity;
                if self._claim_after(old_end, growth) {
                    let meta = $crate::unwrap![some_or? self.seqs.get_mut(handle), false];
                    meta.capacity = target as $cprim;
                    self.allocated_len += growth;
                    return true;
                }
                // Otherwise relocate into another contiguous span.
                //
                // The old span stays reserved until its contents have been copied.
                let new_start = $crate::unwrap![some_or? self._alloc_span(target), false];
                $crate::whilst! { i in 0..len; {
                    self.cells[new_start + i] = self.cells[old_start + i];
                }}
                let meta = match self.seqs.get_mut(handle) {
                    Some(meta) => meta,
                    None => { self._release_span(new_start, target); return false; }
                };
                meta.start = new_start as $cprim;
                meta.capacity = target as $cprim;
                self._release_span(old_start, capacity);
                self.allocated_len += growth;
                true
            }
            /// Appends one cell to a sequence.
            ///
            /// The sequence may relocate while its handle remains valid.
            ///
            /// Returns `Err(value)` if `handle` is invalid or no contiguous span fits.
            $hvis const fn push(&mut self, handle: $Handle, value: T) -> Result<(), T>
                where T: Copy {
                if !self.reserve_exact(handle, 1) { return Err(value); }
                let pos = {
                    let meta = $crate::unwrap![some_ok_or? self.seqs.get_mut(handle), value];
                    let len = meta.len as usize;
                    let pos = meta.start as usize + len;
                    meta.len = (len + 1) as $cprim;
                    pos
                };
                self.cells[pos] = value;
                self.cell_len += 1;
                Ok(())
            }
            /// Appends all cells from `values`.
            ///
            /// The sequence may relocate while its handle remains valid.
            ///
            /// Returns `false` without modification if `handle` is invalid
            /// or the complete slice cannot fit.
            $hvis const fn extend_from_slice(&mut self, handle: $Handle, values: &[T]) -> bool
            where T: Copy {
                if !self.reserve_exact(handle, values.len()) { return false; }
                let old = *$crate::unwrap![some_or? self.seqs.get(handle), false];
                let len = old.len as usize;
                let start = old.start as usize + len;
                $crate::whilst! { i in 0..values.len(); {
                    self.cells[start + i] = values[i];
                }}
                let meta = $crate::unwrap![some_or? self.seqs.get_mut(handle), false];
                meta.len = (len + values.len()) as $cprim;
                self.cell_len += values.len();
                true
            }
            /// Removes and returns the last cell of a sequence.
            ///
            /// Reserved capacity is preserved.
            $hvis const fn pop(&mut self, handle: $Handle) -> Option<T> where T: Copy {
                let value = {
                    let meta = $crate::unwrap![some? self.seqs.get(handle)];
                    let len = meta.len as usize;
                    if len == 0 { return None; }
                    self.cells[meta.start as usize + len - 1]
                };
                let meta = $crate::unwrap![some? self.seqs.get_mut(handle)];
                meta.len = (meta.len as usize - 1) as $cprim;
                self.cell_len -= 1;
                Some(value)
            }

            /* packing */

            /// Removes gaps between physical sequence spans.
            ///
            /// Per-sequence capacities, logical contents, handles,
            /// and physical sequence order are preserved.
            $vis fn compact(&mut self) where T: Copy {
                self._repack(false);
            }
            /// Packs all logical sequence contents into one contiguous prefix.
            ///
            /// Each sequence's reserved capacity is reduced to its logical length.
            /// Handles, contents, and physical sequence order are preserved.
            $vis fn pack(&mut self) where T: Copy {
                self._repack(true);
            }

            /* free extents */

            const fn _alloc_span(&mut self, len: usize) -> Option<usize> {
                if len == 0 { return Some(0); }
                // First fit among interior holes.
                $crate::whilst! { i in 0..self.free_len; {
                    let span_len = self.free[i].len as usize;
                    if span_len >= len {
                        let start = self.free[i].start as usize;
                        if span_len == len {
                            self._remove_free(i);
                        } else {
                            self.free[i].start = (start + len) as $cprim;
                            self.free[i].len = (span_len - len) as $cprim;
                        }
                        return Some(start);
                    }
                }}
                // Otherwise grow the frontier.
                let frontier = self.frontier as usize;
                if len <= CELLS - frontier {
                    self.frontier = (frontier + len) as $cprim;
                    Some(frontier)
                } else {
                    None
                }
            }
            const fn _release_span(&mut self, start: usize, len: usize) {
                if len == 0 { return; }
                let end = start + len;
                // Trailing reclamation lowers the frontier directly.
                if end == self.frontier as usize {
                    self.frontier = start as $cprim;
                    // Absorb free spans immediately preceding it.
                    while self.free_len != 0 {
                        let i = self.free_len - 1;
                        let span = self.free[i];
                        let span_end = span.start as usize + span.len as usize;
                        if span_end != self.frontier as usize { break; }
                        self.frontier = span.start;
                        self.free_len -= 1;
                    }
                    return;
                }
                // Insert sorted by start.
                assert!(self.free_len < SEQS, "pool_seq free-span invariant violated");
                let mut pos = 0;
                while pos < self.free_len && (self.free[pos].start as usize) < start {
                    pos += 1;
                }
                let mut i = self.free_len;
                while i > pos {
                    self.free[i] = self.free[i - 1];
                    i -= 1;
                }
                self.free[pos] = $FreeSpan {
                    start: start as $cprim,
                    len: len as $cprim,
                };
                self.free_len += 1;
                // Coalesce with the previous span.
                if pos != 0 {
                    let prev = pos - 1;
                    let prev_end = self.free[prev].start as usize
                                 + self.free[prev].len as usize;
                    if prev_end == self.free[pos].start as usize {
                        let merged = self.free[prev].len as usize
                                   + self.free[pos].len as usize;
                        self.free[prev].len = merged as $cprim;
                        self._remove_free(pos);
                        pos = prev;
                    }
                }
                // Coalesce with the following span.
                if pos + 1 < self.free_len {
                    let end = self.free[pos].start as usize
                            + self.free[pos].len as usize;
                    if end == self.free[pos + 1].start as usize {
                        let merged = self.free[pos].len as usize
                                   + self.free[pos + 1].len as usize;
                        self.free[pos].len = merged as $cprim;
                        self._remove_free(pos + 1);
                    }
                }
            }
            const fn _remove_free(&mut self, index: usize) {
                let mut i = index;
                while i + 1 < self.free_len {
                    self.free[i] = self.free[i + 1];
                    i += 1;
                }
                self.free_len -= 1;
            }
            /// Claims `len` free cells immediately following `end`.
            const fn _claim_after(&mut self, end: usize, len: usize) -> bool {
                if len == 0 { return true; }
                // Extend the physical frontier.
                if end == self.frontier as usize {
                    if len > CELLS - end { return false; }
                    self.frontier = (end + len) as $cprim;
                    return true;
                }
                // Consume the beginning of an immediately following free span.
                $crate::whilst! { i in 0..self.free_len; {
                    let start = self.free[i].start as usize;
                    if start > end { break; }
                    if start == end {
                        let span_len = self.free[i].len as usize;
                        if span_len < len { return false; }
                        if span_len == len {
                            self._remove_free(i);
                        } else {
                            self.free[i].start = (start + len) as $cprim;
                            self.free[i].len = (span_len - len) as $cprim;
                        }
                        return true;
                    }
                }}
                false
            }
            fn _repack(&mut self, shrink: bool) where T: Copy {
                let mut write = 0usize;
                let mut source_floor = 0usize;
                loop {
                    // Descriptor slots are identity-ordered, not physically ordered.
                    // Find the next live physical extent.
                    let mut found = false;
                    let mut next_start = 0usize;
                    let mut next_len = 0usize;
                    let mut next_capacity = 0usize;
                    for meta in self.seqs.iter() {
                        let capacity = meta.capacity as usize;
                        if capacity == 0 { continue; }
                        let start = meta.start as usize;
                        if start < source_floor { continue; }
                        if !found || start < next_start {
                            found = true;
                            next_start = start;
                            next_len = meta.len as usize;
                            next_capacity = capacity;
                        }
                    }
                    if !found { break; }
                    // Moving toward lower addresses is safe with a forward copy.
                    if write != next_start {
                        $crate::whilst! { i in 0..next_len; {
                            self.cells[write + i] = self.cells[next_start + i];
                        }}
                    }
                    // Positive-capacity extents have unique starts.
                    for meta in self.seqs.iter_mut() {
                        if meta.capacity as usize != 0 && meta.start as usize == next_start {
                            meta.start = write as $cprim;
                            if shrink { meta.capacity = meta.len; }
                            break;
                        }
                    }
                    // Advance using the OLD physical extent before packing its reservation.
                    source_floor = next_start + next_capacity;
                    write += if shrink { next_len } else { next_capacity };
                }
                if shrink { self.allocated_len = self.cell_len; }
                self.free_len = 0;
                self.frontier = write as $cprim;
            }
        }
    };
}
