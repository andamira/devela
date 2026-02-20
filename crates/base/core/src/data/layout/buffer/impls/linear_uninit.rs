// devela_base_core::data::layout::buffer::impls::linear_uninit

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_uninit {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
       $(#[$impl_attr])*
        ///
        /// Partially initialized array.
        ///
        /// # Invariants
        /// - Only slots `0 .. len` are initialized
        /// - Slots `len .. CAP` are uninitialized and must never be dropped as `T`
        /// - Drop behavior depends on `len`
        ///
        /// Consequences
        /// - pop_back can safely move out `T`
        /// - Real drop operations are meaningful
        /// - `len` controls both logical membership and initialization
        impl<T, const CAP: usize> $name<T, [$crate::MaybeUninit<T>; CAP]> {
            /* construct */

            /// Creates an empty buffer with uninitialized storage.
            pub const fn new() -> Self {
                Self::_new([const { $crate::MaybeUninit::uninit() }; CAP], Self::_idx_zero())
            }

            /// Creates a buffer by moving all elements from an array.
            ///
            /// Initializes exactly `N` elements.
            ///
            /// # Panics
            /// Panics if `N > CAP`.
            pub fn from_array_exact<const N: usize>(src: [T; N]) -> Self {
                assert!(N <= CAP); // MAYBE return Option
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                let src = $crate::ManuallyDrop::new(src);
                let ptr = src.as_ptr();
                // SAFETY: each element is read exactly once
                $crate::whilst! { i in 0..N; { storage[i].write(unsafe { ptr.add(i).read() }); }}
                Self::_new(storage, Self::_usize_to_idx(N))
            }

            /// Creates a buffer from raw uninitialized storage.
            ///
            /// # Safety
            /// Caller must guarantee:
            /// - `storage[0..len]` are initialized
            /// - `storage[len..]` are uninitialized
            pub const unsafe fn from_array_unchecked(
                storage: [$crate::MaybeUninit<T>; CAP],
                len: $I,
            ) -> Self {
                debug_assert!(Self::_idx_ge(len, Self::CAP));
                Self::_new(storage, $crate::MaybeNiche(len))
            }

            /// Creates a new buffer by cloning all the possible elements from `src`.
            pub fn from_slice_clone(src: &[T]) -> Option<Self> where T: Clone {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i].write(src[i].clone()); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by copying all the possible elements from `src`.
            pub const fn from_slice_copy(src: &[T]) -> Option<Self> where T: Copy {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); { storage[i].write(src[i]); }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the default value.
            pub fn from_slice_move_default(src: &mut [T]) -> Option<Self> where T: Default {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i].write($crate::Mem::take(&mut src[i]));
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }
            /// Creates a new buffer by moving all the possible elements from `src`,
            /// and replacing them with the initializing value.
            pub const fn from_slice_move_init(src: &mut [T]) -> Option<Self>
            where T: $crate::ConstInitCore {
                if src.len() > CAP { return None; }
                let mut storage = [const { $crate::MaybeUninit::uninit() }; CAP];
                $crate::whilst! { i in 0..src.len(); {
                    storage[i].write($crate::Mem::replace(&mut src[i], T::INIT));
                }}
                Some(Self::_new(storage, Self::_usize_to_idx(src.len())))
            }

            /* size & capacity */

            $crate::buffer_linear!(%common_static $name, $I, $P);

            /* logical range control */

            /// Drops all initialized elements and resets the buffer.
            pub fn clear(&mut self) {
                while !self.is_empty() {
                    self.len = self._len_dec();
                    // SAFETY: by invariant, slots 0..old_len are initialized. We decrement len
                    // first, so `self.len` now indexes the last previously-initialized slot.
                    unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                }
            }

            /// Drops the last element without returning it.
            pub fn drop_back(&mut self) -> bool {
                if self.is_empty() { return false; }
                self.len = self._len_dec();
                // SAFETY: The index we drop is < old_len so it is initialized.
                unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                true
            }

            /// Sets the length to `min(new_len, len)`, dropping the truncated elements.
            ///
            /// If `new_len >= len`, this is a no-op.
            // WAIT: [const_drop_in_place](https://github.com/rust-lang/rust/issues/109342)
            pub fn truncate(&mut self, new_len: $I) {
                while self.len() > new_len {
                    self.len = self._len_dec();
                    // SAFETY: see above
                    unsafe { self.storage[self._len_usize()].assume_init_drop(); }
                }
            }
            /// Primitive-index variant of [`truncate`][Self::truncate],
            #[inline(always)]
            pub fn truncate_prim(&mut self, new_len: $P) -> Result<(), $crate::InvalidValue> {
                self.truncate($crate::unwrap![ok? Self::_prim_to_idx(new_len)]);
                Ok(())
            }

            /* push */

            /// Appends a value to the back of the buffer.
            ///
            /// Returns `Err(value)` if the buffer is full.
            pub const fn push_back(&mut self, value: T) -> Result<(), T> {
                if self.is_full() { return Err(value); }
                self.storage[self._len_usize()].write(value);
                self.len = self._len_inc();
                Ok(())
            }

            /* pop */

            /// Removes and returns the last element.
            ///
            /// This moves the value out without requiring `T: Copy` or `T: Clone`.
            pub const fn pop_back(&mut self) -> Option<T> {
                if self.is_empty() { return None; }
                self.len = self._len_dec();
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { self.storage[self._len_usize()].assume_init_read() })
            }

            /* peek */

            /// Returns a reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &*self.storage[self._len_dec().to_usize_saturating()].as_ptr() })
            }
            /// Returns a reference to the last element without removing it.
            pub const fn peek_mut_back(&mut self) -> Option<&mut T> {
                if self.is_empty() { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe {
                    &mut *self.storage[self._len_dec().to_usize_saturating()].as_mut_ptr()
                })
            }

            /* get */

            /// Returns a reference to the element at `index`, if within bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &*self.storage[Self::_idx_to_usize(index)].as_ptr() })
            }

            /// Returns an exclusive reference to the element at `index`, if within bounds.
            pub const fn get_mut(&mut self, index: $I) -> Option<&mut T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                Some(unsafe { &mut *self.storage[Self::_idx_to_usize(index)].as_mut_ptr() })
            }

            /* take */

            /// Takes the value at `index`, replacing it with `T::default()`.
            pub fn take_default(&mut self, index: $I) -> Option<T> where T: Default {
                self.take_with(index, T::default())
            }
            /// Takes the value at `index`, replacing it with `T::INIT`.
            pub const fn take_init(&mut self, index: $I) -> Option<T>
            where T: $crate::ConstInitCore {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                let old = unsafe { self.storage[index_usize].assume_init_read() };
                self.storage[index_usize].write(T::INIT);
                Some(old)
            }
            /// Takes the value at `index`, replacing it with `value`.
            pub fn take_with(&mut self, index: $I, value: T) -> Option<T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                let index_usize = Self::_idx_to_usize(index);
                // SAFETY: `index < self.len`, so the slot is initialized per invariant.
                let old = unsafe { self.storage[index_usize].assume_init_read() };
                self.storage[index_usize].write(value);
                Some(old)
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts(
                    self.storage.as_ptr() as *const T,
                    self._len_usize()
                )}
            }
            /// Returns the active logical range as a mutable slice.
            pub const fn as_mut_slice(&mut self) -> &mut [T] {
                // SAFETY: `0..self.len` is initialized per invariant.
                unsafe { ::core::slice::from_raw_parts_mut(
                    self.storage.as_mut_ptr() as *mut T, self._len_usize()
                ) }
            }

            $crate::buffer_linear!(%common_iter_visit $name, $I, $P);
        }
    };
}
