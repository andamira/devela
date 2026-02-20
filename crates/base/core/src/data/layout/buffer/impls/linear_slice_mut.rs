// devela_base_core::data::layout::buffer::impls::linear_slice_mut

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_slice_mut {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Buffer view over an exclusive slice.
        ///
        /// # Invariants
        /// - Elements are stored in `storage[0 .. len)`
        /// - `len <= storage.len()`
        ///
        /// # Notes
        /// - This type does not own its elements
        /// - Shrinking the buffer does not drop values, but mutating may overwrite.
        impl<'a, T> $name<'a, T, &'a mut [T]> {
            /* construct */

            /// Creates an empty buffer using the entire slice as backing storage.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_new(slice: &'a mut [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_idx_zero()))
            }
            /// Creates an empty buffer, truncating the backing slice
            /// if it exceeds what the index type can represent.
            pub const fn new_truncated(slice: &'a mut [T]) -> Self {
                let cap = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to_mut(slice, cap);
                Self::_new(slice, Self::_idx_zero())
            }

            /// Creates a buffer over an exclusive slice.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_from_slice(slice: &'a mut [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_usize_to_idx_sat(slice.len())))
            }
            /// Creates a buffer over an exclusive slice with an explicit logical length.
            ///
            /// Returns `None` if `len > slice.len()` or cannot be represented by the index type.
            pub const fn from_slice_with(slice: &'a mut [T], len: $I) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                if $crate::MaybeNiche(len).to_usize_saturating() > slice.len() { return None }
                Some(Self::_new(slice, $crate::MaybeNiche(len)))
            }
            /// Creates a buffer over an exclusive slice, truncating the visible active range
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a mut [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to_mut(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /* capacity */

            $crate::buffer_linear!(%common_view $name, $I, $P);

            /* logical range control */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements.
            pub const fn clear(&mut self) { self.len = Self::_idx_zero(); }

            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub const fn truncate(&mut self, new_len: $I) {
                if Self::_idx_le(new_len, self.len()) { self._set_len(new_len); }
            }
            /// Primitive-index variant of [`truncate`][Self::truncate],
            pub const fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }
            /// Appends a copy of `value` to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = value;
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns a cloned value from the back of the buffer.
            pub fn pop_back_clone(&mut self) -> Option<T> where T: Clone {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()].clone())
            }
            /// Removes and returns a copied value from the back of the buffer.
            pub const fn pop_back_copy(&mut self) -> Option<T> where T: Copy {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                Some(self.storage[self._len_usize()])
            }

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self._len_dec().to_usize_saturating()])
            }
            /// Returns an exclusive reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                Some(&mut self.storage[self._len_dec().to_usize_saturating()])
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }
            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&mut self.storage[Self::_idx_to_usize(index)])
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }
            /// Returns the active logical range as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
}
