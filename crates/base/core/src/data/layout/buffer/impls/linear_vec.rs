// devela_base_core::data::layout::buffer::impls::linear_vec

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_vec {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Dynamically sized array.
        ///
        /// # Semantics
        /// - Logical length is identical to `Vec::len()`.
        /// - Capacity may grow dynamically
        /// - `push_back` never fails
        impl<T> $name<T, ::alloc::vec::Vec<T>> {
            /* construct */

            /// Creates a buffer from an emtpy `Vec`.
            pub const fn new() -> Self { Self::_new(::alloc::vec::Vec::new()) }

            /// Creates a buffer from an emtpy `Vec` with at least the given `capacity`.
            pub fn with_capacity(capacity: $I) -> Self {
                Self::_new(::alloc::vec::Vec::with_capacity(Self::_idx_to_usize(capacity)))
            }
            /// Creates a buffer from an emtpy `Vec` with at least the given `capacity`.
            pub fn with_capacity_prim(capacity: $P) -> Result<Self, $crate::InvalidValue> {
                Ok(Self::with_capacity(Self::_prim_to_idx(capacity)?))
            }

            /* size & capacity */

            /// Returns the number of elements currently stored in the buffer.
            pub const fn len(&self) -> $I {
                Self::_usize_to_idx_sat(self.storage.capacity()).repr()
            }
            /// Returns the number of elements currently stored in the buffer.
            pub const fn len_prim(&self) -> $P { self.storage.len() as $P }
            /// Returns `true` if the buffer contains no elements.
            pub const fn is_empty(&self) -> bool { self.storage.is_empty() }

            /// Returns the capacity of the underlying `Vec`.
            pub const fn capacity(&self) -> $I {
                Self::_usize_to_idx_sat(self.storage.capacity()).repr()
            }
            /// Returns the capacity of the underlying `Vec`.
            pub const fn capacity_prim(&self) -> $P { self.storage.capacity() as $P }
            /// Always returns `false`, since it can never be full.
            pub const fn is_full(&self) -> bool { false }

            /* logical range control */

            /// Sets the logical length to zero.
            ///
            /// Does not drop elements.
            pub fn clear(&mut self) { self.storage.clear() }

            /// Sets the logical length to `min(new_len, len)`.
            ///
            /// If `new_len >= len`, this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                self.storage.truncate(Self::_idx_to_usize(new_len));
            }
            /// Primitive-index variant of [`truncate`][Self::truncate],
            pub fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
            }

            /* push */

            /// Removes and returns a value from the back of the buffer.
            pub fn push_back(&mut self, value: T) { self.storage.push(value); }

            /* pop */

            /// Removes and returns a value from the back of the buffer.
            pub fn pop_back(&mut self) -> Option<T> { self.storage.pop() }

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub fn peek_back(&self) -> Option<&T> { self.storage.last() }
            /// Returns an exclusive reference to the last element without removing it.
            pub fn peek_mut_back(&mut self) -> Option<&mut T> { self.storage.last_mut() }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub fn get(&self, index: $I) -> Option<&T> {
                let idx = Self::_idx_to_usize(index);
                self.storage.get(idx)
            }
            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                let idx = Self::_idx_to_usize(index);
                self.storage.get_mut(idx)
            }

            /* take */

            /// Takes the value at `index`, replacing it with `value`.
            pub fn take_with(&mut self, index: $I, other: T) -> Option<T> {
                let idx = Self::_idx_to_usize(index);
                if idx >= self.storage.len() { return None; }
                Some($crate::Mem::replace(&mut self.storage[idx], other))
            }
            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.take_with(index, T::default())
            }
            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub fn take_init(&mut self, index: $I) -> Option<T> where T: $crate::ConstInitCore {
                self.take_with(index, T::INIT)
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub fn as_slice(&self) -> &[T] { &self.storage }
            /// Returns the active logical range as an exclusive slice.
            pub fn as_mut_slice(&mut self) -> &mut [T] { &mut self.storage }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
}
