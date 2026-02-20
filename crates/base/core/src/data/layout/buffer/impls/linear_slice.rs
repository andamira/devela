// devela_base_core::data::layout::buffer::impls::linear_slice

#[doc(hidden)]
#[macro_export]
macro_rules! __buffer_linear_impl_slice {
    ($(#[$impl_attr:meta])* $name:ident, $I:ty, $P:ty) => {
        $(#[$impl_attr])*
        ///
        /// Read-only buffer view over a shared slice.
        ///
        /// # Invariants
        /// - Elements are read from `storage[0 .. len)`
        /// - `len <= storage.len()`
        ///
        /// # Notes
        /// - This type does not own its elements
        /// - No mutation or removal operations are supported
        /// - `len` limits the visible logical range
        impl<'a, T> $name<'a, T, &'a [T]> {
            /// Creates a buffer over a shared slice.
            ///
            /// Returns `None` if the slice length cannot be represented by the index type.
            pub const fn try_from_slice(slice: &'a [T]) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                Some(Self::_new(slice, Self::_usize_to_idx_sat(slice.len())))
            }

            /// Creates a buffer over an exclusive slice with an explicit logical length.
            ///
            /// Returns `None` if `len > slice.len()` or cannot be represented by the index type.
            pub const fn from_slice_with(slice: &'a [T], len: $I) -> Option<Self> {
                if slice.len() > Self::_IDX_MAX_USIZE { return None }
                if $crate::MaybeNiche(len).to_usize_saturating() > slice.len() { return None }
                Some(Self::_new(slice, $crate::MaybeNiche(len)))
            }

            /// Creates a buffer over a shared slice, truncating the visible logical range
            /// if the slice length exceeds what the index type can represent.
            pub const fn from_slice_truncated(slice: &'a [T]) -> Self {
                let len_usize = $crate::Cmp(slice.len()).min(Self::_IDX_MAX_USIZE);
                let slice = $crate::Slice::range_to(slice, len_usize);
                Self::_new(slice, Self::_usize_to_idx_sat(len_usize))
            }

            /* capacity */

            $crate::buffer_linear!(%common_view $name, $I, $P);

            /* peek */

            /// Returns a shared reference to the last element without removing it.
            pub const fn peek_back(&self) -> Option<&T> {
                if self.is_empty() { return None; }
                Some(&self.storage[self.len.to_usize_saturating() - 1])
            }

            /* get */

            /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
            pub const fn get(&self, index: $I) -> Option<&T> {
                if Self::_idx_ge(index, self.len()) { return None; }
                Some(&self.storage[Self::_idx_to_usize(index)])
            }

            /* views */

            /// Returns the active logical range as a slice.
            pub const fn as_slice(&self) -> &[T] {
                $crate::Slice::range_to(&self.storage, self.len.to_usize_saturating())
            }

            /* iteration */

            /// Iterates over the initialized elements.
            pub fn iter(&self) -> impl Iterator<Item = &T> {
                let len = self._len_usize(); self.storage[..len].iter()
            }

            /* visitation */

            /// Visits each initialized element without exposing borrow identity.
            pub fn visit_each<F>(&self, f: F) where for<'v> F: Fn(&'v T) {
                let len = self._len_usize(); for item in &self.storage[..len] { f(item); }
            }
            /// Visits the active logical range as a shared slice
            /// without exposing borrow identity.
            pub fn visit_slice<F, R>(&self, f: F) -> R where for<'v> F: FnOnce(&'v [T]) -> R {
                let len = self._len_usize(); f(&self.storage[..len])
            }
        }
    };
}
