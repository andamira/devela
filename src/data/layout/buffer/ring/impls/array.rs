// devela/src/data/layout/buffer/ring/impls/array.rs

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_ring_impl_array {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {

        $(#[$impl_attr])*
        ///
        /// Fully initialized ring array.
        ///
        /// # Invariants
        /// - All `CAP` slots always contain a valid `T`.
        /// - `len` controls logical membership, not initialization.
        /// - `head` is the physical start of the logical range.
        /// - The tail is derived as `(head + len) % CAP`.
        /// - Dropping the ring drops all `CAP` stored values.
        ///
        /// # Consequences
        /// - Pushing can overwrite inactive initialized slots.
        /// - Popping by value needs `Copy`, `Clone`, or a replacement value.
        /// - Shrinking `len` does not drop elements immediately.
        #[rustfmt::skip]
        impl<T, const CAP: usize> $name<T, [T; CAP]> {
            /* construct */

            /// Creates an empty ring with backing storage initialized from `T::INIT`.
            pub const fn new_init() -> Self where T: $crate::ConstInit {
                Self::_new([T::INIT; CAP], Self::_idx_zero(), Self::_idx_zero())
            }
            /// Creates an empty ring from already initialized storage.
            pub const fn from_array_empty(array: [T; CAP]) -> Self {
                Self::_new(array, Self::_idx_zero(), Self::_idx_zero())
            }
            /// Creates a full ring from already initialized storage.
            pub const fn from_array_full(array: [T; CAP]) -> Self {
                Self::_new(array, Self::_idx_zero(), Self::_usize_to_midx(CAP))
            }
            /// Creates a ring from an array and explicit ring state.
            ///
            /// Returns `None` if:
            /// - `len > CAP`
            /// - `head >= CAP` while `len > 0`
            pub fn from_array_ring(array: [T; CAP], head: $I, len: $I) -> Option<Self> {
                let head_usize = Self::_idx_to_usize(head);
                let len_usize = Self::_idx_to_usize(len);
                if len_usize > CAP { return None; }
                if CAP == 0 {
                    if len_usize == 0 {
                        return Some(Self::_new(array, Self::_idx_zero(), Self::_idx_zero()));
                    }
                    return None;
                }
                if len_usize != 0 && head_usize >= CAP { return None; }
                let head = if len_usize == 0 {
                    Self::_idx_zero()
                } else {
                    Self::_usize_to_midx(head_usize)
                };
                Some(Self::_new(array, head, Self::_usize_to_midx(len_usize)))
            }
            /// Primitive-index variant of [`from_array_ring`][Self::from_array_ring].
            pub fn from_array_ring_prim(array: [T; CAP], head: $P, len: $P) -> Option<Self> {
                let head = Self::_prim_to_idx(head).ok()?;
                let len = Self::_prim_to_idx(len).ok()?;
                Self::from_array_ring(array, head, len)
            }

            /// Creates a ring by cloning all elements from `src`,
            /// after initializing the remaining capacity with `init`.
            pub fn from_slice_clone(src: &[T], init: T) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = $crate::array_from_fn(|_| init.clone());
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = src[i].clone();
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(src.len())))
            }
            /// Creates a ring by copying all elements from `src`,
            /// after initializing the remaining capacity with `init`.
            pub const fn from_slice_copy(src: &[T], init: T) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [init; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = src[i];
                }}
                Some(Self::_new(storage, Self::_idx_zero(), Self::_usize_to_midx(src.len())))
            }

            /* size & capacity */

            $crate::buffer_ring!(%common_static $name, $I, $P);

            /* logical range control */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements. The array remains fully initialized.
            pub const fn clear(&mut self) {
                self.head = Self::_idx_zero();
                self.len = Self::_idx_zero();
            }
            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub const fn truncate(&mut self, new_len: $I) {
                if Self::_idx_le(new_len, self.len()) {
                    self._set_len(new_len);
                    if Self::_idx_eq(new_len, Self::_idx_zero().repr()) {
                        self.head = Self::_idx_zero();
                    }
                }
            }
            /// Primitive-index variant of [`truncate`][Self::truncate].
            pub const fn truncate_prim(&mut self, new_len: $P)
                -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
            }

            /* spare */

            /// Returns the current contiguous spare capacity at the back of the ring.
            #[must_use]
            pub const fn spare_back_capacity(&self) -> usize {
                if self.is_full() { 0 }
                else {
                    let (tail, head, len) =
                        (self._tail_usize(), self._head_usize(), self._len_usize());
                    if len == 0 || tail >= head { CAP - tail } else { head - tail }
                }
            }
            /// Returns the contiguous spare storage available at the back of the ring.
            ///
            /// The returned slice is part of the initialized backing array but is not yet
            /// part of the logical ring contents. Write into this slice, then call
            /// [`commit_back_slice`][Self::commit_back_slice] with the number of written
            /// elements.
            ///
            /// The slice may be shorter than
            /// [`remaining_capacity`][Self::remaining_capacity] when the ring wraps before
            /// the physical end of the backing array.
            #[must_use]
            pub fn spare_back_slice_mut(&mut self) -> &mut [T] {
                if self.is_full() { return &mut []; }
                let (tail, head, len) = (self._tail_usize(), self._head_usize(), self._len_usize());
                let end = if len == 0 || tail >= head { CAP } else { head };
                &mut self.storage[tail..end]
            }
            /// Extends the logical back of the ring by `count` contiguous elements.
            ///
            /// This commits elements previously written into
            /// [`spare_back_slice_mut`][Self::spare_back_slice_mut].
            ///
            /// Returns `Err(contiguous_remaining)` if `count` exceeds the current
            /// contiguous spare slice length.
            pub const fn commit_back_slice(&mut self, count: $I) -> Result<(), $I> {
                let count_usize = Self::_idx_to_usize(count);
                let spare = self.spare_back_capacity();
                if count_usize > spare { return Err(Self::_usize_to_idx(spare)); }
                self.len = Self::_usize_to_midx(self._len_usize() + count_usize);
                Ok(())
            }
            /// Primitive-index variant of [`commit_back_slice`][Self::commit_back_slice].
            ///
            /// Returns `Err(contiguous_remaining)` if `count` exceeds the current
            /// contiguous spare slice length, or if `count` cannot be represented by the
            /// index type.
            pub const fn commit_back_slice_prim(&mut self, count: $P) -> Result<(), $P> {
                let Ok(count_idx) = Self::_prim_to_idx(count) else {
                    return Err(self.remaining_capacity_prim());
                };
                match self.commit_back_slice(count_idx) {
                    Ok(()) => Ok(()),
                    Err(rem) => Err(Self::_idx_to_prim(rem)),
                }
            }
            /// `usize` variant of [`commit_back_slice`][Self::commit_back_slice].
            ///
            /// Returns `Err(contiguous_remaining)` if `count` exceeds the current
            /// contiguous spare slice length, or if `count` cannot be represented by the
            /// index type.
            pub const fn commit_back_slice_usize(&mut self, count: usize) -> Result<(), usize> {
                let Ok(count_idx) = $crate::MaybeNiche::<$I>::try_from_usize(count) else {
                    return Err(self.spare_back_capacity());
                };
                match self.commit_back_slice(count_idx.repr()) {
                    Ok(()) => Ok(()),
                    Err(rem) => Err(Self::_idx_to_usize(rem)),
                }
            }

            /* push: back */

            /// Appends a value to the back of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                let tail = self._tail_usize();
                self.storage[tail] = value;
                self.len = self._len_inc();
                Ok(())
            }
            /// Appends a copy of `value` to the back of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                let tail = self._tail_usize();
                self.storage[tail] = value;
                self.len = self._len_inc();
                Ok(())
            }

            /// Appends as many cloned elements from `src` as fit at the back.
            ///
            /// Elements keep their slice order.
            pub fn push_back_slice(&mut self, src: &[T]) -> usize where T: Clone {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(self._tail_usize() + i);
                    self.storage[physical] = src[i].clone();
                }}
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Appends as many copied elements from `src` as fit at the back.
            ///
            /// Elements keep their slice order.
            pub const fn push_back_slice_copy(&mut self, src: &[T]) -> usize where T: Copy {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(self._tail_usize() + i);
                    self.storage[physical] = src[i];
                }}
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Appends all copied elements from `src`, or none if insufficient capacity.
            ///
            /// Returns `Err(remaining_capacity)` if not enough space is available.
            pub const fn push_back_slice_copy_exact(&mut self, src: &[T])
                -> Result<(), usize> where T: Copy {
                let rem = CAP - self._len_usize();
                if src.len() > rem { return Err(rem); }
                let _ = self.push_back_slice_copy(src);
                Ok(())
            }

            /* push: front */

            /// Prepends a value to the front of the ring.
            ///
            /// Returns `Err(value)` if the ring is full.
            pub fn push_front(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                let head = self._prev_head_usize();
                self.storage[head] = value;
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
                self.storage[head] = value;
                self.head = Self::_usize_to_midx(head);
                self.len = self._len_inc();
                Ok(())
            }

            /// Prepends as many cloned elements from `src` as fit at the front.
            ///
            /// Elements keep their slice order. If the ring contains `[10, 20]`,
            /// then pushing `[1, 2, 3]` to the front makes the logical order
            /// `[1, 2, 3, 10, 20]`.
            pub fn push_front_slice(&mut self, src: &[T]) -> usize where T: Clone {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                if count == 0 { return 0; }
                let head = self._head_usize();
                let new_head = if head >= count { head - count } else { CAP - (count - head) };
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(new_head + i);
                    self.storage[physical] = src[i].clone();
                }}
                self.head = Self::_usize_to_midx(new_head);
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Prepends as many copied elements from `src` as fit at the front.
            ///
            /// Elements keep their slice order. If the ring contains `[10, 20]`,
            /// then pushing `[1, 2, 3]` to the front makes the logical order
            /// `[1, 2, 3, 10, 20]`.
            pub const fn push_front_slice_copy(&mut self, src: &[T]) -> usize where T: Copy {
                let count = $crate::cmp!(min src.len(), CAP - self._len_usize());
                if count == 0 { return 0; }
                let head = self._head_usize();
                let new_head = if head >= count { head - count } else { CAP - (count - head) };
                $crate::whilst! { i in 0..count; {
                    let physical = Self::_wrap_usize(new_head + i);
                    self.storage[physical] = src[i];
                }}
                self.head = Self::_usize_to_midx(new_head);
                self.len = Self::_usize_to_midx(self._len_usize() + count);
                count
            }
            /// Prepends all copied elements from `src`, or none if insufficient capacity.
            ///
            /// Returns `Err(remaining_capacity)` if not enough space is available.
            pub const fn push_front_slice_copy_exact(&mut self, src: &[T])
                -> Result<(), usize> where T: Copy {
                let rem = CAP - self._len_usize();
                if src.len() > rem { return Err(rem); }
                let _ = self.push_front_slice_copy(src);
                Ok(())
            }

            /* pop: front */

            /// Removes and returns a copied value from the front,
            /// replacing its storage slot with `T::INIT`.
            pub const fn pop_front(&mut self) -> Option<T> where T: $crate::ConstInit {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = $crate::Mem::replace(&mut self.storage[head], T::INIT);
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                else { self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1)); }
                Some(value)
            }
            /// Removes and returns a value from the front,
            /// replacing its storage slot with `T::default()`.
            pub fn pop_front_default(&mut self) -> Option<T> where T: Default {
                self.pop_front_with(T::default())
            }
            /// Removes and returns a value from the front,
            /// replacing its storage slot with `replacement`.
            pub fn pop_front_with(&mut self, replacement: T) -> Option<T> {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = $crate::Mem::replace(&mut self.storage[head], replacement);
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                else { self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1)); }
                Some(value)
            }
            /// Removes and returns a copied value from the front,
            /// replacing its storage slot with `replacement`.
            pub const fn pop_front_copy_with(&mut self, replacement: T) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = self.storage[head];
                self.storage[head] = replacement;
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                else { self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1)); }
                Some(value)
            }
            /// Removes and returns a cloned value from the front of the ring.
            pub fn pop_front_clone(&mut self) -> Option<T> where T: Clone {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = self.storage[head].clone();
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                else { self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1)); }
                Some(value)
            }
            /// Removes and returns a copied value from the front of the ring.
            pub const fn pop_front_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let head = self._head_usize();
                let value = self.storage[head];
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                else { self.head = Self::_usize_to_midx(Self::_wrap_usize(head + 1)); }
                Some(value)
            }

            /* pop: back */

            /// Removes and returns a copied value from the back,
            /// replacing its storage slot with `T::INIT`.
            pub const fn pop_back(&mut self) -> Option<T> where T: $crate::ConstInit {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = $crate::Mem::replace(&mut self.storage[back], T::INIT);
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Removes and returns a value from the back,
            /// replacing its storage slot with `T::default()`.
            pub fn pop_back_default(&mut self) -> Option<T> where T: Default {
                self.pop_back_with(T::default())
            }
            /// Removes and returns a value from the back,
            /// replacing its storage slot with `replacement`.
            pub fn pop_back_with(&mut self, replacement: T) -> Option<T> {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = $crate::Mem::replace(&mut self.storage[back], replacement);
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Removes and returns a copied value from the back,
            /// replacing its storage slot with `replacement`.
            pub const fn pop_back_copy_with(&mut self, replacement: T) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = self.storage[back];
                self.storage[back] = replacement;
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Removes and returns a cloned value from the back of the ring.
            pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = self.storage[back].clone();
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Removes and returns a copied value from the back of the ring.
            pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                let back = self._back_usize();
                let value = self.storage[back];
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }

            /* peek */

            /// Returns a reference to the front element without removing it.
            pub const fn peek_front(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self._head_usize()])
            }
            /// Returns an exclusive reference to the front element without removing it.
            pub const fn peek_mut_front(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                Some(&mut self.storage[self._head_usize()])
            }

            /// Returns a reference to the back element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self._back_usize()])
            }
            /// Returns an exclusive reference to the back element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                Some(&mut self.storage[self._back_usize()])
            }

            /* get */

            /// Returns a shared reference to the element at logical `index`.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                Some(&self.storage[physical])
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
                Some(&mut self.storage[physical])
            }
            /// Primitive-index variant of [`get_mut`][Self::get_mut].
            pub const fn get_mut_prim(&mut self, index: $P)
                -> Result<Option<&mut T>, $crate::InvalidValue> {
                Ok(self.get_mut($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /* take */

            /// Takes the value at logical `index`, replacing it with `replacement`.
            ///
            /// Does not change the logical length.
            pub fn take_with(&mut self, index: $I, replacement: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                Some($crate::Mem::replace(&mut self.storage[physical], replacement))
            }
            /// Primitive-index variant of [`take_with`][Self::take_with].
            pub fn take_with_prim(&mut self, index: $P, replacement: T)
                -> Result<Option<T>, $crate::InvalidValue> {
                Ok(self.take_with($crate::unwrap![ok? Self::_prim_to_idx(index)], replacement))
            }

            /// Takes the value at logical `index`, replacing it with `T::default()`.
            ///
            /// Does not change the logical length.
            pub fn take_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.take_with(index, T::default())
            }
            /// Primitive-index variant of [`take_default`][Self::take_default].
            pub fn take_default_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> where T: Default {
                Ok(self.take_default($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /// Takes a copied value at logical `index`, replacing it with `replacement`.
            ///
            /// Does not change the logical length.
            pub const fn take_copy_with(&mut self, index: $I, replacement: T)
                -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                let value = self.storage[physical];
                self.storage[physical] = replacement;
                Some(value)
            }
            /// Primitive-index variant of [`take_copy_with`][Self::take_copy_with].
            pub const fn take_copy_with_prim(&mut self, index: $P, replacement: T)
                -> Result<Option<T>, $crate::InvalidValue> where T: Copy {
                Ok(self.take_copy_with($crate::unwrap![ok? Self::_prim_to_idx(index)], replacement))
            }

            /// Takes the value at logical `index`, replacing it with `T::INIT`.
            ///
            /// Does not change the logical length.
            pub const fn take_init(&mut self, index: $I) -> Option<T> where T: $crate::ConstInit {
                if Self::_idx_ge(index, self.len()) { return None; }
                let physical = self._physical_usize(Self::_idx_to_usize(index));
                Some($crate::Mem::replace(&mut self.storage[physical], T::INIT))
            }
            /// Primitive-index variant of [`take_init`][Self::take_init].
            pub const fn take_init_prim(&mut self, index: $P)
               -> Result<Option<T>, $crate::InvalidValue> where T: $crate::ConstInit {
                Ok(self.take_init($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /* swap */

            /// Removes and returns the value at logical `index`,
            /// filling the gap with the logical back element and replacing the
            /// old back slot with `replacement`.
            ///
            /// Decrements `len`. Does not preserve order.
            pub fn swap_remove_with(&mut self, index: $I, replacement: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_physical = self._physical_usize(Self::_idx_to_usize(index));
                let back_physical = self._back_usize();
                let value = if index_physical == back_physical {
                    $crate::Mem::replace(&mut self.storage[back_physical], replacement)
                } else {
                    let back_value =
                        $crate::Mem::replace(&mut self.storage[back_physical], replacement);
                    $crate::Mem::replace(&mut self.storage[index_physical], back_value)
                };
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Primitive-index variant of [`swap_remove_with`][Self::swap_remove_with].
            pub fn swap_remove_with_prim(&mut self, index: $P, replacement: T)
                -> Result<Option<T>, $crate::InvalidValue> {
                Ok(self.swap_remove_with(
                    $crate::unwrap![ok? Self::_prim_to_idx(index)], replacement))
            }
            /// Removes and returns the value at logical `index`,
            /// replacing the old back slot with `T::default()`.
            ///
            /// Decrements `len`. Does not preserve order.
            pub fn swap_remove_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.swap_remove_with(index, T::default())
            }
            /// Primitive-index variant of [`swap_remove_default`][Self::swap_remove_default].
            pub fn swap_remove_default_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> where T: Default {
                Ok(self.swap_remove_default($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /// Removes and returns a copied value at logical `index`,
            /// filling the gap with the logical back element.
            ///
            /// Decrements `len`. Does not preserve order.
            ///
            /// The vacated physical back slot is left unchanged but outside the
            /// active logical range.
            pub const fn swap_remove_copy(&mut self, index: $I) -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_physical = self._physical_usize(Self::_idx_to_usize(index));
                let back_physical = self._back_usize();
                let value = self.storage[index_physical];
                if index_physical != back_physical {
                    self.storage[index_physical] = self.storage[back_physical];
                }
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Primitive-index variant of [`swap_remove_copy`][Self::swap_remove_copy].
            pub const fn swap_remove_copy_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> where T: Copy {
                Ok(self.swap_remove_copy($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /// Removes and returns a copied value at logical `index`,
            /// filling the gap with the logical back element and replacing the
            /// old back slot with `replacement`.
            ///
            /// Decrements `len`. Does not preserve order.
            pub const fn swap_remove_copy_with(&mut self, index: $I, replacement: T)
                -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_physical = self._physical_usize(Self::_idx_to_usize(index));
                let back_physical = self._back_usize();
                let value = self.storage[index_physical];
                if index_physical != back_physical {
                    self.storage[index_physical] = self.storage[back_physical];
                }
                self.storage[back_physical] = replacement;
                self.len = self._len_dec();
                if self.is_empty() { self.head = Self::_idx_zero(); }
                Some(value)
            }
            /// Primitive-index variant of [`swap_remove_copy_with`][Self::swap_remove_copy_with].
            pub const fn swap_remove_copy_with_prim(&mut self, index: $P, replacement: T)
                -> Result<Option<T>, $crate::InvalidValue> where T: Copy {
                Ok(self.swap_remove_copy_with(
                    $crate::unwrap![ok? Self::_prim_to_idx(index)], replacement))
            }
            /// Removes and returns a copied value at logical `index`,
            /// replacing the old back slot with `T::INIT`.
            ///
            /// Decrements `len`. Does not preserve order.
            pub const fn swap_remove_init(&mut self, index: $I)
                -> Option<T> where T: $crate::ConstInit + Copy {
                self.swap_remove_copy_with(index, T::INIT)
            }
            /// Primitive-index variant of [`swap_remove_init`][Self::swap_remove_init].
            pub const fn swap_remove_init_prim(&mut self, index: $P)
                -> Result<Option<T>, $crate::InvalidValue> where T: $crate::ConstInit + Copy {
                Ok(self.swap_remove_init($crate::unwrap![ok? Self::_prim_to_idx(index)]))
            }

            /* views */

            /// Returns the active logical range as two slices.
            ///
            /// The first slice starts at `head`. The second slice is non-empty
            /// only when the active range wraps around the end of storage.
            pub fn as_slices(&self) -> (&[T], &[T]) {
                let len = self._len_usize();
                if len == 0 { return (&self.storage[..0], &self.storage[..0]); }
                let head = self._head_usize();
                let first_len = if len <= CAP - head { len } else { CAP - head };
                let second_len = len - first_len;
                let (left, right) = self.storage.split_at(head);
                (&right[..first_len], &left[..second_len])
            }
            /// Returns the active logical range as two mutable slices.
            ///
            /// The first slice starts at `head`. The second slice is non-empty
            /// only when the active range wraps around the end of storage.
            pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
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

            /// Iterates over the active elements in logical order.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let (a, b) = self.as_slices();
                a.iter().chain(b.iter())
            }
            /// Iterates mutably over the active elements in logical order.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                let (a, b) = self.as_mut_slices();
                a.iter_mut().chain(b.iter_mut())
            }

            /* visitation */

            /// Visits each active element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                for x in self.iter() { f(x); }
            }
            /// Visits each active element mutably without exposing borrow identity.
            pub fn visit_each_mut<F>(&mut self, f: F) where for<'v> F: Fn(&'v mut T) {
                for x in self.iter_mut() { f(x); }
            }

            /// Visits the active logical range as two shared slices.
            pub fn visit_slices<F, R>(&self, f: F) -> R
            where for<'v> F: FnOnce(&'v [T], &'v [T]) -> R {
                let (a, b) = self.as_slices();
                f(a, b)
            }
            /// Visits the active logical range as two exclusive slices.
            pub fn visit_mut_slices<F, R>(&mut self, f: F) -> R
            where for<'v> F: FnOnce(&'v mut [T], &'v mut [T]) -> R {
                let (a, b) = self.as_mut_slices();
                f(a, b)
            }
        }
    };
}
