// devela::data::layout::buffer::impls::linear_array

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_array {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized array.
        ///
        /// # Invariants
        /// - All CAP slots always contain a valid T
        /// - len controls logical membership, not initialization
        /// - Dropping the array drops all CAP elements
        ///
        /// Consequences
        /// - Cannot move out `T` safely
        /// - Pop must be Copy or Clone
        /// - Shrinking len does not affect drop behavior
        #[rustfmt::skip]
        impl<T, const CAP: usize> $name<T, [T; CAP]> {
            /* construct */

            /// Creates a buffer from an already initialized array, with logical length 0.
            pub const fn new(array: [T; CAP]) -> Self {
                Self::_new(array, Self::_idx_zero())
            }
            /// Creates a new fully initialized buffer with logical length 0.
            pub const fn new_init() -> Self where T: $crate::ConstInit {
                Self::_new([T::INIT; CAP], Self::_idx_zero())
            }

            /// Creates a buffer from an already initialized array,
            /// limiting the logical length to `max_len`.
            pub fn from_array_clamped(array: [T; CAP], max_len: $I) -> Self {
                let a = $crate::MaybeNiche(max_len).prim();
                let b = $crate::MaybeNiche(Self::CAP).prim();
                let min = $crate::Cmp(a).min(b);
                // SAFETY: both are already validated
                let len = $crate::unwrap![ok_guaranteed_or_ub
                    $crate::MaybeNiche::<$I>::try_from_prim(min)];
                Self::_new(array, len)
            }
            /// Primitive-index variant of [`from_array_clamped`][Self::from_array_clamped].
            #[inline(always)]
            pub fn from_array_clamped_prim(array: [T; CAP], max_len: $P) -> Self {
                Self::from_array_clamped(array, Self::_prim_to_idx_lossy(max_len))
            }

            /// Creates a new buffer by cloning all the possible elements from `src`,
            /// after initializing the capacity with the `init` value.
            pub fn from_slice_clone(src: &[T], init: T) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = $crate::array_from_fn(|_| init.clone());
                $crate::whilst! { i in 0..src.len(); { storage[i] = src[i].clone(); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by copying all the possible elements from `src`,
            /// after initializing the capacity with the `init` value.
            pub const fn from_slice_copy(src: &[T], init: T) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [init; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = src[i]; }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the default value,
            /// after initializing the capacity with the default value.
            pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
                if src.len() > CAP { return None; }
                let mut storage = $crate::array_from_fn(|_| T::default());
                $crate::whilst! { i in 0..src.len(); {
                    storage[i] = $crate::Mem::take(&mut src[i]);
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

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
            #[inline(always)]
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

            /// Appends as many cloned elements from `src` as fit.
            ///
            /// Returns the number of elements appended.
            pub fn push_slice(&mut self, src: &[T]) -> usize where T: Clone {
                let len = self._len_usize();
                let count = usize::min(src.len(), CAP - len);
                self.storage[len..len + count].clone_from_slice(&src[..count]);
                self.len = Self::_usize_to_idx(len + count);
                count
            }
            /// Appends as many copied elements from `src` as fit.
            ///
            /// Returns the number of elements appended.
            pub const fn push_slice_copy(&mut self, src: &[T]) -> usize where T: Copy {
                let len = self._len_usize();
                let count = $crate::cmp!(min src.len(), CAP - len);
                let dst_slice = $crate::Slice::range_mut(&mut self.storage, len, len + count);
                let src_slice = $crate::Slice::range_to(&src, count);
                dst_slice.copy_from_slice(src_slice);
                self.len = Self::_usize_to_idx(len + count);
                count
            }
            /// Appends all copied elements from `src`,
            /// or returns the number appended before running out of space.
            pub const fn push_slice_copy_exact(&mut self, src: &[T]) -> Result<(), usize>
            where T: Copy {
                let rem = CAP - self._len_usize();
                if src.len() > rem { return Err(rem); }
                let _ = self.push_slice_copy(src);
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

            /* take */

            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T>
            where T: Default {
                if index >= self.len() { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], T::default()))
            }
            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub const fn take_init(&mut self, index: $I) -> Option<T>
            where T: $crate::ConstInit {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], T::INIT))
            }
            /// Takes the value at `index`, replacing it with `other`.
            pub fn take_with(&mut self, index: $I, other: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], other))
            }
            /// Takes the value at `index`, replacing it with a copy of `other`.
            pub const fn take_with_copy(&mut self, index: $I, other: T) -> Option<T>
            where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                Some($crate::Mem::replace(&mut self.storage[index_usize], other))
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                let len = self._len_usize(); $crate::Slice::range_to(&self.storage, len)
            }
            /// Returns the active logical range as an exclusive slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] {
                let len = self._len_usize(); $crate::Slice::range_to_mut(&mut self.storage, len)
            }

            /* iteration & visitation */

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
}
