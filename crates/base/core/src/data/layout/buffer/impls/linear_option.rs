// devela_base_core::data::layout::buffer::impls::linear_option

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_option {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Fully initialized array using `Option<T>` as a drop boundary.
        ///
        /// # Invariants
        /// - Slots `0 .. len` are `Some`
        /// - Slots `len .. CAP` are logically outside the buffer
        /// - No holes are permitted in the active logical range.
        ///
        /// # Notes
        /// - `Option<T>` is used to control initialization and dropping, not sparsity
        /// - `len` is the number of elements
        /// - Methods never access storage past `len`
        impl<T, const CAP: usize> $name<T, [Option<T>; CAP]> {
            /* construct */

            /// Creates a buffer from a fully initialized array with logical length 0.
            pub const fn new() -> Self {
                Self::_new([const { None }; CAP], Self::_idx_zero())
            }

            /// Creates a buffer from a fully initialized array of clonable elements.
            /// # Panic
            /// Panics if `N > CAP`.
            pub fn from_array_clone<const N: usize>(src: [T; N]) -> Self where T: Clone {
                assert!(N <= CAP); // IMPROVE: Option instead of panic
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; { storage[i] = Some(src[i].clone()); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            /// Creates a buffer from a fully initialized array of copiable elements.
            /// # Panic
            /// Panics if `N > CAP`.
            pub const fn from_array_copy<const N: usize>(src: [T; N]) -> Self where T: Copy {
                assert!(N <= CAP); // IMPROVE: Option instead of panic
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..N; { storage[i] = Some(src[i]); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            $crate::_devela_policy! { safe:"safe_data", unsafe:"unsafe_array",
                /// Creates a buffer from an array of options and an explicit logical length,
                /// without validating the linear invariant.
                ///
                /// # Panics
                /// Panics in debug if `len > CAP`.
                ///
                /// # Safety
                /// Caller must guarantee:
                /// - `len <= CAP`
                /// - `storage[0..len]` are `Some`
                /// - `storage[len..CAP]` are `None`
                pub const unsafe fn from_array_unchecked(array: [Option<T>; CAP], len: $I) -> Self {
                    debug_assert!(Self::_idx_ge(len, Self::CAP));
                    Self::_new(array, $crate::MaybeNiche(len))
                }
            }

            /// Creates a buffer from an array of options, validating the linear invariant.
            ///
            /// Returns `None` if:
            /// - a `None` appears before a `Some`
            /// - any element after the active logical range is `Some`
            pub fn from_array_linear(array: [Option<T>; CAP]) -> Option<Self> {
                let mut len = 0;
                $crate::whilst! { i in 0..CAP; {
                    if array[i].is_some() { len += 1; } else { break; }
                }}
                $crate::whilst! { i in len,..CAP; { if array[i].is_some() { return None; } }}
                Some(Self::_new(array, Self::_usize_to_idx(len)))
            }

            /// Creates a buffer from an array of options by taking the prefix of `Some` values.
            ///
            /// The logical length is inferred as the index of the first `None`.
            /// Elements after the first `None` are ignored and need not be `None`.
            ///
            /// Returns `None` if a `None` appears before a `Some` in the prefix.
            pub fn from_array_prefix(array: [Option<T>; CAP]) -> Option<Self> {
                let mut len = 0;
                $crate::whilst! { i in 0..CAP; {
                    if array[i].is_some() { len += 1; } else { break; }
                }}
                Some(Self::_new(array, Self::_usize_to_idx(len)))
            }

            /// Creates a new buffer by cloning all the possible elements from `src`.
            pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = Some(src[i].clone()); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /// Creates a new buffer by copying all the possible elements from `src`.
            pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [const { None }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i] = Some(src[i]); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

            /* logical range control */

            /// Clears the buffer, dropping all elements.
            pub fn clear(&mut self) {
                $crate::whilst! { i in 0..self._len_usize(); { self.storage[i] = None; }}
                self.len = Self::_idx_zero();
            }

            /// Truncates the buffer to `new_len`, dropping excess elements.
            ///
            /// If `new_len >= len` this is a no-op.
            pub fn truncate(&mut self, new_len: $I) {
                if new_len >= self.len() { return; }
                $crate::whilst! { i in Self::_idx_to_usize(new_len), ..self._len_usize(); {
                    self.storage[i] = None;
                }}
                self._set_len(new_len);
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }

            /// Appends a copy of `value` to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back_copy(&mut self, value: T) -> Result<(), T> where T: Copy {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()] = Some(value);
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns a value from the back of the buffer.
            pub const fn pop_back(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                self.storage[self._len_usize()].take()
            }

            /* peek */

            /// Returns a reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                self.storage[self._len_dec().to_usize_saturating()].as_ref()

            }
            /// Returns a reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                self.storage[self._len_dec().to_usize_saturating()].as_mut()
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                self.storage[Self::_idx_to_usize(index)].as_ref()
            }

            /// Returns an exclusive reference to the element at `index`,
            /// or `None` if out of bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                self.storage[Self::_idx_to_usize(index)].as_mut()
            }

            /* swap */

            /// Removes and returns the value at `index`, filling the gap with the last element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub fn swap_remove(&mut self, index: $I) -> Option<T> {
                if index >= self.len() { return None; }
                let last = self._len_dec().repr();
                self._set_len(last);
                let last_usize = Self::_idx_to_usize(last);
                if index == last {
                    self.storage[last_usize].take()
                } else {
                    let index_usize = Self::_idx_to_usize(index);
                    let value = self.storage[index_usize].take();
                    self.storage[index_usize] = self.storage[last_usize].take();
                    value
                }
            }

            /// Removes and returns the value at `index`, filling the gap with the last element.
            ///
            /// Decrements `len`. Does not preserve order.
            pub const fn swap_remove_copy(&mut self, index: $I) -> Option<T> where T: Copy {
                if Self::_idx_ge(index, self.len()) { return None; }
                let last = self._len_dec().repr();
                self._set_len(last);
                let last_usize = Self::_idx_to_usize(last);
                if Self::_idx_eq(index, last) {
                    self.storage[last_usize]
                } else {
                    let index_usize = Self::_idx_to_usize(index);
                    let value = self.storage[index_usize];
                    self.storage[index_usize] = self.storage[last_usize];
                    value
                }
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[Option<T>] {
                $crate::Slice::range_to(&self.storage, self._len_usize())
            }

            /// Returns the active logical range as a mutable slice.
            pub fn as_mut_slice(&mut self) -> &mut [Option<T>] {
                let len_usize = self._len_usize();
                $crate::Slice::range_to_mut(&mut self.storage, len_usize)
            }

            /// Iterates over the initialized elements.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let len = self._len_usize();
                self.storage[..len].iter().map(|x| x.as_ref().unwrap())
            }
            /// Iterates mutably over the initialized elements.
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
                let len = self._len_usize();
                self.storage[..len].iter_mut().map(|x| x.as_mut().unwrap())
            }

            /* visitation */

            /// Visits each initialized element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                let len = self._len_usize();
                self.storage[..len].iter() // SAFETY: prefix elements are all Some(T)
                    .for_each(|x| f($crate::unwrap![some_guaranteed_or_ub x.as_ref()]));

            }
            /// Visits each initialized element mutably without exposing borrow identity.
            pub fn visit_each_mut<F>(&mut self, f: F) where for<'v> F: Fn(&'v mut T) {
                let len = self._len_usize();
                self.storage[..len].iter_mut() // SAFETY: prefix elements are all Some(T)
                    .for_each(|x| f($crate::unwrap![some_guaranteed_or_ub x.as_mut()]));
            }

            /// Visits the active logical range as a shared slice of `Some(T)`,
            /// without exposing borrow identity.
            pub fn visit_slice<F, R>(&self, f: F)
                -> R where for<'v> F: FnOnce(&'v [Option<T>]) -> R {
                let len = self._len_usize(); f(&self.storage[..len])
            }
            /// Visits the active logical range as an exclusive slice of `Some(T)`,
            /// without exposing borrow identity.
            pub fn visit_mut_slice<F, R>(&mut self, f: F)
                -> R where for<'v> F: FnOnce(&'v mut [Option<T>]) -> R {
                let len = self._len_usize(); f(&mut self.storage[..len])
            }
        }
    };
}
