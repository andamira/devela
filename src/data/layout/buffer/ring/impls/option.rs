// devela/src/data/layout/buffer/ring/impls/option.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_ring_impl_option {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {

        impl<T, const CAP: usize> Default for $name<T, [Option<T>; CAP]> {
            fn default() -> Self { Self::new_empty() }
        }

        $(#[$impl_attr])*
        ///
        /// Ring buffer backed by `[Option<T>; CAP]`.
        ///
        /// # Invariants
        /// - `len <= CAP`.
        /// - If `len == 0`, `head == 0`.
        /// - If `len > 0`, `head < CAP`.
        /// - The occupied logical range `0 .. len` maps through
        ///   `(head + logical_index) % CAP`.
        /// - Occupied physical slots are `Some(T)`.
        /// - Unoccupied physical slots are `None`.
        ///
        /// # Notes
        /// - `Option<T>` is used to control occupancy and dropping, not sparsity.
        /// - `tail` is derived as `(head + len) % CAP`.
        impl<T, const CAP: usize> $name<T, [Option<T>; CAP]> {
            /* construct */

            /// Creates an empty ring over optional storage.
            ///
            /// This constructor belongs to the `option` storage backend, where logical
            /// occupancy is represented by `Some` and `None` slots.
            pub const fn new_empty() -> Self {
                Self::_new([const { None }; CAP], Self::_idx_zero(), Self::_idx_zero())
            }
            /// Creates a ring from an array of options and explicit ring state.
            ///
            /// Returns `None` if the occupied slots do not match `head` and `len`.
            pub fn from_array_ring(array: [Option<T>; CAP], head: $I, len: $I) -> Option<Self> {
                let head_usize = Self::_idx_to_usize(head);
                let len_usize = Self::_idx_to_usize(len);
                if len_usize > CAP { return None; }
                if CAP == 0 {
                    if len_usize == 0 && head_usize == 0 {
                        return Some(Self::_new(array, Self::_idx_zero(), Self::_idx_zero()));
                    }
                    return None;
                }
                if len_usize != 0 && head_usize >= CAP { return None; }
                let head_usize = if len_usize == 0 { 0 } else { head_usize };
                let split = head_usize + len_usize;
                let wraps = split > CAP;
                $crate::whilst! { physical in 0..CAP; {
                    let occupied = if len_usize == CAP {
                        true
                    } else if len_usize == 0 {
                        false
                    } else if !wraps {
                        physical >= head_usize && physical < split
                    } else {
                        physical >= head_usize || physical < split - CAP
                    };
                    if occupied != array[physical].is_some() { return None; }
                }}
                Some(Self::_new(
                    array,
                    Self::_usize_to_midx(head_usize),
                    Self::_usize_to_midx(len_usize),
                ))
            }
            /// Primitive-index variant of [`from_array_ring`][Self::from_array_ring].
            ///
            /// Returns `None` if the occupied slots do not match `head` and `len`,
            /// or if any of the given primitive values are invalid.
            pub fn from_array_ring_prim(array: [Option<T>; CAP], head: $P, len: $P)
                -> Option<Self> {
                let head = Self::_prim_to_idx(head).ok()?;
                let len = Self::_prim_to_idx(len).ok()?;
                Self::from_array_ring(array, head, len)
            }

            /// Creates a ring from an array of options with a linear prefix layout.
            ///
            /// The ring head is `0`, and the logical length is inferred as the
            /// index of the first `None`.
            ///
            /// Returns `None` if a `Some` appears after the first `None`.
            pub fn from_array_linear(array: [Option<T>; CAP]) -> Option<Self> {
                let mut len = 0;
                $crate::whilst! { i in 0..CAP; {
                    if array[i].is_some() { len += 1; } else { break; }
                }}
                $crate::whilst! { i in len,..CAP; {
                    if array[i].is_some() { return None; }
                }}
                Some(Self::_new(array, Self::_idx_zero(), Self::_usize_to_midx(len)))
            }

            /// Creates a ring by cloning all elements from `src`.
            pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = Some(src[i].clone());
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(src.len())))
            }
            /// Creates a ring by copying all elements from `src`.
            pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = Some(src[i]);
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(src.len())))
            }

            /// Creates a ring by cloning all elements from an array.
            pub fn from_array_clone<const N: usize>(src: [T; N]) -> Option<Self> where T: Clone {
                if N > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; {
                    storage[i] = Some(src[i].clone());
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(N)))
            }
            /// Creates a ring by copying all elements from an array.
            pub const fn from_array_copy<const N: usize>(src: [T; N])
                -> Option<Self> where T: Copy {
                if N > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; {
                    storage[i] = Some(src[i]);
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(N)))
            }

            /* size & capacity */

            $crate::buffer_ring!(%common_static $name, $I, $P);

            /* logical range control */

            /// Clears the ring, dropping all elements.
            pub fn clear(&mut self) {
                let len = self._len_usize();
                $crate::whilst! { logical in 0..len; {
                    let physical = self._physical_usize(logical);
                    self.storage[physical] = None;
                }}
                self.head = Self::_idx_zero();
                self.len = Self::_idx_zero();
            }
            /// Truncates the ring to `new_len`, dropping elements from the back.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                if Self::_idx_ge(new_len, self.len()) { return; }
                let old_len = self._len_usize();
                let new_len_usize = Self::_idx_to_usize(new_len);
                $crate::whilst! { logical in new_len_usize,..old_len; {
                    let physical = self._physical_usize(logical);
                    self.storage[physical] = None;
                }}
                self._set_len(new_len);
                if new_len_usize == 0 { self.head = Self::_idx_zero(); }
            }

            /* push */

            /// Appends a value to the back of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                let tail = self._tail_usize();
                self.storage[tail] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }
            /// Appends a copy of `value` to the back of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                let tail = self._tail_usize();
                self.storage[tail] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }

            /// Appends as many elements cloned from `src` as fit at the back of the ring.
            ///
            /// Elements keep their slice order.
            /// After pushing `[a, b, c]`, `c` becomes the logical back element.
            pub fn push_back_slice(&mut self, src: &[T]) -> usize where T: Clone {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(self._tail_usize() + i);
                    self.storage[physical] = Some(src[i].clone());
                }}
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Appends as many elements copied from `src` as fit at the back of the ring.
            ///
            /// Elements keep their slice order.
            /// After pushing `[a, b, c]`, `c` becomes the logical back element.
            pub const fn push_back_slice_copy(&mut self, src: &[T]) -> usize where T: Copy {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(self._tail_usize() + i);
                    self.storage[physical] = Some(src[i]);
                }}
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Appends all copied from `src` at the back of the ring,
            /// or none if insufficient capacity.
            ///
            /// Returns `Err(remaining_capacity)` if not enough space is available.
            ///
            /// Elements keep their slice order.
            /// After pushing `[a, b, c]`, `c` becomes the logical back element.
            pub const fn push_back_slice_copy_exact(&mut self, src: &[T])
                -> Result<(), usize> where T: Copy {
                let rem = CAP - self._len_usize();
                if src.len() > rem { return Err(rem); }
                let _ = self.push_back_slice_copy(src);
                Ok(())
            }

            /// Prepends a value to the front of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub fn push_front(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                let head = self._prev_head_usize();
                self.storage[head] = Some(value);
                self.head = Self::_usize_to_midx(head);
                self.len = self._len_inc();
                Ok(())
            }
            /// Prepends a copy of `value` to the front of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub const fn push_front_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                let head = self._prev_head_usize();
                self.storage[head] = Some(value);
                self.head = Self::_usize_to_midx(head);
                self.len = self._len_inc();
                Ok(())
            }

            /// Prepends as many elements cloned from `src` as fit.
            ///
            /// Elements keep their slice order. If the ring contains `[10, 20]`,
            /// then pushing `[1, 2, 3]` to the front makes the logical order
            /// `[1, 2, 3, 10, 20]`.
            ///
            /// Returns the number of elements prepended.
            pub fn push_front_slice(&mut self, src: &[T]) -> usize where T: Clone {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                if count == 0 { return 0; }
                let head = self._head_usize();
                let new_head = if head >= count {
                    head - count
                } else {
                    CAP - (count - head)
                };
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(new_head + i);
                    self.storage[physical] = Some(src[i].clone());
                }}
                self.head = Self::_usize_to_midx(new_head);
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Prepends as many copied elements from `src` as fit.
            ///
            /// Elements keep their slice order. If the ring contains `[10, 20]`,
            /// then pushing `[1, 2, 3]` to the front makes the logical order
            /// `[1, 2, 3, 10, 20]`.
            ///
            /// Returns the number of elements prepended.
            pub const fn push_front_slice_copy(&mut self, src: &[T]) -> usize where T: Copy {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                if count == 0 { return 0; }
                let head = self._head_usize();
                let new_head = if head >= count {
                    head - count
                } else {
                    CAP - (count - head)
                };
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(new_head + i);
                    self.storage[physical] = Some(src[i]);
                }}
                self.head = Self::_usize_to_midx(new_head);
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Prepends all copied elements from `src`, or none if insufficient capacity.
            ///
            /// Elements keep their slice order. Returns `Err(remaining_capacity)` if not
            /// enough space is available.
            pub const fn push_front_slice_copy_exact(&mut self, src: &[T]) -> Result<(), usize>
                where T: Copy {
                let rem = CAP - self._len_usize();
                if src.len() > rem { return Err(rem); }
                let _ = self.push_front_slice_copy(src);
                Ok(())
            }

            /* pop */

            /// Removes and returns a value from the front of the ring.
            pub fn pop_front(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = self.storage[head].take();
                self.len = self._len_dec();
                if self.is_empty() {
                    self.head = Self::_idx_zero();
                } else {
                    self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1));
                }
                value
            }
            /// Removes and returns a copy from the front of the ring.
            pub const fn pop_front_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = self.storage[head];
                self.storage[head] = None;
                self.len = self._len_dec();
                if self.is_empty() {
                    self.head = Self::_idx_zero();
                } else {
                    self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1));
                }
                value
            }

            /// Removes and returns a value from the back of the ring.
            pub fn pop_back(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = self.storage[back].take();
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                value
            }
            /// Removes and returns a copy from the back of the ring.
            pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = self.storage[back];
                self.storage[back] = None;
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                value
            }

            /* peek */

            /// Returns a reference to the front element without removing it.
            pub const fn peek_front(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                self.storage[self._head_usize()].as_ref()
            }
            /// Returns an exclusive reference to the front element without removing it.
            pub fn peek_mut_front(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                self.storage[self._head_usize()].as_mut()
            }

            /// Returns a reference to the back element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                self.storage[self._back_usize()].as_ref()
            }
            /// Returns an exclusive reference to the back element without removing it.
            pub fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                self.storage[self._back_usize()].as_mut()
            }

            /* get */

            /// Returns a shared reference to the element at logical `index`.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                self.storage[physical].as_ref()
            }
            /// Primitive-index variant of [`get`][Self::get].
            pub const fn get_prim(&self, index: $P)
                -> Result<Option<&T>, $crate::InvalidValue> {
                Ok(self.get($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /// Returns an exclusive reference to the element at logical `index`.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                self.storage[physical].as_mut()
            }
            /// Primitive-index variant of [`get_mut`][Self::get_mut].
            pub const fn get_mut_prim(&mut self, index: $P)
                -> Result<Option<&mut T>, $crate::InvalidValue> {
                Ok(self.get_mut($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /* swap */

            /// Removes and returns the value at logical `index`,
            /// filling the gap with the logical back element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub fn swap_remove(&mut self, index: $I) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_logical = Self::_idx_to_usize(index);
                let index_physical = self._physical_usize(index_logical);
                let back_physical = self._back_usize();
                let value = self.storage[index_physical].take();
                if index_physical != back_physical {
                    self.storage[index_physical] = self.storage[back_physical].take();
                }
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                value
            }
            /// Primitive-index variant of [`swap_remove`][Self::swap_remove].
            pub fn swap_remove_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> {
                Ok(self.swap_remove($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /// Removes and returns a copy of the value at logical `index`,
            /// filling the gap with the logical back element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub const fn swap_remove_copy(&mut self, index: $I) -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_logical = Self::_idx_to_usize(index);
                let index_physical = self._physical_usize(index_logical);
                let back_physical = self._back_usize();
                let value = self.storage[index_physical];
                if index_physical != back_physical {
                    self.storage[index_physical] = self.storage[back_physical];
                }
                self.storage[back_physical] = None;
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                value
            }
            /// Primitive-index variant of [`swap_remove_copy`][Self::swap_remove_copy].
            pub fn swap_remove_copy_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> where T: Copy {
                Ok(self.swap_remove_copy($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /* views */

            /// Returns the active logical range as two option slices.
            ///
            /// The first slice starts at `head`. The second slice is non-empty
            /// only when the active range wraps around the end of storage.
            pub fn as_slices(&self) -> (&[Option<T>], &[Option<T>]) {
                let len = self._len_usize();
                if len == 0 { return (&self.storage[..0], &self.storage[..0]); }
                let head = self._head_usize();
                let first_len = if len <= CAP - head { len } else { CAP - head };
                let second_len = len - first_len;
                let (left, right) = self.storage.split_at(head);
                (&right[..first_len], &left[..second_len])
            }
            /// Returns the active logical range as two mutable option slices.
            ///
            /// The first slice starts at `head`. The second slice is non-empty
            /// only when the active range wraps around the end of storage.
            pub fn as_mut_slices(&mut self) -> (&mut [Option<T>], &mut [Option<T>]) {
                let len = self._len_usize();
                if len == 0 {
                    let (left, right) = self.storage.split_at_mut(0);
                    return (left, &mut right[..0]);
                }
                let head = self._head_usize();
                let first_len = if len <= CAP - head { len } else { CAP - head };
                let second_len = len - first_len;
                let (left, right) = self.storage.split_at_mut(head);
                (&mut right[..first_len], &mut left[..second_len])
            }

            /* iteration */

            /// Iterates over the initialized elements in logical order.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let (a, b) = self.as_slices();
                a.iter().chain(b.iter()).map(|x| x.as_ref().unwrap())
            }
            /// Iterates mutably over the initialized elements in logical order.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                let (a, b) = self.as_mut_slices();
                a.iter_mut().chain(b.iter_mut()).map(|x| x.as_mut().unwrap())
            }

            /* visitation */

            /// Visits each initialized element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                for x in self.iter() {
                    f(x);
                }
            }
            /// Visits each initialized element mutably without exposing borrow identity.
            pub fn visit_each_mut<F>(&mut self, f: F) where for<'v> F: Fn(&'v mut T) {
                for x in self.iter_mut() {
                    f(x);
                }
            }

            /// Visits the active logical range as two shared option slices.
            pub fn visit_slices<F, R>(&self, f: F)
                -> R where for<'v> F: FnOnce(&'v [Option<T>], &'v [Option<T>]) -> R {
                let (a, b) = self.as_slices();
                f(a, b)
            }
            /// Visits the active logical range as two exclusive option slices.
            pub fn visit_mut_slices<F, R>(&mut self, f: F)
                -> R where for<'v> F: FnOnce(&'v mut [Option<T>], &'v mut [Option<T>]) -> R {
                let (a, b) = self.as_mut_slices();
                f(a, b)
            }
        }
    };
}
