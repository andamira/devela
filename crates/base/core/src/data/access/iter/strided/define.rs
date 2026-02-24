// devela_base_core::data::access::iter::strided::define
//
//! Defines [`iter_strided`].
//

#[doc = crate::_tags!(iterator)]
/// Generates a strided iterator type over a slice.
#[doc = crate::_doc_location!("data/access/iter")]
///
/// The generated iterator traverses a slice according to
/// an affine index progression:
///
/// `index_k = start + k * stride`
///
/// for increasing `k`, until the inclusive bound is reached.
///
/// # Variants
///
/// - `: ref ($P)` generates an immutable iterator over `&[T]`.
/// - `: mut ($P)` generates a mutable iterator over `&mut [T]`.
///
/// ## Index Type Parameter
///
/// `$P` must implement [`PrimIndex`][crate::PrimIndex], meaning:
/// - an unsigned primitive integer
/// - no wider than the target's pointer width
///
/// Using smaller primitives (e.g. `u8`, `u16`) can reduce
/// iterator size and encode domain constraints at the type level.
/// The indices are cast to `usize` when indexing the slice.
///
/// ## Invariants
///
/// The generated iterator assumes:
/// - `stride > 0`
/// - `front` and `back` are stride-aligned
/// - Generated indices lie within the slice bounds
///
/// # Panics
/// Panics when iterating if any generated index is out of bounds for the slice.
///
/// ## Examples
///
/// Immutable:
/// ```
/// # use devela_base_core::iter_strided;
/// iter_strided!(pub struct RowIter : ref (usize));
/// let data = [0, 1, 2, 3, 4, 5];
/// let mut it = RowIter::from_count(&data, 1, 3, 2);
/// assert_eq!(it.next(), Some(&1));
/// assert_eq!(it.next(), Some(&3));
/// assert_eq!(it.next(), Some(&5));
/// assert_eq!(it.next(), None);
/// ```
///
/// Mutable:
/// ```
/// # use devela_base_core::iter_strided;
/// iter_strided!(pub struct ChannelIter : mut (u16));
/// let mut data = [0, 1, 2, 3];
/// let mut it = ChannelIter::from_bounds(&mut data, 0, 2, 2);
/// if let Some(x) = it.next() { *x = 42; }
/// ```
///
/// See also the canonical implementations:
/// [`StridedIter`][crate::StridedIter] and
/// [`StridedIterMut`][crate::StridedIterMut].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! iter_strided {
    (
        $(#[$attr:meta])* $vis:vis struct $name:ident : mut ($P:ty)
    ) => {
        $crate::iter_strided!(%mut $(#[$attr])* $vis $name, $P, $crate::NonZero<$P>);
    };
    (
        $(#[$attr:meta])* $vis:vis struct $name:ident : ref ($P:ty)

    /* internals */
    ) => {
        $crate::iter_strided!(%ref $(#[$attr])* $vis $name, $P, $crate::NonZero<$P>);
    };
    // safe-guards
    // --------------------------------------------------------------------------------------------
    // needed: mut || ref
    ($(#[$attr:meta])* $vis:vis struct $name:ident $(:)? ($P:ty)) => {
        compile_error!(concat!(
            "iter_strided! requires either a `mut` or `ref` specifier before the index type.\n",
            "E.g.: `iter_strided![", stringify!($vis), " struct ", stringify!($name),
            " : ref (", stringify!($P), ")]`",));
    };
    // only allow implementions over unsigned integers of size <= pointer-width
    (%guard_allowed_type $P:ty) => {
        const __GUARD_ALLOWED_TYPE: () = {
            const fn __allowed_types<P: $crate::PrimIndex>() {}
            __allowed_types::<$P>();
        };
    };
    // --------------------------------------------------------------------------------------------
    (%mut $(#[$attr:meta])* $vis:vis $name:ident, $P:ty, $NZ:ty) => {
        $(#[$attr])*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T> {
            slice: &'a mut [T],
            front: $P,
            back: $P,
            stride: $NZ,
        }
        impl<'a, T> $name<'a, T> {
            $crate::iter_strided!(%guard_allowed_type $P);
            $crate::iter_strided!(%common_indexing $P, $NZ);
            /* constructors */

            /// Creates a mutable strided iterator
            /// from a starting index, a number of elements, and a stride.
            ///
            /// Generates indices:
            /// `start + k * stride` for `k` in `0..remaining`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if indices are out of bounds.
            pub const fn from_count(
                slice: &'a mut [T],
                start: $P,
                remaining: $P,
                stride: $P,
            ) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_count_nz(slice, start, remaining, v), "stride must be > 0"]
            }
            /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
            /// # Panics
            /// Panics in case of overflow.
            pub const fn from_count_nz(
                slice: &'a mut [T],
                start: $P,
                remaining: $P,
                stride: $NZ,
            ) -> Self {
                if remaining == 0 { return $crate::iter_strided!(%empty slice, stride) }
                let off = stride.get().checked_mul(remaining - 1)
                    .expect("overflow in stride multiplication");
                let back = start.checked_add(off).expect("overflow in back computation");
                Self { slice, front: start, back, stride }
            }

            /// Creates a mutable strided iterator
            /// from inclusive front and back limits and a stride.
            ///
            /// Iteration proceeds from `front` toward `back` (inclusive limit)
            /// in steps of `stride`. The iterator is empty if `front > back`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if indices are out of bounds.
            pub const fn from_bounds(slice: &'a mut [T], front: $P, back: $P, stride: $P) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_parts(slice, front, back, v), "stride must be > 0"]
            }

            /// Creates a mutable strided iterator from its structural components.
            ///
            /// The iterator traverses the `slice` starting at index `front`,
            /// advancing by `stride` on each step, and stopping once `back`
            /// has been yielded. The range is inclusive: both `front` and `back`
            /// are considered valid positions.
            ///
            /// The iterator is empty if `front > back`.
            ///
            /// # Panics
            /// Panics if:
            /// - `front <= back` and `(back - front)` is not a multiple of `stride`.
            /// - `back` is not a valid index in the `slice`.
            pub const fn from_parts(slice: &'a mut [T], front: $P, back: $P, stride: $NZ) -> Self {
                if front <= back {
                    assert!((back - front) % stride.get() == 0, "bounds must be stride-aligned");
                    assert!((back as usize) < slice.len(), "index out of bounds");
                }
                Self { slice, front, back, stride }
            }
            /// Decomposes the iterator into its structural components.
            ///
            /// Returns `(slice, front, back, stride)`.
            ///
            /// The returned values can be passed to [`from_parts`][Self::from_parts]
            /// to reconstruct an equivalent iterator, or used to
            /// transfer iteration state to another compatible type.
            pub const fn into_parts(self) -> (&'a mut [T], $P, $P, $NZ) {
                (self.slice, self.front, self.back, self.stride)
            }

            /* queries */

            /// Returns the number of elements remaining in the iterator.
            pub const fn len(&self) -> $P {
                if self.front > self.back { 0 }
                else { ((self.back - self.front) / self.stride.get()) + 1 }
            }
            /// Returns the number of elements remaining in the iterator as a `usize`.
            /// # Panics
            /// Will only panic if the value can't fit in a `usize`.
            pub const fn len_usize(&self) -> usize { self.len() as usize }

            /* state */

            /// Advances the iterator
            /// and returns an exclusive reference to the next value from the front.
            pub const fn next(&mut self) -> Option<&mut T> {
                Some(&mut self.slice[$crate::unwrap!(some? self._next_front_index())])
            }
            /// Returns an exclusive reference to the next value from the front,
            /// without advancing the iterator.
            pub const fn peek(&mut self) -> Option<&mut T> {
                Some(&mut self.slice[$crate::unwrap!(some? self._peek_next_front_index())])
            }
            /// Advances the iterator
            /// and returns an exclusive reference to the next value from the back.
            pub const fn next_back(&mut self) -> Option<&mut T> {
                Some(&mut self.slice[$crate::unwrap!(some? self._next_back_index())])
            }
            /// Returns an exclusive reference to the next value from the back,
            /// without advancing the iterator.
            pub const fn peek_back(&mut self) -> Option<&mut T> {
                Some(&mut self.slice[$crate::unwrap!(some? self._peek_next_back_index())])
            }
        }

        /* mut: traits */

        impl<'a, T> $crate::IteratorLending for $name<'a, T> {
            type Item<'b> = &'b mut T where Self: 'b;
            fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> { self.next() }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorLendingExactSize for $name<'a, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLendingPeek for $name<'a, T> {
            fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> { self.peek() }
        }
        impl<'a, T> $crate::IteratorLendingDoubleEnded for $name<'a, T> {
            fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Some(&mut self.slice[self._next_back_index()?])
            }
        }
        impl<'a, T> $crate::IteratorLendingPeekDoubleEnded for $name<'a, T> {
            fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> { self.peek_back() }
        }
    };
    (%ref $(#[$attr:meta])* $vis:vis $name:ident, $P:ty, $NZ:ty) => {
        $(#[$attr])*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $vis struct $name<'a, T> {
            slice: &'a [T],
            front: $P,
            back: $P,
            stride: $NZ,
        }
        impl<'a, T> $name<'a, T> {
            $crate::iter_strided!(%guard_allowed_type $P);
            $crate::iter_strided!(%common_indexing $P, $NZ);

            /* constructors */

            /// Creates a strided iterator
            /// from a starting index, a number of elements, and a stride.
            ///
            /// Generates indices:
            /// `start + k * stride` for `k` in `0..remaining`.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if indices are out of bounds.
            pub const fn from_count(
                slice: &'a [T],
                start: $P,
                remaining: $P,
                stride: $P,
            ) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_count_nz(slice, start, remaining, v), "stride must be > 0"]
            }
            /// Like [`from_count`][Self::from_count] but takes a non-zero stride.
            /// # Panics
            /// Panics in case of overflow.
            pub const fn from_count_nz(
                slice: &'a [T],
                start: $P,
                remaining: $P,
                stride: $NZ,
            ) -> Self {
                if remaining == 0 { return $crate::iter_strided!(%empty slice, stride) }
                let off = stride.get().checked_mul(remaining - 1)
                    .expect("overflow in stride multiplication");
                let back = start.checked_add(off).expect("overflow in back computation");
                Self { slice, front: start, back, stride }
            }

            /// Creates a strided iterator
            /// from inclusive front and back limits and a stride.
            ///
            /// Iteration proceeds from `front` toward `back` (inclusive limit)
            /// in steps of `stride`. The iterator is empty if `front > back`.
            ///
            /// This constructor does not validate that generated indices
            /// lie within the `slice`. Violations will cause panics during iteration.
            ///
            /// # Panics
            /// Panics if `stride == 0`.
            /// May panic when advanced if indices are out of bounds.
            pub const fn from_bounds(slice: &'a [T], front: $P, back: $P, stride: $P) -> Self {
                $crate::unwrap![some_map_into_expect <$NZ>::new(stride),
                    |v| Self::from_parts(slice, front, back, v), "stride must be > 0"]
            }

            /// Creates a strided iterator from its structural components.
            ///
            /// The iterator traverses the `slice` starting at index `front`,
            /// advancing by `stride` on each step, and stopping once `back`
            /// has been yielded. The range is inclusive: both `front` and `back`
            /// are considered valid positions.
            ///
            /// The iterator is empty if `front > back`.
            ///
            /// # Panics
            /// Panics if:
            /// - `front <= back` and `(back - front)` is not a multiple of `stride`.
            /// - `back` is not a valid index in the `slice`.
            pub const fn from_parts(
                slice: &'a [T],
                front: $P,
                back: $P,
                stride: $NZ,
            ) -> Self {
                if front <= back {
                    assert!((back - front) % stride.get() == 0, "bounds must be stride-aligned");
                    assert!((back as usize) < slice.len(), "index out of bounds");
                }
                Self { slice, front, back, stride }
            }
            /// Decomposes the iterator into its structural components.
            ///
            /// Returns `(slice, front, back, stride)`.
            ///
            /// The returned values can be passed to [`from_parts`][Self::from_parts]
            /// to reconstruct an equivalent iterator, or used to
            /// transfer iteration state to another compatible type.
            pub const fn into_parts(self) -> (&'a [T], $P, $P, $NZ) {
                (self.slice, self.front, self.back, self.stride)
            }

            /* queries */

            /// Returns the number of elements remaining in the iterator.
            pub const fn len(&self) -> $P {
                if self.front > self.back { 0 }
                else { ((self.back - self.front) / self.stride.get()) + 1 }
            }
            /// Returns the number of elements remaining in the iterator as a `usize`.
            /// # Panics
            /// Will only panic if the value can't fit in a `usize`.
            pub const fn len_usize(&self) -> usize { self.len() as usize }

            /* state */

            /// Advances the iterator and returns a shared reference to the next value.
            pub const fn next(&mut self) -> Option<&T> {
                Some(&self.slice[$crate::unwrap!(some? self._next_front_index())])
            }
            /// Returns a shared reference to the next value, without updating the iterator.
            pub const fn peek(&self) -> Option<&T> {
                Some(&self.slice[$crate::unwrap!(some? self._peek_next_front_index())])
            }
            /// Advances the iterator and returns a shared reference to the next value from the back.
            pub const fn next_back(&mut self) -> Option<&T> {
                Some(&self.slice[$crate::unwrap!(some? self._next_back_index())])
            }
            /// Returns a shared reference to the next value from the back, without updating the iterator.
            pub const fn peek_back(&self) -> Option<&T> {
                Some(&self.slice[$crate::unwrap!(some? self._peek_next_back_index())])
            }
        }

        /* ref: traits*/

        impl<'a, T> $crate::Iterator for $name<'a, T> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                Some(&self.slice[self._next_front_index()?])
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorDoubleEnded for $name<'a, T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                Some(&self.slice[self._next_back_index()?])
            }
        }
        impl<T> $crate::IteratorExactSize for $name<'_, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLending for $name<'a, T> {
            type Item<'b> = &'b T where Self: 'b;
            fn next<'b>(&'b mut self) -> Option<Self::Item<'b>> { self.next() }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len_usize();
                (len, Some(len))
            }
        }
        impl<'a, T> $crate::IteratorLendingExactSize for $name<'a, T> {
            fn len(&self) -> usize { self.len_usize() }
        }
        impl<'a, T> $crate::IteratorLendingPeek for $name<'a, T> {
            fn peek<'b>(&'b mut self) -> Option<Self::Item<'b>> { Self::peek(self) }
        }
        impl<'a, T> $crate::IteratorLendingDoubleEnded for $name<'a, T> {
            fn next_back<'b>(&'b mut self) -> Option<Self::Item<'b>> {
                Some(&self.slice[self._next_back_index()?])
            }
        }
        impl<'a, T> $crate::IteratorLendingPeekDoubleEnded for $name<'a, T> {
            fn peek_back<'b>(&'b mut self) -> Option<Self::Item<'b>> { Self::peek_back(self) }
        }
    };
    (%common_indexing $P:ty, $NZ:ty) => {
        /// Advances the iterator and returns the next index from the front.
        const fn _next_front_index(&mut self) -> Option<usize> {
            // if unlikely(self.front > self.back) { // MAYBE:BENCH
            if self.front > self.back {
                None
            } else {
                let idx = self.front;
                // self.front += self.stride.get();
                // NOTE: for the last element, forces an empty sentinel state
                // if unlikely(self.front == self.back) { // MAYBE:BENCH
                if self.front == self.back {
                    self.front = 1;
                    self.back = 0;
                } else {
                    self.front += self.stride.get();
                }
                Some(idx as usize)
            }
        }
        /// Advances the iterator and returns the next index from the back.
        const fn _next_back_index(&mut self) -> Option<usize> {
            if self.front > self.back {
                None
            } else {
                let idx = self.back;
                // self.back -= self.stride.get();
                // NOTE: for the last element, forces an empty sentinel state
                if self.front == self.back {
                    self.front = 1;
                    self.back = 0;
                } else {
                    self.back -= self.stride.get();
                }
                Some(idx as usize)
            }
        }
        /// Returns the next index from the front without updating the iterator.
        const fn _peek_next_front_index(&self) -> Option<usize> {
            if self.front > self.back { None } else { Some(self.front as usize) }
        }
        /// Returns the next index from the back without updating the iterator.
        const fn _peek_next_back_index(&self) -> Option<usize> {
            if self.front > self.back { None } else { Some(self.back as usize) }
        }
    };
    (%empty $slice:expr, $stride:expr) => {
        Self { slice: $slice, front: 1, back: 0, stride: $stride }
    };
}
#[doc(inline)]
pub use crate::iter_strided;
